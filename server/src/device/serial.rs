use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Context, Result};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    sync::mpsc,
    task,
};
use tokio_serial::{SerialPortBuilderExt, SerialStream};

use crate::app_state::{AppState, GpsFix, Target};
use crate::device::protocol::{Command, Event};

pub type CommandTx = mpsc::Sender<Command>;

/// Spawns the background serial task.
/// Runs forever and updates AppState directly.
pub fn spawn_serial_link(port_path: String, baud: u32, state: Arc<Mutex<AppState>>) -> CommandTx {
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<Command>(32);

    task::spawn(async move {
        loop {
            match open_port(&port_path, baud).await {
                Ok(port) => {
                    if let Err(e) = run_session(port, state.clone(), &mut cmd_rx).await {
                        eprintln!("Serial session error: {e:?}");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to open serial port: {e:?}");
                }
            }

            // Backoff before reconnect
            tokio::time::sleep(Duration::from_millis(800)).await;
        }
    });

    cmd_tx
}

async fn open_port(path: &str, baud: u32) -> Result<SerialStream> {
    tokio_serial::new(path, baud)
        .open_native_async()
        .with_context(|| format!("Failed to open serial port {path} @ {baud}"))
}

async fn run_session(
    port: SerialStream,
    state: Arc<Mutex<AppState>>,
    cmd_rx: &mut mpsc::Receiver<Command>,
) -> Result<()> {
    let (read_half, mut write_half) = tokio::io::split(port);
    let mut reader = BufReader::new(read_half).lines();

    loop {
        tokio::select! {
            // Incoming data from ESP32
            line = reader.next_line() => {
                let Some(line) = line? else { break };
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                if let Ok(event) = serde_json::from_str::<Event>(line) {
                    println!("ESP32: {:?}", event);
                    let mut state = state.lock().unwrap();

                    match event {
                        Event::Gps { lat, lon, fix } => {
                            state.latitude = Some(lat);
                            state.longitude = Some(lon);
                            state.gps_fix = Some(match fix {
                                3 => GpsFix::Fix3D,
                                2 => GpsFix::Fix2D,
                                _ => GpsFix::NoFix,
                            });
                        }

                        Event::Status { tracking } => {
                            state.mount_status =
                                if tracking { crate::app_state::MountStatus::Tracking }
                                else { crate::app_state::MountStatus::Idle };
                        }

                        Event::Slew {
                            current_ra,
                            current_dec,
                            target_ra,
                            target_dec,
                        } => {
                            state.current_pos = Some(Target {
                                ra: current_ra,
                                dec: current_dec,
                            });

                            state.target_pos = Some(Target {
                                ra: target_ra,
                                dec: target_dec,
                            });
                        }

                        Event::Error { message } => {
                            state.last_error = Some(message);
                        }

                        Event::Pong => {}

                        Event::Ok => {}
                    }
                }
            }

            // Outgoing commands to ESP32
            Some(cmd) = cmd_rx.recv() => {
                let json = serde_json::to_string(&cmd)?;

                write_half.write_all(json.as_bytes()).await?;
                write_half.write_all(b"\n").await?;
                write_half.flush().await?;
            }
        }
    }

    Ok(())
}

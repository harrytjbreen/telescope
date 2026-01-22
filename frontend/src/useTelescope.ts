import { useEffect, useState, useCallback, useRef } from 'react';

// Matches AppStateDto in Rust
export interface AppState {
  mount_status: string;
  gps_time?: string;
  gps_fix?: string;
  latitude?: number;
  longitude?: number;
  current_ra?: number;
  current_dec?: number;
  target_ra?: number;
  target_dec?: number;
  last_error?: string;
}

// Matches ServerMessage enum in Rust
type ServerMessage = 
  | { type: 'State'; data: AppState }
  | { type: 'Error'; message: string };

// Matches ClientMessage enum in Rust
type ClientMessage = 
  | { type: 'Goto'; ra: number; dec: number }
  | { type: 'StartTracking' }
  | { type: 'StopTracking' };

export function useTelescope(url: string = 'ws://localhost:8080/ws') {
  const [state, setState] = useState<AppState | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [connected, setConnected] = useState<boolean>(false);
  const ws = useRef<WebSocket | null>(null);

  const connect = useCallback(() => {
    // Prevent multiple simultaneous connections
    if (ws.current?.readyState === WebSocket.CONNECTING) return;
    
    const socket = new WebSocket(url);
    ws.current = socket;

    socket.onopen = () => {
      setConnected(true);
      setError(null);
      console.log('WebSocket connected');
    };

    socket.onmessage = (event) => {
      try {
        const msg: ServerMessage = JSON.parse(event.data);
        if (msg.type === 'State') {
          setState(msg.data);
        } else if (msg.type === 'Error') {
          setError(`Server Error: ${msg.message}`);
        }
      } catch (e) {
        console.error('Failed to parse message', e);
      }
    };

    socket.onerror = (event) => {
      setError('Could not connect to the telescope backend. Ensure the server is running.');
      setConnected(false);
    };

    socket.onclose = () => {
      setConnected(false);
      console.log('WebSocket closed');
    };
  }, [url]);

  useEffect(() => {
    connect();
    return () => ws.current?.close();
  }, [connect]);

  const sendMessage = useCallback((msg: ClientMessage) => {
    if (ws.current?.readyState === WebSocket.OPEN) {
      ws.current.send(JSON.stringify(msg));
    }
  }, []);

  const goto = (ra: number, dec: number) => sendMessage({ type: 'Goto', ra, dec });
  const startTracking = () => sendMessage({ type: 'StartTracking' });
  const stopTracking = () => sendMessage({ type: 'StopTracking' });

  return { state, error, connected, goto, startTracking, stopTracking, reconnect: connect };
}
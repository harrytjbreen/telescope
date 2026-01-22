use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::{fs, path::PathBuf};

pub async fn init_db() -> anyhow::Result<SqlitePool> {
    let db_path = get_db_path()?;

    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true),
    )
    .await?;

    let migrator = sqlx::migrate!();
    println!("Found {} migrations", migrator.migrations.len());

    migrator.run(&pool).await?;

    Ok(pool)
}

fn get_db_path() -> anyhow::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let dir = exe
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Could not resolve exe dir"))?;

    let data_dir = dir.join("data");
    fs::create_dir_all(&data_dir)?;

    Ok(data_dir.join("telescope.db"))
}


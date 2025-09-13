use std::{env, fs, time::Duration};

use anyhow::anyhow;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

pub async fn import(cancellation_token: CancellationToken, input_path: String, output_path: String, no_traverse: bool) -> anyhow::Result<()> {
    log::info!("Importing media files...");

    let input_path = fs::canonicalize(input_path)?;
    let output_path = match fs::canonicalize(&output_path) {
        Ok(path) => path,
        Err(_) => {
            // If folder doesn't exist
            env::current_dir()?.join(&output_path)
        }
    };

    log::debug!("Input path: {}, output path: {}, no traverse: {no_traverse}", input_path.display(), output_path.display());

    for _ in 0..10 {
        if cancellation_token.is_cancelled() {
            return Err(anyhow!("Import cancelled"));
        }

        sleep(Duration::from_secs(1)).await;
        log::debug!(".");
    }

    Ok(())
}
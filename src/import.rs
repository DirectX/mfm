use std::{env, fs, path::PathBuf, pin::Pin};

use anyhow::anyhow;
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

    let _ = scan_dir(&cancellation_token, input_path).await;

    Ok(())
}

pub fn scan_dir(cancellation_token: &CancellationToken, root_path: PathBuf) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    let token = cancellation_token.clone();
    Box::pin(async move {
        let entries = fs::read_dir(root_path)?;
        
        for entry_result in entries {
            if token.is_cancelled() {
                return Err(anyhow!("Cancelled"));
            }

            let entry = entry_result?; // Handle the Result
            let path = entry.path();
            log::debug!("{}", path.display());

            if path.is_dir() {
                scan_dir(&token, path).await?;
            }
        }

        Ok(())
    })
}
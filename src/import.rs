use std::{env, fs, path::{Component, PathBuf}, pin::Pin};

use anyhow::anyhow;
use chrono::{DateTime, Local};
use tokio_util::sync::CancellationToken;

use crate::exif::get_comprehensive_exif_info;

pub async fn import(
    cancellation_token: CancellationToken,
    input_path: String,
    output_path: String,
    no_traverse: bool,
) -> anyhow::Result<()> {
    log::info!("Importing media files...");

    let input_path = fs::canonicalize(input_path)?;
    let output_path = match fs::canonicalize(&output_path) {
        Ok(path) => path,
        Err(_) => {
            // If folder doesn't exist
            env::current_dir()?.join(&output_path)
        }
    };

    log::debug!(
        "Input path: {}, output path: {}, no traverse: {no_traverse}",
        input_path.display(),
        output_path.display()
    );

    let _ = scan_dir(&cancellation_token, input_path).await;

    Ok(())
}

pub fn scan_dir(
    cancellation_token: &CancellationToken,
    root_path: PathBuf,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    let token = cancellation_token.clone();
    Box::pin(async move {
        let entries = fs::read_dir(root_path)?;

        for entry_result in entries {
            if token.is_cancelled() {
                return Err(anyhow!("Graceful shutdown"));
            }

            let entry = entry_result?;
            let path = entry.path();
            log::debug!("{}", path.display());

            if path.is_dir() {
                scan_dir(&token, path).await?;
            } else {
                if let Some(ext) = path.extension() {
                    println!("Extension: {}", ext.to_string_lossy());

                    let upper_folders = get_upper_folders(&path, 3);
                    log::debug!("{:?}", upper_folders);
    
                    let dst_file_name = get_file_name(&path)?;
                    log::debug!("{} -> {}", path.display(), dst_file_name);
                }
            }
        }

        Ok(())
    })
}

pub fn get_file_name(src_path: &PathBuf) -> anyhow::Result<String> {
    match get_comprehensive_exif_info(src_path.to_str().unwrap_or(".")) {
        Ok(exif_date_info) => {
            if let Some(local_creation_time) = exif_date_info.date_time_original {
                log::debug!("EXIF: {}", local_creation_time.format("%Y%m%d_%H%M%S"));
                Ok(format!("123"))
            } else {
                let metadata = fs::metadata(src_path)?;
                let modified = metadata.modified()?;
                let local_modified_time: DateTime<Local> = modified.into();
                log::debug!("FS: {}", local_modified_time.format("%Y%m%d_%H%M%S"));
                Ok(format!("456"))
            }
        }
        Err(err) => {
            log::warn!("{}", err);
            let metadata = fs::metadata(src_path)?;
            let modified = metadata.modified()?;
            let local_modified_time: DateTime<Local> = modified.into();
            log::debug!("FS: {}", local_modified_time.format("%Y%m%d_%H%M%S"));
            Ok(format!("789"))
        }
    }
}

pub fn get_upper_folders(path: &PathBuf, n: usize) -> Vec<String> {
    let components: Vec<_> = path.components()
        .filter_map(|comp| {
            if let Component::Normal(os_str) = comp {
                os_str.to_str().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect();
    
    // Get the last n directories (excluding the filename)
    let dir_count = if path.is_file() { 
        components.len().saturating_sub(1) // Exclude filename
    } else { 
        components.len() 
    };
    
    if dir_count >= n {
        components[dir_count - n..dir_count].to_vec()
    } else {
        components[..dir_count].to_vec()
    }
}
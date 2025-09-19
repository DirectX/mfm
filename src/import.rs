use std::{env, fs, path::{Component, PathBuf}, pin::Pin};

use anyhow::anyhow;
use chrono::{DateTime, Local};
use tokio_util::sync::CancellationToken;

use crate::{exif::get_comprehensive_exif_info, utils::{get_mediatype, normalize_extension}};

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
            let output_path = env::current_dir()?.join(&output_path);
            fs::create_dir_all(&output_path)?;
            println!("Created directories!");
            PathBuf::from(output_path)
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

            if path.is_dir() {
                scan_dir(&token, path).await?;
            } else {
                if let Some(ext) = path.extension() {
                    let extension = normalize_extension(ext);
                    log::debug!("Extension: {}", extension);

                    let mediatype = get_mediatype(extension);
                    log::debug!("Mediatype: {}", mediatype);

                    let (upper_folders, filename) = extract_path_components(&path, 3);
                    log::debug!("{:?}, {}", upper_folders, filename);
    
                    let dst_filename = get_filename(&path)?;
                    log::debug!("{} -> {}", path.display(), dst_filename);
                }
            }
        }

        Ok(())
    })
}

pub fn get_filename(src_path: &PathBuf) -> anyhow::Result<String> {
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
            log::debug!("EXIF error: {}, falling back to file stat", err);
            let metadata = fs::metadata(src_path)?;
            let modified = metadata.modified()?;
            let local_modified_time: DateTime<Local> = modified.into();
            log::debug!("FS: {}", local_modified_time.format("%Y%m%d_%H%M%S"));
            Ok(format!("789"))
        }
    }
}

/// Extracts the upper-level directory names from a file path.
///
/// # Arguments
///
/// * `path` - A reference to a `PathBuf` containing the file path to analyze.
///            Should be an absolute or relative path to a file or directory.
/// * `levels` - The number of directory levels to extract from the end of the path.
///              For example, `2` would extract the last 2 directory names before the filename.
///
/// # Returns
///
/// A tuple containing:
/// * `Vec<String>` - The extracted directory names in order from parent to child
/// * `String` - The filename (including extension) or empty string if no filename
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// 
/// let path = PathBuf::from("/a/b/c/file.jpg");
/// let (dirs, filename) = extract_path_components(&path, 2);
/// assert_eq!(dirs, vec!["b", "c"]);
/// assert_eq!(filename, "file.jpg");
/// ```
fn extract_path_components(path: &PathBuf, levels: usize) -> (Vec<String>, String) {
     let filename_string = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

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
    
    if dir_count >= levels {
        (components[dir_count - levels..dir_count].to_vec(), filename_string.to_owned())
    } else {
        (components[..dir_count].to_vec(), filename_string.to_owned())
    }
}
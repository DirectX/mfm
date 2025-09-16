use std::{collections::HashSet, ffi::OsStr, fmt};

pub enum MediaType {
    Image,
    Video,
    Unknown,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaType::Image => write!(f, "Image"),
            MediaType::Video => write!(f, "Video"),
            MediaType::Unknown => write!(f, "Unknown"),
        }
    }
}

pub fn normalize_extension(ext: &OsStr) -> String {
    let extension = ext.to_string_lossy().to_lowercase();

    match extension.as_str() {
        "jpeg" => "jpg".to_string(),
        "tiff" => "tif".to_string(),
        _ => extension,
    }
}

pub fn get_mediatype(extension: String) -> MediaType {
    static IMAGE_EXTENSIONS: &[&str] = &[
        "jpg", "png", "gif", "bmp", "tif", "webp",
    ];

    // TODO: create HashSet once (in real code, make this static or lazy_static)
    let image_extensions: HashSet<&str> = IMAGE_EXTENSIONS.iter().copied().collect();

    static VIDEO_EXTENSIONS: &[&str] = &[
        "mp4", "mov", "mkv",
    ];

    // TODO: create HashSet once (in real code, make this static or lazy_static)
    let video_extensions: HashSet<&str> = VIDEO_EXTENSIONS.iter().copied().collect();

    if image_extensions.contains(&extension.as_str()) {
        MediaType::Image
    } else {
        if video_extensions.contains(&extension.as_str()) {
            MediaType::Video
        } else {
            MediaType::Unknown
        }
    }
}
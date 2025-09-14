use std::{fs::File, io::BufReader};

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use exif::{In, Reader, Tag};

#[derive(Debug)]
pub struct ExifDateInfo {
    pub date_time_original: Option<DateTime<Local>>,
    pub date_time: Option<DateTime<Local>>,
    pub date_time_digitized: Option<DateTime<Local>>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
}

pub fn parse_exif_date(date_str: &str) -> Option<DateTime<Local>> {
    let formats = [
        "%Y:%m:%d %H:%M:%S",
        "%Y-%m-%d %H:%M:%S",
        "%Y:%m:%d %H:%M:%S%.f",
    ];
    
    for format in &formats {
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(date_str, format) {
            // Convert naive datetime to local timezone
            return Local.from_local_datetime(&naive_dt).single();
        }
    }
    None
}

pub fn get_comprehensive_exif_info(file_path: &str) -> Result<ExifDateInfo, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut bufreader = BufReader::new(&file);
    let exifreader = Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;

    let mut info = ExifDateInfo {
        date_time_original: None,
        date_time: None,
        date_time_digitized: None,
        camera_make: None,
        camera_model: None,
    };

    // Get date fields
    if let Some(field) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
        info.date_time_original = parse_exif_date(&field.display_value().to_string());
    }
    
    if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
        info.date_time = parse_exif_date(&field.display_value().to_string());
    }
    
    if let Some(field) = exif.get_field(Tag::DateTimeDigitized, In::PRIMARY) {
        info.date_time_digitized = parse_exif_date(&field.display_value().to_string());
    }

    // Get camera info
    if let Some(field) = exif.get_field(Tag::Make, In::PRIMARY) {
        info.camera_make = Some(field.display_value().to_string());
    }
    
    if let Some(field) = exif.get_field(Tag::Model, In::PRIMARY) {
        info.camera_model = Some(field.display_value().to_string());
    }

    Ok(info)
}
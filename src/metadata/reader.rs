/// EXIF/Metadata reading implementation

use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::Path;
use crate::errors::ImgrsError;
use super::types::ImageMetadata;

/// Read EXIF from file path
pub fn read_exif_from_path(path: impl AsRef<Path>) -> Result<ImageMetadata, ImgrsError> {
    let file = File::open(path.as_ref())
        .map_err(|e| ImgrsError::InvalidOperation(format!("Failed to open file: {}", e)))?;
    
    let mut bufreader = BufReader::new(file);
    read_exif(&mut bufreader)
}

/// Read EXIF from bytes
    #[allow(dead_code)]
pub fn read_exif_from_bytes(data: &[u8]) -> Result<ImageMetadata, ImgrsError> {
    let mut cursor = Cursor::new(data);
    read_exif(&mut cursor)
}

/// Read EXIF from any reader
pub fn read_exif<R: std::io::BufRead + std::io::Seek>(
    reader: &mut R,
) -> Result<ImageMetadata, ImgrsError> {
    use exif::Reader;
    
    let exifreader = Reader::new();
    
    match exifreader.read_from_container(reader) {
        Ok(exif) => {
            let metadata = extract_metadata(&exif);
            Ok(metadata)
        }
        Err(_) => {
            // Return empty metadata if no EXIF found
            Ok(ImageMetadata::default())
        }
    }
}

/// Extract metadata (public function)
pub fn extract_metadata(exif: &exif::Exif) -> ImageMetadata {
    let mut metadata = ImageMetadata::default();
    
    // Extract basic EXIF data
    let mut exif_data = super::types::ExifData::default();
    
    use exif::{Tag, In};
    
    // Camera make and model
    if let Some(field) = exif.get_field(Tag::Make, In::PRIMARY) {
        exif_data.make = Some(field.display_value().to_string());
    }
    
    if let Some(field) = exif.get_field(Tag::Model, In::PRIMARY) {
        exif_data.model = Some(field.display_value().to_string());
    }
    
    // Date/Time
    if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
        exif_data.date_time = Some(field.display_value().to_string());
    }
    
    if let Some(field) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
        exif_data.date_time_original = Some(field.display_value().to_string());
    }
    
    // Artist and Copyright
    if let Some(field) = exif.get_field(Tag::Artist, In::PRIMARY) {
        exif_data.artist = Some(field.display_value().to_string());
    }
    
    if let Some(field) = exif.get_field(Tag::Copyright, In::PRIMARY) {
        exif_data.copyright = Some(field.display_value().to_string());
    }
    
    metadata.exif = Some(exif_data);
    metadata.gps = extract_gps_info(exif);
    metadata.camera = extract_camera_info(exif);
    
    metadata
}

/// Extract GPS information from EXIF
fn extract_gps_info(exif: &exif::Exif) -> Option<super::types::GpsInfo> {
    use exif::{Tag, In};
    let mut gps = super::types::GpsInfo::default();
    let mut has_gps = false;
    
    // GPS Latitude
    if let Some(field) = exif.get_field(Tag::GPSLatitude, In::PRIMARY) {
        if let Some(lat) = parse_gps_coordinate(&field.value) {
            gps.latitude = Some(lat);
            has_gps = true;
        }
    }
    
    // GPS Longitude
    if let Some(field) = exif.get_field(Tag::GPSLongitude, In::PRIMARY) {
        if let Some(lon) = parse_gps_coordinate(&field.value) {
            gps.longitude = Some(lon);
            has_gps = true;
        }
    }
    
    if has_gps {
        Some(gps)
    } else {
        None
    }
}

/// Extract camera information from EXIF
fn extract_camera_info(exif: &exif::Exif) -> Option<super::types::CameraInfo> {
    use exif::{Tag, In};
    let mut camera = super::types::CameraInfo::default();
    let mut has_camera_info = false;
    
    // ISO Speed
    if let Some(field) = exif.get_field(Tag::PhotographicSensitivity, In::PRIMARY) {
        if let exif::Value::Short(ref v) = field.value {
            if !v.is_empty() {
                camera.iso = Some(v[0] as u32);
                has_camera_info = true;
            }
        }
    }
    
    // Exposure Time
    if let Some(field) = exif.get_field(Tag::ExposureTime, In::PRIMARY) {
        camera.exposure_time = Some(field.display_value().to_string());
        has_camera_info = true;
    }
    
    // F-Number
    if let Some(field) = exif.get_field(Tag::FNumber, In::PRIMARY) {
        if let exif::Value::Rational(ref v) = field.value {
            if !v.is_empty() {
                camera.f_number = Some(v[0].to_f64());
                has_camera_info = true;
            }
        }
    }
    
    // Focal Length
    if let Some(field) = exif.get_field(Tag::FocalLength, In::PRIMARY) {
        if let exif::Value::Rational(ref v) = field.value {
            if !v.is_empty() {
                camera.focal_length = Some(v[0].to_f64());
                has_camera_info = true;
            }
        }
    }
    
    if has_camera_info {
        Some(camera)
    } else {
        None
    }
}

/// Parse GPS coordinate
fn parse_gps_coordinate(value: &exif::Value) -> Option<f64> {
    if let exif::Value::Rational(ref v) = value {
        if v.len() >= 3 {
            let degrees = v[0].to_f64();
            let minutes = v[1].to_f64();
            let seconds = v[2].to_f64();
            return Some(degrees + minutes / 60.0 + seconds / 3600.0);
        }
    }
    None
}

/// Get specific EXIF field value
    #[allow(dead_code)]
pub fn get_exif_field(path: impl AsRef<Path>, tag_name: &str) -> Result<Option<String>, ImgrsError> {
    let metadata = read_exif_from_path(path)?;
    
    if let Some(exif) = metadata.exif {
        // Check common fields
        match tag_name.to_lowercase().as_str() {
            "make" => Ok(exif.make),
            "model" => Ok(exif.model),
            "datetime" => Ok(exif.date_time),
            "software" => Ok(exif.software),
            "artist" => Ok(exif.artist),
            "copyright" => Ok(exif.copyright),
            _ => Ok(exif.raw_data.get(tag_name).cloned()),
        }
    } else {
        Ok(None)
    }
}

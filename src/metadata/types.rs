/// Metadata type definitions

use std::collections::HashMap;

/// Complete image metadata structure
#[derive(Debug, Clone, Default)]
pub struct ImageMetadata {
    pub exif: Option<ExifData>,
    pub gps: Option<GpsInfo>,
    pub camera: Option<CameraInfo>,
    pub basic: BasicInfo,
}

/// EXIF data structure
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ExifData {
    pub make: Option<String>,
    pub model: Option<String>,
    pub software: Option<String>,
    pub date_time: Option<String>,
    pub date_time_original: Option<String>,
    pub date_time_digitized: Option<String>,
    pub artist: Option<String>,
    pub copyright: Option<String>,
    pub description: Option<String>,
    pub user_comment: Option<String>,
    pub orientation: Option<u32>,
    pub x_resolution: Option<f64>,
    pub y_resolution: Option<f64>,
    pub resolution_unit: Option<String>,
    pub color_space: Option<String>,
    pub raw_data: HashMap<String, String>,
}

/// GPS information
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct GpsInfo {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub latitude_ref: Option<String>,
    pub longitude_ref: Option<String>,
    pub altitude_ref: Option<u8>,
    pub timestamp: Option<String>,
    pub date_stamp: Option<String>,
    pub speed: Option<f64>,
    pub direction: Option<f64>,
}

/// Camera settings information
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CameraInfo {
    pub iso: Option<u32>,
    pub exposure_time: Option<String>,
    pub f_number: Option<f64>,
    pub focal_length: Option<f64>,
    pub focal_length_35mm: Option<u32>,
    pub exposure_program: Option<String>,
    pub metering_mode: Option<String>,
    pub flash: Option<String>,
    pub white_balance: Option<String>,
    pub lens_make: Option<String>,
    pub lens_model: Option<String>,
}

/// Basic image information
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct BasicInfo {
    pub width: u32,
    pub height: u32,
    pub format: Option<String>,
    pub color_space: Option<String>,
    pub bit_depth: Option<u32>,
}

impl ImageMetadata {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Default::default()
    }
    
    /// Check if metadata contains EXIF data
    pub fn has_exif(&self) -> bool {
        self.exif.is_some()
    }
    
    /// Check if metadata contains GPS data
    pub fn has_gps(&self) -> bool {
        self.gps.is_some()
    }
    
    /// Check if metadata contains camera info
    #[allow(dead_code)]
    pub fn has_camera_info(&self) -> bool {
        self.camera.is_some()
    }
    
    /// Get a summary string of the metadata
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(ref exif) = self.exif {
            if let Some(ref make) = exif.make {
                if let Some(ref model) = exif.model {
                    parts.push(format!("{} {}", make, model));
                }
            }
        }
        
        if let Some(ref camera) = self.camera {
            if let Some(iso) = camera.iso {
                parts.push(format!("ISO {}", iso));
            }
            if let Some(ref exp) = camera.exposure_time {
                parts.push(format!("{}", exp));
            }
            if let Some(f) = camera.f_number {
                parts.push(format!("f/{:.1}", f));
            }
        }
        
        if self.has_gps() {
            parts.push("GPS".to_string());
        }
        
        if parts.is_empty() {
            format!("{}x{}", self.basic.width, self.basic.height)
        } else {
            format!("{}x{} | {}", self.basic.width, self.basic.height, parts.join(" | "))
        }
    }
}


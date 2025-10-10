/// EXIF and metadata handling module
/// 
/// Provides comprehensive EXIF/metadata support including:
/// - Reading EXIF data from images
/// - Writing EXIF data to images
/// - Accessing common metadata fields
/// - GPS information
/// - Camera settings
/// - Date/time information

pub mod reader;
pub mod writer;
pub mod types;

pub use reader::{read_exif, read_exif_from_path, extract_metadata};
pub use writer::{write_exif, preserve_exif};
pub use types::{ImageMetadata, ExifData, GpsInfo, CameraInfo};


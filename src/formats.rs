use crate::errors::ImgrsError;
use image::{ImageFormat, DynamicImage};
use std::path::Path;
use std::io::Cursor;

/// Format-specific save options
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FormatOptions {
    pub quality: Option<u8>,
    pub optimize: bool,
    pub progressive: bool,
    pub compress_level: Option<u8>,
    pub colors: Option<u8>,
    pub lossless: bool,
    pub subsampling: Option<String>,
    pub method: Option<u8>,
    pub interlace: bool,
    pub remove_metadata: bool,
    pub alpha_quality: Option<u8>,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            quality: Some(95),
            optimize: true,
            progressive: false,
            compress_level: Some(6),
            colors: None,
            lossless: false,
            subsampling: None,
            method: None,
            interlace: false,
            remove_metadata: false,
            alpha_quality: None,
        }
    }
}

/// Parse a format string into an ImageFormat with enhanced support
pub fn parse_format(format_str: &str) -> Result<ImageFormat, ImgrsError> {
    match format_str.to_uppercase().as_str() {
        "JPEG" | "JPG" | "JPE" => Ok(ImageFormat::Jpeg),
        "PNG" => Ok(ImageFormat::Png),
        "GIF" => Ok(ImageFormat::Gif),
        "BMP" => Ok(ImageFormat::Bmp),
        "TIFF" | "TIF" => Ok(ImageFormat::Tiff),
        "WEBP" => Ok(ImageFormat::WebP),
        "ICO" => Ok(ImageFormat::Ico),
        "PNM" => Ok(ImageFormat::Pnm),
        "DDS" => Ok(ImageFormat::Dds),
        "TGA" => Ok(ImageFormat::Tga),
        "FARBFELD" | "FF" => Ok(ImageFormat::Farbfeld),
        "AVIF" => Ok(ImageFormat::Avif),
        // HEIF, JPEG2000, and JXL are not supported by the image crate
        "HEIF" | "HEIC" | "JPEG2000" | "JP2" | "J2K" | "JXL" => {
            Err(ImgrsError::UnsupportedFormat(format!(
                "Format {} is not supported by the image crate",
                format_str
            )))
        }
        _ => Err(ImgrsError::UnsupportedFormat(format!(
            "Unsupported format: {}",
            format_str
        ))),
    }
}

/// Get format capabilities and limitations
#[allow(dead_code)]
pub fn get_format_capabilities(format_str: &str) -> Result<FormatCapabilities, ImgrsError> {
    parse_format(format_str)?;
    
    Ok(FormatCapabilities {
        format: format_str.to_string(),
        lossy: matches!(format_str.to_uppercase().as_str(), "JPEG" | "JPG" | "WEBP" | "AVIF"),
        lossless: matches!(format_str.to_uppercase().as_str(), "PNG" | "TIFF" | "GIF"),
        supports_transparency: matches!(format_str.to_uppercase().as_str(), "PNG" | "GIF" | "WEBP" | "AVIF"),
        supports_animation: matches!(format_str.to_uppercase().as_str(), "GIF" | "WEBP"),
        supports_icc_profile: matches!(format_str.to_uppercase().as_str(), "PNG" | "TIFF" | "JPEG"),
        max_dimensions: get_max_dimensions(format_str),
        extensions: get_format_extensions(format_str),
        mime_types: get_format_mime_types(format_str),
    })
}

/// Format capabilities structure
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FormatCapabilities {
    pub format: String,
    pub lossy: bool,
    pub lossless: bool,
    pub supports_transparency: bool,
    pub supports_animation: bool,
    pub supports_icc_profile: bool,
    pub max_dimensions: (u32, u32),
    pub extensions: Vec<String>,
    pub mime_types: Vec<String>,
}

/// Get default format options for a specific format
#[allow(dead_code)]
pub fn get_default_format_options(format_str: &str) -> Result<FormatOptions, ImgrsError> {
    let mut options = FormatOptions::default();
    
    match format_str.to_uppercase().as_str() {
        "JPEG" | "JPG" => {
            options.quality = Some(95);
            options.progressive = true;
            options.subsampling = Some("4:2:0".to_string());
        }
        "PNG" => {
            options.compress_level = Some(6);
            options.interlace = false;
        }
        "WEBP" => {
            options.quality = Some(80);
            options.method = Some(6);
            options.lossless = false;
        }
        "GIF" => {
            options.colors = Some(255);
            options.optimize = true;
        }
        "TIFF" => {
            options.compress_level = Some(6);
            options.lossless = true;
        }
        _ => {}
    }
    
    Ok(options)
}

/// Convert image to bytes with specified format and options
#[allow(dead_code)]
pub fn convert_to_bytes(
    image: &DynamicImage,
    format_str: &str,
    _options: Option<FormatOptions>,
) -> Result<Vec<u8>, ImgrsError> {
    let image_format = parse_format(format_str)?;
    
    let mut buffer = Cursor::new(Vec::new());
    image.write_to(&mut buffer, image_format)?;
    
    Ok(buffer.into_inner())
}

/// Convert image to specified format and save to file
#[allow(dead_code)]
pub fn convert_and_save(
    image: &DynamicImage,
    output_path: &Path,
    format_str: &str,
    _options: Option<FormatOptions>,
) -> Result<(), ImgrsError> {
    let image_format = parse_format(format_str)?;
    
    let file = std::fs::File::create(output_path)?;
    let mut writer = std::io::BufWriter::new(file);
    image.write_to(&mut writer, image_format)?;
    
    Ok(())
}

/// Validate conversion from source format to target format
#[allow(dead_code)]
pub fn validate_conversion(
    source_format: &str,
    target_format: &str,
    image_mode: &str,
) -> Result<ConversionValidation, ImgrsError> {
    let _source_capabilities = get_format_capabilities(source_format)?;
    let target_capabilities = get_format_capabilities(target_format)?;
    
    let mut validation = ConversionValidation {
        can_convert: true,
        warnings: Vec::new(),
        recommendations: Vec::new(),
        errors: Vec::new(),
    };
    
    // Check if formats are different
    if source_format.to_uppercase() == target_format.to_uppercase() {
        validation.warnings.push("Source and target formats are identical".to_string());
    }
    
    // Check mode compatibility
    if image_mode == "RGBA" && target_capabilities.lossy {
        validation.recommendations.push(
            "Consider converting to RGB first to avoid background color issues".to_string()
        );
    }
    
    if image_mode == "CMYK" && !target_capabilities.supports_icc_profile {
        validation.warnings.push(
            "CMYK to RGB conversion may affect color appearance".to_string()
        );
    }
    
    // Check transparency support
    if image_mode.contains("A") && !target_capabilities.supports_transparency {
        validation.errors.push(format!(
            "Format {} does not support transparency (alpha channel)",
            target_format
        ));
        validation.can_convert = false;
    }
    
    Ok(validation)
}

/// Conversion validation result
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConversionValidation {
    pub can_convert: bool,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
    pub errors: Vec<String>,
}

/// Get maximum supported dimensions for a format
fn get_max_dimensions(format_str: &str) -> (u32, u32) {
    match format_str.to_uppercase().as_str() {
        "JPEG" | "JPG" | "PNG" | "GIF" | "WEBP" => (65535, 65535),
        "TIFF" => (u32::MAX, u32::MAX),
        "AVIF" => (16383, 16383),
        _ => (8192, 8192), // Conservative default
    }
}

/// Get file extensions for a format
fn get_format_extensions(format_str: &str) -> Vec<String> {
    match format_str.to_uppercase().as_str() {
        "JPEG" | "JPG" => vec![".jpg".to_string(), ".jpeg".to_string(), ".jpe".to_string()],
        "PNG" => vec![".png".to_string()],
        "GIF" => vec![".gif".to_string()],
        "BMP" => vec![".bmp".to_string()],
        "TIFF" | "TIF" => vec![".tiff".to_string(), ".tif".to_string()],
        "WEBP" => vec![".webp".to_string()],
        "ICO" => vec![".ico".to_string()],
        "AVIF" => vec![".avif".to_string()],
        _ => vec![format!(".{}", format_str.to_lowercase())],
    }
}

/// Get MIME types for a format
fn get_format_mime_types(format_str: &str) -> Vec<String> {
    match format_str.to_uppercase().as_str() {
        "JPEG" | "JPG" => vec!["image/jpeg".to_string()],
        "PNG" => vec!["image/png".to_string()],
        "GIF" => vec!["image/gif".to_string()],
        "BMP" => vec!["image/bmp".to_string()],
        "TIFF" | "TIF" => vec!["image/tiff".to_string()],
        "WEBP" => vec!["image/webp".to_string()],
        "ICO" => vec!["image/x-icon".to_string()],
        "AVIF" => vec!["image/avif".to_string()],
        _ => vec![format!("image/{}", format_str.to_lowercase())],
    }
}

/// Font loading and management with ttf/otf/woff/woff2 support

use ab_glyph::FontVec;
use crate::errors::ImgrsError;
use std::path::Path;
use std::fs;

#[allow(dead_code)]

/// Embedded default fonts
const DEFAULT_FONT: &[u8] = include_bytes!("../../fonts/DejaVuSans.ttf");
    #[allow(dead_code)]
const BOLD_FONT: &[u8] = include_bytes!("../../fonts/DejaVuSans.ttf"); // TODO: Add DejaVuSans-Bold.ttf

/// Font format types
#[derive(Debug, Clone, Copy, PartialEq)]
enum FontFormat {
    Ttf,
    Otf,
    Woff,
    Woff2,
}

/// Font manager for loading and caching fonts
    #[allow(dead_code)]
pub struct FontManager {
    default_font: Option<FontVec>,
    custom_fonts: Vec<FontVec>,
}

impl FontManager {
    #[allow(dead_code)]
    pub fn new() -> Self {
        FontManager {
            default_font: None,
            custom_fonts: Vec::new(),
        }
    }
    
    /// Get or load default font
    #[allow(dead_code)]
    pub fn get_default(&mut self) -> Result<&FontVec, ImgrsError> {
        if self.default_font.is_none() {
            self.default_font = Some(load_embedded_font()?);
        }
        Ok(self.default_font.as_ref().unwrap())
    }
    
    /// Load custom font from path
    #[allow(dead_code)]
    pub fn load_custom(&mut self, path: &Path) -> Result<usize, ImgrsError> {
        let font = load_font_from_path(path)?;
        self.custom_fonts.push(font);
        Ok(self.custom_fonts.len() - 1)
    }
    
    /// Get custom font by index
    #[allow(dead_code)]
    pub fn get_custom(&self, index: usize) -> Option<&FontVec> {
        self.custom_fonts.get(index)
    }
}

/// Detect font format from file extension or magic bytes
fn detect_font_format(path: &Path, data: &[u8]) -> FontFormat {
    // First try file extension
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        match ext_str.as_str() {
            "ttf" => return FontFormat::Ttf,
            "otf" => return FontFormat::Otf,
            "woff" => return FontFormat::Woff,
            "woff2" => return FontFormat::Woff2,
            _ => {}
        }
    }
    
    // Fall back to magic bytes detection
    if data.len() >= 4 {
        match &data[0..4] {
            b"wOFF" => return FontFormat::Woff,
            b"wOF2" => return FontFormat::Woff2,
            [0x00, 0x01, 0x00, 0x00] => return FontFormat::Ttf,
            b"OTTO" => return FontFormat::Otf,
            _ => {}
        }
    }
    
    // Default to TTF
    FontFormat::Ttf
}

/// Convert WOFF to TTF format
fn convert_woff_to_ttf(woff_data: &[u8]) -> Result<Vec<u8>, ImgrsError> {
    wuff::decompress_woff1(woff_data)
        .map_err(|e| ImgrsError::InvalidOperation(format!("Failed to convert WOFF to TTF: {:?}", e)))
}

/// Convert WOFF2 to TTF format
fn convert_woff2_to_ttf(woff2_data: &[u8]) -> Result<Vec<u8>, ImgrsError> {
    wuff::decompress_woff2(woff2_data)
        .map_err(|e| ImgrsError::InvalidOperation(format!("Failed to convert WOFF2 to TTF: {:?}", e)))
}

/// Load embedded default font
fn load_embedded_font() -> Result<FontVec, ImgrsError> {
    FontVec::try_from_vec(DEFAULT_FONT.to_vec())
        .map_err(|e| ImgrsError::InvalidOperation(format!("Failed to load embedded font: {:?}", e)))
}

/// Load font from file path (TTF, OTF, WOFF, or WOFF2)
pub fn load_font_from_path(path: impl AsRef<Path>) -> Result<FontVec, ImgrsError> {
    let path = path.as_ref();
    let font_data = fs::read(path)
        .map_err(|e| ImgrsError::InvalidOperation(format!("Failed to read font file: {}", e)))?;
    
    // Detect format
    let format = detect_font_format(path, &font_data);
    
    // Convert to TTF/OTF if needed
    let ttf_data = match format {
        FontFormat::Ttf | FontFormat::Otf => font_data,
        FontFormat::Woff => convert_woff_to_ttf(&font_data)?,
        FontFormat::Woff2 => convert_woff2_to_ttf(&font_data)?,
    };
    
    FontVec::try_from_vec(ttf_data)
        .map_err(|e| ImgrsError::InvalidOperation(format!("Invalid font file: {:?}", e)))
}

/// Load font from bytes
#[allow(dead_code)]
pub fn load_font_from_bytes(data: &[u8]) -> Result<FontVec, ImgrsError> {
    // Try to detect format from magic bytes
    let format = if data.len() >= 4 {
        match &data[0..4] {
            b"wOFF" => FontFormat::Woff,
            b"wOF2" => FontFormat::Woff2,
            [0x00, 0x01, 0x00, 0x00] => FontFormat::Ttf,
            b"OTTO" => FontFormat::Otf,
            _ => FontFormat::Ttf,
        }
    } else {
        FontFormat::Ttf
    };
    
    // Convert to TTF/OTF if needed
    let ttf_data = match format {
        FontFormat::Ttf | FontFormat::Otf => data.to_vec(),
        FontFormat::Woff => convert_woff_to_ttf(data)?,
        FontFormat::Woff2 => convert_woff2_to_ttf(data)?,
    };
    
    FontVec::try_from_vec(ttf_data)
        .map_err(|e| ImgrsError::InvalidOperation(format!("Invalid font data: {:?}", e)))
}

/// Get default font (convenience function)
pub fn get_default_font() -> Result<FontVec, ImgrsError> {
    load_embedded_font()
}

/// Load font - try path first, fall back to default
pub fn load_font(path: Option<&Path>) -> Result<FontVec, ImgrsError> {
    match path {
        Some(p) => load_font_from_path(p),
        None => get_default_font(),
    }
}
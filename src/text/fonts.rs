/// Font loading and management

use ab_glyph::FontVec;
use crate::errors::ImgrsError;
use std::path::Path;
use std::fs;

#[allow(dead_code)]

/// Embedded default fonts
const DEFAULT_FONT: &[u8] = include_bytes!("../../fonts/DejaVuSans.ttf");
    #[allow(dead_code)]
const BOLD_FONT: &[u8] = include_bytes!("../../fonts/DejaVuSans.ttf"); // TODO: Add DejaVuSans-Bold.ttf

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

/// Load embedded default font
fn load_embedded_font() -> Result<FontVec, ImgrsError> {
    FontVec::try_from_vec(DEFAULT_FONT.to_vec())
        .map_err(|e| ImgrsError::InvalidOperation(format!("Failed to load embedded font: {:?}", e)))
}

/// Load font from file path (TTF or OTF)
pub fn load_font_from_path(path: impl AsRef<Path>) -> Result<FontVec, ImgrsError> {
    let font_data = fs::read(path.as_ref())
        .map_err(|e| ImgrsError::InvalidOperation(format!("Failed to read font file: {}", e)))?;
    
    FontVec::try_from_vec(font_data)
        .map_err(|e| ImgrsError::InvalidOperation(format!("Invalid font file: {:?}", e)))
}

/// Load font from bytes
#[allow(dead_code)]
pub fn load_font_from_bytes(data: &[u8]) -> Result<FontVec, ImgrsError> {
    FontVec::try_from_vec(data.to_vec())
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


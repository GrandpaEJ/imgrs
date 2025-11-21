/// Enhanced text rendering operations for PyImage with comprehensive font support

use crate::errors::ImgrsError;
use crate::image::core::{PyImage, LazyImage};
use crate::text::{draw_text, draw_text_styled, draw_text_centered, draw_text_multiline, TextStyle, TextAlign, wrap_text};
use crate::text::styles::FontWeight;
use crate::text::{get_text_size, get_multiline_text_size};
use pyo3::prelude::*;
use pyo3::{PyObject, Python};

impl PyImage {
    /// Basic rich text drawing
    pub fn draw_rich_text_impl(
        &self,
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_path: Option<&str>,
    ) -> Result<PyImage, ImgrsError> {
        let image = match &self.lazy_image {
            LazyImage::Loaded(img) => img,
            LazyImage::Path { path } => {
                let img = image::open(path)?;
                return Self::draw_rich_text_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, x, y, size, color, font_path
                );
            }
            LazyImage::Bytes { data } => {
                let img = image::load_from_memory(data)?;
                return Self::draw_rich_text_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, x, y, size, color, font_path
                );
            }
        };

        let font_path = font_path.map(|s| std::path::Path::new(s));
        let result = draw_text(image, text, x, y, size, color, font_path)?;
        Ok(PyImage::new_from_image(result, self.format))
    }

    /// Styled rich text drawing
    pub fn draw_rich_text_styled_impl(
        &self,
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_path: Option<&str>,
        align: Option<&str>,
        background: Option<(u8, u8, u8, u8)>,
        outline: Option<(u8, u8, u8, u8, f32)>,
        shadow: Option<(i32, i32, u8, u8, u8, u8)>,
        opacity: Option<f32>,
        line_spacing: Option<f32>,
        letter_spacing: Option<f32>,
        max_width: Option<u32>,
        rotation: Option<f32>,
    ) -> Result<PyImage, ImgrsError> {
        let image = match &self.lazy_image {
            LazyImage::Loaded(img) => img,
            LazyImage::Path { path } => {
                let img = image::open(path)?;
                return Self::draw_rich_text_styled_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, x, y, size, color, font_path, align, background,
                    outline, shadow, opacity, line_spacing, letter_spacing,
                    max_width, rotation
                );
            }
            LazyImage::Bytes { data } => {
                let img = image::load_from_memory(data)?;
                return Self::draw_rich_text_styled_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, x, y, size, color, font_path, align, background,
                    outline, shadow, opacity, line_spacing, letter_spacing,
                    max_width, rotation
                );
            }
        };

        let style = TextStyle {
            color,
            size,
            weight: FontWeight::Normal,
            align: align.map(|a| match a {
                "center" => TextAlign::Center,
                "right" => TextAlign::Right,
                _ => TextAlign::Left,
            }).unwrap_or(TextAlign::Left),
            background,
            outline,
            shadow,
            glow: None,
            opacity: opacity.unwrap_or(1.0),
            line_spacing: line_spacing.unwrap_or(1.2),
            letter_spacing: letter_spacing.unwrap_or(0.0),
            max_width,
            rotation: rotation.unwrap_or(0.0),
        };

        let font_path = font_path.map(|s| std::path::Path::new(s));
        let result = draw_text_styled(image, text, x, y, &style, font_path)?;
        Ok(PyImage::new_from_image(result, self.format))
    }

    /// Centered rich text drawing
    pub fn draw_rich_text_centered_impl(
        &self,
        text: &str,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_path: Option<&str>,
    ) -> Result<PyImage, ImgrsError> {
        let image = match &self.lazy_image {
            LazyImage::Loaded(img) => img,
            LazyImage::Path { path } => {
                let img = image::open(path)?;
                return Self::draw_rich_text_centered_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, y, size, color, font_path
                );
            }
            LazyImage::Bytes { data } => {
                let img = image::load_from_memory(data)?;
                return Self::draw_rich_text_centered_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, y, size, color, font_path
                );
            }
        };

        let style = TextStyle {
            color,
            size,
            weight: FontWeight::Normal,
            align: TextAlign::Center,
            background: None,
            outline: None,
            shadow: None,
            glow: None,
            opacity: 1.0,
            line_spacing: 1.2,
            letter_spacing: 0.0,
            max_width: None,
            rotation: 0.0,
        };

        let font_path = font_path.map(|s| std::path::Path::new(s));
        let result = draw_text_centered(image, text, y, &style, font_path)?;
        Ok(PyImage::new_from_image(result, self.format))
    }

    /// Multiline rich text drawing
    pub fn draw_rich_text_multiline_impl(
        &self,
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_path: Option<&str>,
        line_spacing: Option<f32>,
        align: Option<&str>,
    ) -> Result<PyImage, ImgrsError> {
        let image = match &self.lazy_image {
            LazyImage::Loaded(img) => img,
            LazyImage::Path { path } => {
                let img = image::open(path)?;
                return Self::draw_rich_text_multiline_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, x, y, size, color, font_path, line_spacing, align
                );
            }
            LazyImage::Bytes { data } => {
                let img = image::load_from_memory(data)?;
                return Self::draw_rich_text_multiline_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, x, y, size, color, font_path, line_spacing, align
                );
            }
        };

        let style = TextStyle {
            color,
            size,
            weight: FontWeight::Normal,
            align: align.map(|a| match a {
                "center" => TextAlign::Center,
                "right" => TextAlign::Right,
                _ => TextAlign::Left,
            }).unwrap_or(TextAlign::Left),
            background: None,
            outline: None,
            shadow: None,
            glow: None,
            opacity: 1.0,
            line_spacing: line_spacing.unwrap_or(1.2),
            letter_spacing: 0.0,
            max_width: None,
            rotation: 0.0,
        };

        let font_path = font_path.map(|s| std::path::Path::new(s));
        let result = draw_text_multiline(image, text, x, y, &style, font_path)?;
        Ok(PyImage::new_from_image(result, self.format))
    }

    /// Advanced text drawing with comprehensive styling
    pub fn draw_advanced_text_impl(
        &self,
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_family: &str,
        font_weight: &str,
        font_style: &str,
        font_path: Option<&str>,
        letter_spacing: f32,
        opacity: f32,
        align: Option<&str>,
        background: Option<(u8, u8, u8, u8)>,
        outline: Option<(u8, u8, u8, u8, f32)>,
        shadow: Option<(i32, i32, u8, u8, u8, u8)>,
        glow: Option<(u8, u8, u8, u8, f32)>,
        max_width: Option<u32>,
        line_spacing: f32,
        text_justify: &str,
        rotation: f32,
    ) -> Result<PyImage, ImgrsError> {
        let image = match &self.lazy_image {
            LazyImage::Loaded(img) => img,
            LazyImage::Path { path } => {
                let img = image::open(path)?;
                return Self::draw_advanced_text_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, x, y, size, color, font_family, font_weight, font_style,
                    font_path, letter_spacing, opacity, align, background,
                    outline, shadow, glow, max_width, line_spacing, text_justify, rotation
                );
            }
            LazyImage::Bytes { data } => {
                let img = image::load_from_memory(data)?;
                return Self::draw_advanced_text_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, x, y, size, color, font_family, font_weight, font_style,
                    font_path, letter_spacing, opacity, align, background,
                    outline, shadow, glow, max_width, line_spacing, text_justify, rotation
                );
            }
        };

        let resolved_font = resolve_font_path(font_path, font_family, font_weight, font_style)?;
        let align_enum = align.map(|a| match a {
            "center" => TextAlign::Center,
            "right" => TextAlign::Right,
            _ => TextAlign::Left,
        }).unwrap_or(TextAlign::Left);

        let style = TextStyle {
            color,
            size,
            weight: FontWeight::Normal,
            align: align_enum,
            background,
            outline,
            shadow,
            glow,
            opacity,
            line_spacing,
            letter_spacing,
            max_width,
            rotation,
        };

        let result = if text.contains('\n') || max_width.is_some() {
            let wrapped_text = if let Some(max_width) = max_width {
                wrap_text(text, max_width, size, resolved_font.as_deref())?
            } else {
                text.to_string()
            };
            draw_text_multiline(image, &wrapped_text, x, y, &style, resolved_font.as_deref())?
        } else {
            draw_text_styled(image, text, x, y, &style, resolved_font.as_deref())?
        };

        Ok(PyImage::new_from_image(result, self.format))
    }

    /// Multiline enhanced text drawing
    pub fn draw_multiline_text_impl(
        &self,
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_family: &str,
        font_weight: &str,
        font_style: &str,
        font_path: Option<&str>,
        line_spacing: f32,
        letter_spacing: f32,
        align: Option<&str>,
        text_justify: &str,
        max_width: Option<u32>,
        opacity: f32,
    ) -> Result<PyImage, ImgrsError> {
        // For now, use basic multiline with resolved font
        let image = match &self.lazy_image {
            LazyImage::Loaded(img) => img,
            LazyImage::Path { path } => {
                let img = image::open(path)?;
                return Self::draw_multiline_text_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, x, y, size, color, font_family, font_weight, font_style,
                    font_path, line_spacing, letter_spacing, align, text_justify,
                    max_width, opacity
                );
            }
            LazyImage::Bytes { data } => {
                let img = image::load_from_memory(data)?;
                return Self::draw_multiline_text_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, x, y, size, color, font_family, font_weight, font_style,
                    font_path, line_spacing, letter_spacing, align, text_justify,
                    max_width, opacity
                );
            }
        };

        let resolved_font = resolve_font_path(font_path, font_family, font_weight, font_style)?;
        let align_enum = align.map(|a| match a {
            "center" => TextAlign::Center,
            "right" => TextAlign::Right,
            _ => TextAlign::Left,
        }).unwrap_or(TextAlign::Left);

        let style = TextStyle {
            color,
            size,
            weight: FontWeight::Normal,
            align: align_enum,
            background: None,
            outline: None,
            shadow: None,
            glow: None,
            opacity: opacity,
            line_spacing,
            letter_spacing,
            max_width,
            rotation: 0.0,
        };

        let result = draw_text_multiline(image, text, x, y, &style, resolved_font.as_deref())?;
        Ok(PyImage::new_from_image(result, self.format))
    }

    /// Centered enhanced text drawing
    pub fn draw_centered_text_impl(
        &self,
        text: &str,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_family: &str,
        font_weight: &str,
        font_style: &str,
        font_path: Option<&str>,
        opacity: f32,
        letter_spacing: f32,
    ) -> Result<PyImage, ImgrsError> {
        let image = match &self.lazy_image {
            LazyImage::Loaded(img) => img,
            LazyImage::Path { path } => {
                let img = image::open(path)?;
                return Self::draw_centered_text_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, y, size, color, font_family, font_weight, font_style,
                    font_path, opacity, letter_spacing
                );
            }
            LazyImage::Bytes { data } => {
                let img = image::load_from_memory(data)?;
                return Self::draw_centered_text_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, y, size, color, font_family, font_weight, font_style,
                    font_path, opacity, letter_spacing
                );
            }
        };

        let resolved_font = resolve_font_path(font_path, font_family, font_weight, font_style)?;
        let style = TextStyle {
            color,
            size,
            weight: FontWeight::Normal,
            align: TextAlign::Center,
            background: None,
            outline: None,
            shadow: None,
            glow: None,
            opacity,
            line_spacing: 1.2,
            letter_spacing,
            max_width: None,
            rotation: 0.0,
        };

        let result = draw_text_centered(image, text, y, &style, resolved_font.as_deref())?;
        Ok(PyImage::new_from_image(result, self.format))
    }

    /// Multi-font text drawing
    pub fn draw_multi_font_text_impl(
        &self,
        text: &str,
        x: i32,
        y: i32,
        _fonts: &Bound<'_, PyAny>,
    ) -> Result<PyImage, ImgrsError> {
        // TODO: Implement multi-font text drawing
        // For now, use basic text
        self.draw_rich_text_impl(text, x, y, 32.0, (0, 0, 0, 255), None)
    }

    /// Enhanced text drawing with font family and weight support
    pub fn draw_enhanced_text_impl(
        &self,
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_family: &str,
        font_weight: &str,
        font_style: &str,
        font_path: Option<&str>,
        letter_spacing: f32,
        opacity: f32,
    ) -> Result<PyImage, ImgrsError> {
        let image = match &self.lazy_image {
            LazyImage::Loaded(img) => img,
            LazyImage::Path { path } => {
                let img = image::open(path)?;
                return Self::draw_enhanced_text_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, x, y, size, color, font_family, font_weight, font_style,
                    font_path, letter_spacing, opacity
                );
            }
            LazyImage::Bytes { data } => {
                let img = image::load_from_memory(data)?;
                return Self::draw_enhanced_text_impl(
                    &PyImage::new_from_image(img, self.format),
                    text, x, y, size, color, font_family, font_weight, font_style,
                    font_path, letter_spacing, opacity
                );
            }
        };
        
        // Enhanced font resolution with family fallback
        let resolved_font = resolve_font_path(font_path, font_family, font_weight, font_style)?;
        let result = draw_text(image, text, x, y, size, color, resolved_font.as_deref())?;
        Ok(PyImage::new_from_image(result, self.format))
    }
    
    /// List available fonts
    pub fn list_available_fonts_impl() -> PyResult<Vec<String>> {
        let mut fonts = Vec::new();
        
        // Check fonts directory
        if let Ok(entries) = std::fs::read_dir("fonts") {
            for entry in entries.flatten() {
                if let Some(path) = entry.path().to_str() {
                    if path.ends_with(".ttf") || path.ends_with(".otf") || path.ends_with(".ttc") || path.ends_with(".woff2") {
                        fonts.push(path.to_string());
                    }
                }
            }
        }
        
        // Add system fonts if they exist
        let system_font_paths = [
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/System/Library/Fonts/Arial.ttf",
            "/Windows/Fonts/arial.ttf",
        ];
        
        for font_path in system_font_paths {
            if std::path::Path::new(font_path).exists() {
                fonts.push(font_path.to_string());
            }
        }
        
        fonts.sort();
        fonts.dedup();
        Ok(fonts)
    }

    /// Get text size
    pub fn get_text_size_impl(text: &str, size: f32, font_path: Option<&str>) -> Result<(u32, u32), ImgrsError> {
        let font_path = font_path.map(|s| std::path::Path::new(s));
        let (width, height, _, _) = get_text_size(text, size, font_path)?;
        Ok((width, height))
    }

    /// Get multiline text size
    pub fn get_multiline_text_size_impl(text: &str, size: f32, line_spacing: f32, font_path: Option<&str>) -> Result<(u32, u32, usize), ImgrsError> {
        let font_path = font_path.map(|s| std::path::Path::new(s));
        get_multiline_text_size(text, size, line_spacing, font_path)
    }

    /// Get text box
    pub fn get_text_box_impl(text: &str, x: i32, y: i32, size: f32, font_path: Option<&str>) -> Result<PyObject, ImgrsError> {
        use crate::text::get_text_box;
        use pyo3::types::PyDict;

        let font_path = font_path.map(|s| std::path::Path::new(s));
        let text_box = get_text_box(text, x, y, size, font_path)?;

        Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new_bound(py);
            dict.set_item("x", text_box.x)?;
            dict.set_item("y", text_box.y)?;
            dict.set_item("width", text_box.width)?;
            dict.set_item("height", text_box.height)?;
            dict.set_item("ascent", text_box.ascent)?;
            dict.set_item("descent", text_box.descent)?;
            dict.set_item("baseline_y", text_box.baseline_y)?;
            dict.set_item("bottom_y", text_box.bottom_y)?;
            dict.set_item("right_x", text_box.right_x)?;
            dict.set_item("advance_width", text_box.width as i32)?; // Using width as advance for now
            dict.set_item("left_bearing", 0)?; // Not implemented
            dict.set_item("right_bearing", 0)?; // Not implemented
            Ok(dict.to_object(py))
        })
    }

    /// Get enhanced text size
    pub fn get_enhanced_text_size_impl(text: &str, size: f32, font_family: &str, font_weight: &str, font_style: &str, font_path: Option<&str>, _letter_spacing: f32) -> Result<(u32, u32), ImgrsError> {
        let resolved_font = resolve_font_path(font_path, font_family, font_weight, font_style)?;
        let (width, height, _, _) = get_text_size(text, size, resolved_font.as_deref())?;
        Ok((width, height))
    }

    /// Get enhanced multiline text size
    pub fn get_enhanced_multiline_text_size_impl(text: &str, size: f32, line_spacing: f32, font_family: &str, font_weight: &str, font_style: &str, font_path: Option<&str>, _letter_spacing: f32, _max_width: Option<u32>) -> Result<(u32, u32, usize), ImgrsError> {
        let resolved_font = resolve_font_path(font_path, font_family, font_weight, font_style)?;
        get_multiline_text_size(text, size, line_spacing, resolved_font.as_deref())
    }

    /// Get enhanced text box
    pub fn get_enhanced_text_box_impl(text: &str, x: i32, y: i32, size: f32, font_family: &str, font_weight: &str, font_style: &str, font_path: Option<&str>, _letter_spacing: f32) -> Result<PyObject, ImgrsError> {
        // For now, use the basic get_text_box since enhanced font resolution is not fully implemented
        // TODO: Implement proper enhanced font resolution
        let resolved_font = resolve_font_path(font_path, font_family, font_weight, font_style)?;
        use crate::text::get_text_box;
        use pyo3::types::PyDict;

        let text_box = get_text_box(text, x, y, size, resolved_font.as_deref())?;

        Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new_bound(py);
            dict.set_item("x", text_box.x)?;
            dict.set_item("y", text_box.y)?;
            dict.set_item("width", text_box.width)?;
            dict.set_item("height", text_box.height)?;
            dict.set_item("ascent", text_box.ascent)?;
            dict.set_item("descent", text_box.descent)?;
            dict.set_item("baseline_y", text_box.baseline_y)?;
            dict.set_item("bottom_y", text_box.bottom_y)?;
            dict.set_item("right_x", text_box.right_x)?;
            dict.set_item("advance_width", text_box.width as i32)?; // Using width as advance for now
            dict.set_item("left_bearing", 0)?; // Not implemented
            dict.set_item("right_bearing", 0)?; // Not implemented
            Ok(dict.to_object(py))
        })
    }
}

/// Enhanced font resolution with family fallback
fn resolve_font_path(
    explicit_path: Option<&str>,
    _font_family: &str,
    _font_weight: &str,
    _font_style: &str,
) -> Result<Option<std::path::PathBuf>, ImgrsError> {
    // If explicit path provided, use it
    if let Some(path) = explicit_path {
        let path_buf = std::path::Path::new(path).to_path_buf();
        if path_buf.exists() {
            return Ok(Some(path_buf));
        }
    }

    // For now, always use embedded font to avoid file path issues
    Ok(None)
}

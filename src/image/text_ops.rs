/// Text rendering operations for PyImage

use crate::errors::ImgrsError;
use crate::image::core::{PyImage, LazyImage};
use crate::text::{draw_text, draw_text_styled, draw_text_centered, TextStyle, TextAlign};
use crate::text::{get_text_size, get_multiline_text_size, get_text_box};
use std::path::Path;
use pyo3::prelude::*;
use pyo3::types::PyDict;

impl PyImage {
    /// Draw rich text on image (basic)
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
        
        let font = font_path.map(Path::new);
        let result = draw_text(image, text, x, y, size, color, font)?;
        Ok(PyImage::new_from_image(result, self.format))
    }
    
    /// Draw rich text with full styling
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

        let mut style = TextStyle::new()
            .with_size(size)
            .with_color(color.0, color.1, color.2, color.3);
        
        // Apply optional parameters
        if let Some(bg) = background {
            style = style.with_background(bg.0, bg.1, bg.2, bg.3);
        }
        
        if let Some(align_str) = align {
            let alignment = match align_str.to_lowercase().as_str() {
                "center" => TextAlign::Center,
                "right" => TextAlign::Right,
                _ => TextAlign::Left,
            };
            style = style.with_align(alignment);
        }
        
        if let Some(out) = outline {
            style = style.with_outline(out.0, out.1, out.2, out.3, out.4);
        }
        
        if let Some(shd) = shadow {
            style = style.with_shadow(shd.0, shd.1, shd.2, shd.3, shd.4, shd.5);
        }
        
        if let Some(op) = opacity {
            style = style.with_opacity(op);
        }
        
        if let Some(ls) = line_spacing {
            style.line_spacing = ls;
        }
        
        if let Some(ls) = letter_spacing {
            style.letter_spacing = ls;
        }
        
        if let Some(mw) = max_width {
            style = style.with_max_width(mw);
        }
        
        if let Some(rot) = rotation {
            style = style.with_rotation(rot);
        }
        
        let font = font_path.map(Path::new);
        let result = draw_text_styled(image, text, x, y, &style, font)?;
        Ok(PyImage::new_from_image(result, self.format))
    }
    
    /// Draw centered rich text
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
        
        let style = TextStyle::new()
            .with_size(size)
            .with_color(color.0, color.1, color.2, color.3)
            .with_align(TextAlign::Center);
        
        let font = font_path.map(Path::new);
        let result = draw_text_centered(image, text, y, &style, font)?;
        Ok(PyImage::new_from_image(result, self.format))
    }
    
    /// Draw multiline rich text
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
        
        let mut style = TextStyle::new()
            .with_size(size)
            .with_color(color.0, color.1, color.2, color.3);
        
        if let Some(ls) = line_spacing {
            style.line_spacing = ls;
        }
        
        if let Some(align_str) = align {
            let alignment = match align_str.to_lowercase().as_str() {
                "center" => TextAlign::Center,
                "right" => TextAlign::Right,
                _ => TextAlign::Left,
            };
            style = style.with_align(alignment);
        }
        
        let font = font_path.map(Path::new);
        let result = crate::text::draw_text_multiline(image, text, x, y, &style, font)?;
        Ok(PyImage::new_from_image(result, self.format))
    }
    
    /// Get text bounding box size
    pub fn get_text_size_impl(
        text: &str,
        size: f32,
        font_path: Option<&str>,
    ) -> Result<(u32, u32), ImgrsError> {
        let font = font_path.map(Path::new);
        let (width, height, _, _) = get_text_size(text, size, font)?;
        Ok((width, height))
    }
    
    /// Get multiline text size
    pub fn get_multiline_text_size_impl(
        text: &str,
        size: f32,
        line_spacing: f32,
        font_path: Option<&str>,
    ) -> Result<(u32, u32, usize), ImgrsError> {
        let font = font_path.map(Path::new);
        get_multiline_text_size(text, size, line_spacing, font)
    }
    
    /// Get text box with all information
    pub fn get_text_box_impl(
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        font_path: Option<&str>,
    ) -> PyResult<PyObject> {
        let font = font_path.map(Path::new);
        let textbox = get_text_box(text, x, y, size, font)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        Python::with_gil(|py| {
            let dict = PyDict::new_bound(py);
            dict.set_item("x", textbox.x)?;
            dict.set_item("y", textbox.y)?;
            dict.set_item("width", textbox.width)?;
            dict.set_item("height", textbox.height)?;
            dict.set_item("ascent", textbox.ascent)?;
            dict.set_item("descent", textbox.descent)?;
            dict.set_item("baseline_y", textbox.baseline_y)?;
            dict.set_item("bottom_y", textbox.bottom_y)?;
            dict.set_item("right_x", textbox.right_x)?;
            Ok(dict.into())
        })
    }
}


use pyo3::prelude::*;
use crate::emoji;
use super::core::{PyImage, LazyImage};

impl PyImage {
    /// Add emoji to image using preset type
    pub fn add_emoji_impl(
        &mut self,
        emoji_name: &str,
        x: i32,
        y: i32,
        size: u32,
        opacity: f32,
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        // Parse emoji type from name
        let emoji_type = emoji::EmojiType::from_name(emoji_name)
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Unknown emoji type: {}", emoji_name)
                )
            })?;

        let style = emoji::EmojiStyle {
            size,
            x,
            y,
            opacity,
            background: None,
            color: None,
        };

        Python::with_gil(|py| {
            py.allow_threads(|| emoji::add_emoji(image, emoji_type, style))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    /// Add emoji using Unicode text
    pub fn add_emoji_text_impl(
        &mut self,
        emoji: &str,
        x: i32,
        y: i32,
        size: u32,
        opacity: f32,
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        let style = emoji::EmojiStyle {
            size,
            x,
            y,
            opacity,
            background: None,
            color: None,
        };

        Python::with_gil(|py| {
            py.allow_threads(|| emoji::add_emoji_text(image, emoji, style))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    /// Add emoji quickly with minimal parameters
    pub fn add_emoji_quick_impl(
        &mut self,
        emoji_name: &str,
        x: i32,
        y: i32,
        size: u32,
    ) -> PyResult<Self> {
        self.add_emoji_impl(emoji_name, x, y, size, 1.0)
    }

    /// Add multiple emojis at once
    pub fn add_emojis_batch_impl(
        &mut self,
        emojis: Vec<(String, i32, i32, u32, f32)>,
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        let emoji_list: Result<Vec<(emoji::EmojiType, emoji::EmojiStyle)>, PyErr> = emojis
            .into_iter()
            .map(|(name, x, y, size, opacity)| {
                emoji::EmojiType::from_name(&name)
                    .ok_or_else(|| {
                        PyErr::new::<pyo3::exceptions::PyValueError, _>(
                            format!("Unknown emoji type: {}", name)
                        )
                    })
                    .map(|emoji_type| {
                        let style = emoji::EmojiStyle {
                            size,
                            x,
                            y,
                            opacity,
                            background: None,
                            color: None,
                        };
                        (emoji_type, style)
                    })
            })
            .collect();

        let emoji_list = emoji_list?;

        Python::with_gil(|py| {
            py.allow_threads(|| emoji::add_emojis_batch(image, emoji_list))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }
}


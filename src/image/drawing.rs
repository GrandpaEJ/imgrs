use pyo3::prelude::*;
use crate::drawing;
use super::core::{PyImage, LazyImage};

impl PyImage {
    pub fn draw_rectangle_impl(&mut self, x: i32, y: i32, width: u32, height: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                drawing::draw_rectangle(image, x, y, width, height, color)
            })
        }).map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        }).map_err(|e| e.into())
    }

    pub fn draw_circle_impl(&mut self, center_x: i32, center_y: i32, radius: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                drawing::draw_circle(image, center_x, center_y, radius, color)
            })
        }).map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        }).map_err(|e| e.into())
    }

    pub fn draw_line_impl(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                drawing::draw_line(image, x0, y0, x1, y1, color)
            })
        }).map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        }).map_err(|e| e.into())
    }

    pub fn draw_text_impl(&mut self, text: &str, x: i32, y: i32, color: (u8, u8, u8, u8), scale: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                drawing::draw_text(image, text, x, y, color, scale)
            })
        }).map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        }).map_err(|e| e.into())
    }
}


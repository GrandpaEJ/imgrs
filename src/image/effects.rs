use pyo3::prelude::*;
use crate::shadows;
use super::core::{PyImage, LazyImage};

impl PyImage {
    pub fn drop_shadow_impl(&mut self, offset_x: i32, offset_y: i32, blur_radius: f32, shadow_color: (u8, u8, u8, u8)) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                shadows::drop_shadow(image, offset_x, offset_y, blur_radius, shadow_color)
            })
        }).map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        }).map_err(|e| e.into())
    }

    pub fn inner_shadow_impl(&mut self, offset_x: i32, offset_y: i32, blur_radius: f32, shadow_color: (u8, u8, u8, u8)) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                shadows::inner_shadow(image, offset_x, offset_y, blur_radius, shadow_color)
            })
        }).map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        }).map_err(|e| e.into())
    }

    pub fn glow_impl(&mut self, blur_radius: f32, glow_color: (u8, u8, u8, u8), intensity: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                shadows::glow(image, blur_radius, glow_color, intensity)
            })
        }).map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        }).map_err(|e| e.into())
    }
}


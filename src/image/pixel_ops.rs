use pyo3::prelude::*;
use crate::pixels;
use super::core::{PyImage, LazyImage};

impl PyImage {
    pub fn getpixel_impl(&mut self, x: u32, y: u32) -> PyResult<(u8, u8, u8, u8)> {
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                pixels::get_pixel(image, x, y)
            })
        }).map_err(|e| e.into())
    }

    pub fn putpixel_impl(&mut self, x: u32, y: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                pixels::put_pixel(image, x, y, color)
            })
        }).map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        }).map_err(|e| e.into())
    }

    pub fn histogram_impl(&mut self) -> PyResult<(Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>)> {
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                pixels::histogram(image)
            })
        }).map(|(r, g, b, a)| (r.to_vec(), g.to_vec(), b.to_vec(), a.to_vec()))
        .map_err(|e| e.into())
    }

    pub fn dominant_color_impl(&mut self) -> PyResult<(u8, u8, u8, u8)> {
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                pixels::dominant_color(image)
            })
        }).map_err(|e| e.into())
    }

    pub fn average_color_impl(&mut self) -> PyResult<(u8, u8, u8, u8)> {
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                pixels::average_color(image)
            })
        }).map_err(|e| e.into())
    }

    pub fn replace_color_impl(&mut self, target_color: (u8, u8, u8, u8), replacement_color: (u8, u8, u8, u8), tolerance: u8) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                pixels::replace_color(image, target_color, replacement_color, tolerance)
            })
        }).map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        }).map_err(|e| e.into())
    }

    pub fn threshold_impl(&mut self, threshold_value: u8) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                pixels::threshold(image, threshold_value)
            })
        }).map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        }).map_err(|e| e.into())
    }

    pub fn posterize_impl(&mut self, levels: u8) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                pixels::posterize(image, levels)
            })
        }).map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        }).map_err(|e| e.into())
    }
}


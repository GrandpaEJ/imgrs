use pyo3::prelude::*;
use crate::{filters, css_filters};
use super::core::{PyImage, LazyImage};

impl PyImage {
    pub fn blur_impl(&mut self, radius: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                filters::blur(image, radius)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn sharpen_impl(&mut self, strength: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                filters::sharpen(image, strength)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn edge_detect_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                filters::edge_detect(image)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn emboss_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                filters::emboss(image)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn brightness_impl(&mut self, adjustment: i16) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                filters::brightness(image, adjustment)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn contrast_impl(&mut self, factor: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                filters::contrast(image, factor)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    // CSS-like filters
    pub fn sepia_impl(&mut self, amount: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                css_filters::sepia(image, amount)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn grayscale_filter_impl(&mut self, amount: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                css_filters::grayscale(image, amount)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn invert_impl(&mut self, amount: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                css_filters::invert(image, amount)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn hue_rotate_impl(&mut self, degrees: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                css_filters::hue_rotate(image, degrees)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn saturate_impl(&mut self, amount: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                css_filters::saturate(image, amount)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }
}


use pyo3::prelude::*;
use crate::errors::ImgrsError;
use crate::operations;
use super::core::{PyImage, LazyImage};

impl PyImage {
    pub fn resize_impl(&mut self, size: (u32, u32), resample: Option<String>) -> PyResult<Self> {
        let (width, height) = size;
        let format = self.format;
        
        // Load image to check dimensions
        let image = self.get_image()?;
        
        // Early return if size is the same
        if image.width() == width && image.height() == height {
            return Ok(PyImage {
                lazy_image: LazyImage::Loaded(image.clone()),
                format,
            });
        }
        
        let filter = operations::parse_resample_filter(resample.as_deref())?;
        
        Ok(Python::with_gil(|py| {
            py.allow_threads(|| {
                let resized = image.resize(width, height, filter);
                PyImage {
                    lazy_image: LazyImage::Loaded(resized),
                    format,
                }
            })
        }))
    }

    pub fn crop_impl(&mut self, box_coords: (u32, u32, u32, u32)) -> PyResult<Self> {
        let (x, y, width, height) = box_coords;
        let format = self.format;
        
        let image = self.get_image()?;
        
        // Validate crop bounds
        if x + width > image.width() || y + height > image.height() {
            return Err(ImgrsError::InvalidOperation(
                format!("Crop coordinates ({}+{}, {}+{}) exceed image bounds ({}x{})", 
                       x, width, y, height, image.width(), image.height())
            ).into());
        }
        
        if width == 0 || height == 0 {
            return Err(ImgrsError::InvalidOperation(
                "Crop dimensions must be greater than 0".to_string()
            ).into());
        }
        
        Ok(Python::with_gil(|py| {
            py.allow_threads(|| {
                let cropped = image.crop_imm(x, y, width, height);
                PyImage {
                    lazy_image: LazyImage::Loaded(cropped),
                    format,
                }
            })
        }))
    }

    pub fn rotate_impl(&mut self, angle: f64) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        
        Python::with_gil(|py| {
            py.allow_threads(|| {
                let rotated = if (angle - 90.0).abs() < f64::EPSILON {
                    image.rotate90()
                } else if (angle - 180.0).abs() < f64::EPSILON {
                    image.rotate180()
                } else if (angle - 270.0).abs() < f64::EPSILON {
                    image.rotate270()
                } else {
                    return Err(ImgrsError::InvalidOperation(
                        "Only 90, 180, 270 degree rotations supported".to_string()
                    ).into());
                };
                Ok(PyImage {
                    lazy_image: LazyImage::Loaded(rotated),
                    format,
                })
            })
        })
    }

    pub fn transpose_impl(&mut self, method: String) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        
        Python::with_gil(|py| {
            py.allow_threads(|| {
                let transposed = match method.as_str() {
                    "FLIP_LEFT_RIGHT" => image.fliph(),
                    "FLIP_TOP_BOTTOM" => image.flipv(),
                    "ROTATE_90" => image.rotate90(),
                    "ROTATE_180" => image.rotate180(),
                    "ROTATE_270" => image.rotate270(),
                    _ => return Err(ImgrsError::InvalidOperation(
                        format!("Unsupported transpose method: {}", method)
                    ).into()),
                };
                Ok(PyImage {
                    lazy_image: LazyImage::Loaded(transposed),
                    format,
                })
            })
        })
    }
}


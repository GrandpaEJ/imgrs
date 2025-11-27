use super::core::{LazyImage, PyImage};
use super::fast_resize::fast_resize;
use crate::errors::ImgrsError;
use crate::operations;
use image::DynamicImage;
use pyo3::prelude::*;

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

        let filter_str = resample.as_deref().unwrap_or("BILINEAR");

        Ok(Python::with_gil(|py| {
            py.allow_threads(|| {
                // Use fast SIMD resize for RGB/RGBA images
                let resized = match image {
                    DynamicImage::ImageRgb8(_) | DynamicImage::ImageRgba8(_) => {
                        fast_resize(image, width, height, filter_str).unwrap_or_else(|_| {
                            // Fallback to standard resize if fast resize fails
                            let filter =
                                operations::parse_resample_filter(Some(filter_str)).unwrap();
                            image.resize(width, height, filter)
                        })
                    }
                    _ => {
                        // Use standard resize for other formats
                        let filter = operations::parse_resample_filter(Some(filter_str)).unwrap();
                        image.resize(width, height, filter)
                    }
                };

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

        // Fast validation - single bounds check
        let img_width = image.width();
        let img_height = image.height();
        
        if width == 0 || height == 0 || x + width > img_width || y + height > img_height {
            return Err(ImgrsError::InvalidOperation(format!(
                "Invalid crop: ({},{}) {}x{} exceeds image bounds {}x{}",
                x, y, width, height, img_width, img_height
            ))
            .into());
        }

        // Optimized crop - release GIL for better performance
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

    pub fn rotate_impl(&mut self, angle: f64, expand: bool) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                // Normalize angle to 0-360 range
                let normalized_angle = angle.rem_euclid(360.0);
                
                // Fast paths for common rotations (with tolerance for floating point)
                let rotated = if normalized_angle.abs() < 0.01 {
                    // 0 degrees - no rotation needed
                    image.clone()
                } else if (normalized_angle - 90.0).abs() < 0.01 {
                    image.rotate90()
                } else if (normalized_angle - 180.0).abs() < 0.01 {
                    image.rotate180()
                } else if (normalized_angle - 270.0).abs() < 0.01 {
                    image.rotate270()
                } else {
                    use image::Rgba;
                    use imageproc::geometric_transformations::{
                        rotate_about_center, Interpolation,
                    };

                    let radians = normalized_angle.to_radians();

                    if !expand {
                        // Rotate in place (keep original dimensions)
                        let rgba_img = image.to_rgba8();
                        let rotated_rgba = rotate_about_center(
                            &rgba_img,
                            radians as f32,
                            Interpolation::Bilinear,
                            Rgba([0, 0, 0, 0]),
                        );
                        DynamicImage::ImageRgba8(rotated_rgba)
                    } else {
                        // Arbitrary angle rotation - expand to fit
                        let w = image.width() as f64;
                        let h = image.height() as f64;
                        let cos_a = radians.cos();
                        let sin_a = radians.sin();
                        
                        // Calculate new dimensions
                        let corners = [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)];
                        let mut min_x = f64::INFINITY;
                        let mut max_x = f64::NEG_INFINITY;
                        let mut min_y = f64::INFINITY;
                        let mut max_y = f64::NEG_INFINITY;
                        
                        for &(x, y) in &corners {
                            let rx = x * cos_a - y * sin_a;
                            let ry = x * sin_a + y * cos_a;
                            min_x = min_x.min(rx);
                            max_x = max_x.max(rx);
                            min_y = min_y.min(ry);
                            max_y = max_y.max(ry);
                        }
                        
                        let new_width = (max_x - min_x).ceil() as u32;
                        let new_height = (max_y - min_y).ceil() as u32;

                        // Create expanded canvas and rotate
                        let rgba_img = image.to_rgba8();
                        let mut large_rgba = image::RgbaImage::new(new_width, new_height);
                        let offset_x = ((new_width as f64 - w) / 2.0).round() as i64;
                        let offset_y = ((new_height as f64 - h) / 2.0).round() as i64;
                        image::imageops::overlay(&mut large_rgba, &rgba_img, offset_x, offset_y);

                        let rotated_rgba = rotate_about_center(
                            &large_rgba,
                            radians as f32,
                            Interpolation::Bilinear,
                            Rgba([0, 0, 0, 0]),
                        );

                        DynamicImage::ImageRgba8(rotated_rgba)
                    }
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
                    _ => {
                        return Err(ImgrsError::InvalidOperation(format!(
                            "Unsupported transpose method: {}",
                            method
                        ))
                        .into())
                    }
                };
                Ok(PyImage {
                    lazy_image: LazyImage::Loaded(transposed),
                    format,
                })
            })
        })
    }
}

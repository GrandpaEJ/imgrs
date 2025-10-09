use pyo3::prelude::*;
use image::{DynamicImage, Rgba, Rgb};
use crate::errors::ImgrsError;
use crate::filters::simd_ops::fast_rgb_to_gray;
use super::core::{PyImage, LazyImage, color_type_to_mode_string};

impl PyImage {
    pub fn copy_impl(&self) -> Self {
        PyImage {
            lazy_image: self.lazy_image.clone(),
            format: self.format,
        }
    }

    pub fn convert_impl(&mut self, mode: &str) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        // If already in target mode, return a copy
        let current_mode = color_type_to_mode_string(image.color());
        if current_mode == mode {
            return Ok(PyImage {
                lazy_image: LazyImage::Loaded(image.clone()),
                format,
            });
        }

        let converted = Python::with_gil(|py| {
            py.allow_threads(|| {
                match mode {
                    "L" => {
                        // Use SIMD-optimized grayscale conversion for RGB/RGBA
                        match image {
                            DynamicImage::ImageRgb8(_) | DynamicImage::ImageRgba8(_) => {
                                fast_rgb_to_gray(image)
                            }
                            _ => Ok(DynamicImage::ImageLuma8(image.to_luma8()))
                        }
                    }
                    "LA" => {
                        // Convert to grayscale with alpha
                        Ok(DynamicImage::ImageLumaA8(image.to_luma_alpha8()))
                    }
                    "RGB" => {
                        // Convert to RGB
                        Ok(DynamicImage::ImageRgb8(image.to_rgb8()))
                    }
                    "RGBA" => {
                        // Convert to RGBA
                        Ok(DynamicImage::ImageRgba8(image.to_rgba8()))
                    }
                    _ => Err(ImgrsError::InvalidOperation(
                        format!("Unsupported conversion mode: {}", mode)
                    )),
                }
            })
        })?;

        Ok(PyImage {
            lazy_image: LazyImage::Loaded(converted),
            format,
        })
    }

    pub fn split_impl(&mut self) -> PyResult<Vec<Self>> {
        let format = self.format;
        let image = self.get_image()?;

        let result = Python::with_gil(|py| {
            py.allow_threads(|| {
                match image {
                    DynamicImage::ImageRgb8(rgb_img) => {
                        let (width, height) = rgb_img.dimensions();
                        let mut channels = Vec::new();

                        // Extract R, G, B channels
                        for channel_idx in 0..3 {
                            let mut channel_data = Vec::with_capacity((width * height) as usize);
                            for pixel in rgb_img.pixels() {
                                channel_data.push(pixel.0[channel_idx]);
                            }

                            let channel_img = image::GrayImage::from_raw(width, height, channel_data)
                                .ok_or_else(|| ImgrsError::InvalidOperation(
                                    "Failed to create channel image".to_string()
                                ))?;

                            channels.push(PyImage {
                                lazy_image: LazyImage::Loaded(DynamicImage::ImageLuma8(channel_img)),
                                format,
                            });
                        }

                        Ok(channels)
                    }
                    DynamicImage::ImageRgba8(rgba_img) => {
                        let (width, height) = rgba_img.dimensions();
                        let mut channels = Vec::new();

                        // Extract R, G, B, A channels
                        for channel_idx in 0..4 {
                            let mut channel_data = Vec::with_capacity((width * height) as usize);
                            for pixel in rgba_img.pixels() {
                                channel_data.push(pixel.0[channel_idx]);
                            }

                            let channel_img = image::GrayImage::from_raw(width, height, channel_data)
                                .ok_or_else(|| ImgrsError::InvalidOperation(
                                    "Failed to create channel image".to_string()
                                ))?;

                            channels.push(PyImage {
                                lazy_image: LazyImage::Loaded(DynamicImage::ImageLuma8(channel_img)),
                                format,
                            });
                        }

                        Ok(channels)
                    }
                    DynamicImage::ImageLuma8(_) => {
                        // Grayscale image - return single channel
                        Ok(vec![PyImage {
                            lazy_image: LazyImage::Loaded(image.clone()),
                            format,
                        }])
                    }
                    DynamicImage::ImageLumaA8(la_img) => {
                        let (width, height) = la_img.dimensions();
                        let mut channels = Vec::new();

                        // Extract L, A channels
                        for channel_idx in 0..2 {
                            let mut channel_data = Vec::with_capacity((width * height) as usize);
                            for pixel in la_img.pixels() {
                                channel_data.push(pixel.0[channel_idx]);
                            }

                            let channel_img = image::GrayImage::from_raw(width, height, channel_data)
                                .ok_or_else(|| ImgrsError::InvalidOperation(
                                    "Failed to create channel image".to_string()
                                ))?;

                            channels.push(PyImage {
                                lazy_image: LazyImage::Loaded(DynamicImage::ImageLuma8(channel_img)),
                                format,
                            });
                        }

                        Ok(channels)
                    }
                    _ => Err(ImgrsError::InvalidOperation(
                        "Unsupported image format for channel splitting".to_string()
                    )),
                }
            })
        });
        result.map_err(|e| e.into())
    }

    pub fn paste_impl(&mut self, other: &mut Self, position: Option<(i32, i32)>, mask: Option<Self>) -> PyResult<Self> {
        let format = self.format;
        let base_image = self.get_image()?;
        let paste_image = other.get_image()?;

        let (paste_x, paste_y) = position.unwrap_or((0, 0));

        // Get mask image if provided
        let mask_image = if let Some(mut mask_img) = mask {
            Some(mask_img.get_image()?.clone())
        } else {
            None
        };

        Python::with_gil(|py| {
            py.allow_threads(|| {
                // Create a mutable copy of the base image
                let mut result = base_image.clone();

                match (&mut result, paste_image) {
                    (DynamicImage::ImageRgb8(base), DynamicImage::ImageRgb8(paste)) => {
                        let (base_width, base_height) = base.dimensions();
                        let (paste_width, paste_height) = paste.dimensions();

                        for y in 0..paste_height {
                            for x in 0..paste_width {
                                let target_x = paste_x + x as i32;
                                let target_y = paste_y + y as i32;

                                // Check bounds
                                if target_x >= 0 && target_y >= 0
                                    && (target_x as u32) < base_width
                                    && (target_y as u32) < base_height {

                                    let pixel = paste.get_pixel(x, y);

                                    // Apply mask if provided
                                    if let Some(ref mask) = mask_image {
                                        if let DynamicImage::ImageLuma8(mask_gray) = mask {
                                            let mask_pixel = mask_gray.get_pixel(x, y);
                                            let alpha = mask_pixel.0[0] as f32 / 255.0;

                                            if alpha > 0.0 {
                                                let base_pixel = base.get_pixel(target_x as u32, target_y as u32);
                                                let blended = Rgb([
                                                    ((1.0 - alpha) * base_pixel.0[0] as f32 + alpha * pixel.0[0] as f32) as u8,
                                                    ((1.0 - alpha) * base_pixel.0[1] as f32 + alpha * pixel.0[1] as f32) as u8,
                                                    ((1.0 - alpha) * base_pixel.0[2] as f32 + alpha * pixel.0[2] as f32) as u8,
                                                ]);
                                                base.put_pixel(target_x as u32, target_y as u32, blended);
                                            }
                                        }
                                    } else {
                                        base.put_pixel(target_x as u32, target_y as u32, *pixel);
                                    }
                                }
                            }
                        }
                    }
                    (DynamicImage::ImageRgba8(base), DynamicImage::ImageRgba8(paste)) => {
                        let (base_width, base_height) = base.dimensions();
                        let (paste_width, paste_height) = paste.dimensions();

                        for y in 0..paste_height {
                            for x in 0..paste_width {
                                let target_x = paste_x + x as i32;
                                let target_y = paste_y + y as i32;

                                // Check bounds
                                if target_x >= 0 && target_y >= 0
                                    && (target_x as u32) < base_width
                                    && (target_y as u32) < base_height {

                                    let pixel = paste.get_pixel(x, y);
                                    let alpha = pixel.0[3] as f32 / 255.0;

                                    if alpha > 0.0 {
                                        let base_pixel = base.get_pixel(target_x as u32, target_y as u32);
                                        let blended = Rgba([
                                            ((1.0 - alpha) * base_pixel.0[0] as f32 + alpha * pixel.0[0] as f32) as u8,
                                            ((1.0 - alpha) * base_pixel.0[1] as f32 + alpha * pixel.0[1] as f32) as u8,
                                            ((1.0 - alpha) * base_pixel.0[2] as f32 + alpha * pixel.0[2] as f32) as u8,
                                            255, // Keep base alpha
                                        ]);
                                        base.put_pixel(target_x as u32, target_y as u32, blended);
                                    }
                                }
                            }
                        }
                    }
                    // Convert images to compatible formats if needed
                    _ => {
                        let base_rgba = result.to_rgba8();
                        let paste_rgba = paste_image.to_rgba8();
                        let mut result_rgba = base_rgba;

                        let (base_width, base_height) = result_rgba.dimensions();
                        let (paste_width, paste_height) = paste_rgba.dimensions();

                        for y in 0..paste_height {
                            for x in 0..paste_width {
                                let target_x = paste_x + x as i32;
                                let target_y = paste_y + y as i32;

                                // Check bounds
                                if target_x >= 0 && target_y >= 0
                                    && (target_x as u32) < base_width
                                    && (target_y as u32) < base_height {

                                    let pixel = paste_rgba.get_pixel(x, y);
                                    let alpha = pixel.0[3] as f32 / 255.0;

                                    if alpha > 0.0 {
                                        let base_pixel = result_rgba.get_pixel(target_x as u32, target_y as u32);
                                        let blended = Rgba([
                                            ((1.0 - alpha) * base_pixel.0[0] as f32 + alpha * pixel.0[0] as f32) as u8,
                                            ((1.0 - alpha) * base_pixel.0[1] as f32 + alpha * pixel.0[1] as f32) as u8,
                                            ((1.0 - alpha) * base_pixel.0[2] as f32 + alpha * pixel.0[2] as f32) as u8,
                                            base_pixel.0[3], // Keep base alpha
                                        ]);
                                        result_rgba.put_pixel(target_x as u32, target_y as u32, blended);
                                    }
                                }
                            }
                        }

                        result = DynamicImage::ImageRgba8(result_rgba);
                    }
                }

                Ok(PyImage {
                    lazy_image: LazyImage::Loaded(result),
                    format,
                })
            })
        })
    }
}


use super::core::{color_type_to_mode_string, LazyImage, PyImage};
use crate::blending;
use crate::errors::ImgrsError;
use crate::filters::simd_ops::fast_rgb_to_gray;
use image::{DynamicImage, GenericImageView, Rgb, Rgba};
use pyo3::prelude::*;

/// Helper function to get alpha value from mask image
fn get_mask_alpha(mask: &DynamicImage, x: u32, y: u32) -> f32 {
    match mask {
        DynamicImage::ImageLuma8(gray) => gray.get_pixel(x, y).0[0] as f32 / 255.0,
        DynamicImage::ImageLumaA8(la) => la.get_pixel(x, y).0[1] as f32 / 255.0,
        DynamicImage::ImageRgb8(_) => {
            // For RGB masks, use luminance calculation
            let pixel = mask.get_pixel(x, y);
            let r = pixel.0[0] as f32;
            let g = pixel.0[1] as f32;
            let b = pixel.0[2] as f32;
            (0.299 * r + 0.587 * g + 0.114 * b) / 255.0
        }
        DynamicImage::ImageRgba8(rgba) => rgba.get_pixel(x, y).0[3] as f32 / 255.0,
        _ => {
            // For other formats, convert to grayscale and use that
            let gray = mask.to_luma8();
            gray.get_pixel(x, y).0[0] as f32 / 255.0
        }
    }
}

fn get_blend_func(mode: &str) -> PyResult<fn(blending::Pixel, blending::Pixel) -> blending::Pixel> {
    match mode {
        "clear" => Ok(blending::blend_clear),
        "source" => Ok(blending::blend_source),
        "over" => Ok(blending::blend_over),
        "in" => Ok(blending::blend_in),
        "out" => Ok(blending::blend_out),
        "atop" => Ok(blending::blend_atop),
        "dest" => Ok(blending::blend_dest),
        "dest_over" => Ok(blending::blend_dest_over),
        "dest_in" => Ok(blending::blend_dest_in),
        "dest_out" => Ok(blending::blend_dest_out),
        "dest_atop" => Ok(blending::blend_dest_atop),
        "xor" => Ok(blending::blend_xor),
        "add" => Ok(blending::blend_add),
        "saturate" => Ok(blending::blend_saturate),
        "multiply" => Ok(blending::blend_multiply),
        "screen" => Ok(blending::blend_screen),
        "overlay" => Ok(blending::blend_overlay),
        "darken" => Ok(blending::blend_darken),
        "lighten" => Ok(blending::blend_lighten),
        "color_dodge" => Ok(blending::blend_color_dodge),
        "color_burn" => Ok(blending::blend_color_burn),
        "hard_light" => Ok(blending::blend_hard_light),
        "soft_light" => Ok(blending::blend_soft_light),
        "difference" => Ok(blending::blend_difference),
        "exclusion" => Ok(blending::blend_exclusion),
        "hsl_hue" => Ok(blending::blend_hsl_hue),
        "hsl_saturation" => Ok(blending::blend_hsl_saturation),
        "hsl_color" => Ok(blending::blend_hsl_color),
        "hsl_luminosity" => Ok(blending::blend_hsl_luminosity),
        _ => Err(ImgrsError::InvalidOperation(format!("Unknown blend mode: {}", mode)).into()),
    }
}

fn blend_with_position_and_mask(
    base: &DynamicImage,
    overlay: &DynamicImage,
    blend_func: &fn(blending::Pixel, blending::Pixel) -> blending::Pixel,
    mask: Option<&DynamicImage>,
    x_offset: i32,
    y_offset: i32,
) -> Result<DynamicImage, ImgrsError> {
    let (base_width, base_height) = base.dimensions();
    let (overlay_width, overlay_height) = overlay.dimensions();

    let mut result = base.to_rgba8();
    let base_rgba = base.to_rgba8();
    let overlay_rgba = overlay.to_rgba8();

    for y in 0..overlay_height {
        for x in 0..overlay_width {
            let target_x = x_offset + x as i32;
            let target_y = y_offset + y as i32;

            // Check bounds
            if target_x >= 0 && target_y >= 0 &&
               (target_x as u32) < base_width && (target_y as u32) < base_height {

                let base_pixel = base_rgba.get_pixel(target_x as u32, target_y as u32);
                let overlay_pixel = overlay_rgba.get_pixel(x, y);

                // Apply mask if provided
                let mut alpha = overlay_pixel.0[3] as f32 / 255.0;
                if let Some(mask_img) = mask {
                    let mask_alpha = get_mask_alpha(mask_img, x, y);
                    alpha *= mask_alpha;
                }

                if alpha > 0.0 {
                    let blended = blend_func(
                        (base_pixel.0[0], base_pixel.0[1], base_pixel.0[2], base_pixel.0[3]),
                        (overlay_pixel.0[0], overlay_pixel.0[1], overlay_pixel.0[2], (alpha * 255.0) as u8)
                    );

                    result.put_pixel(target_x as u32, target_y as u32, Rgba([
                        blended.0, blended.1, blended.2, blended.3
                    ]));
                }
            }
        }
    }

    Ok(DynamicImage::ImageRgba8(result))
}

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
                            _ => Ok(DynamicImage::ImageLuma8(image.to_luma8())),
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
                    _ => Err(ImgrsError::InvalidOperation(format!(
                        "Unsupported conversion mode: {}",
                        mode
                    ))),
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

                            let channel_img =
                                image::GrayImage::from_raw(width, height, channel_data)
                                    .ok_or_else(|| {
                                        ImgrsError::InvalidOperation(
                                            "Failed to create channel image".to_string(),
                                        )
                                    })?;

                            channels.push(PyImage {
                                lazy_image: LazyImage::Loaded(DynamicImage::ImageLuma8(
                                    channel_img,
                                )),
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

                            let channel_img =
                                image::GrayImage::from_raw(width, height, channel_data)
                                    .ok_or_else(|| {
                                        ImgrsError::InvalidOperation(
                                            "Failed to create channel image".to_string(),
                                        )
                                    })?;

                            channels.push(PyImage {
                                lazy_image: LazyImage::Loaded(DynamicImage::ImageLuma8(
                                    channel_img,
                                )),
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

                            let channel_img =
                                image::GrayImage::from_raw(width, height, channel_data)
                                    .ok_or_else(|| {
                                        ImgrsError::InvalidOperation(
                                            "Failed to create channel image".to_string(),
                                        )
                                    })?;

                            channels.push(PyImage {
                                lazy_image: LazyImage::Loaded(DynamicImage::ImageLuma8(
                                    channel_img,
                                )),
                                format,
                            });
                        }

                        Ok(channels)
                    }
                    _ => Err(ImgrsError::InvalidOperation(
                        "Unsupported image format for channel splitting".to_string(),
                    )),
                }
            })
        });
        result.map_err(|e| e.into())
    }

    pub fn paste_impl(
        &mut self,
        other: &mut Self,
        position: Option<(i32, i32)>,
        mask: Option<Self>,
    ) -> PyResult<Self> {
        let format = self.format;
        let base_image = self.get_image()?;
        let paste_image = other.get_image()?;

        let (paste_x, paste_y) = position.unwrap_or((0, 0));

        // Enhanced mask handling - support both grayscale and RGBA masks
        let (mask_image, _mask_position) = if let Some(mut mask_img) = mask {
            let mask_rust_img = mask_img.get_image()?;
            let (mask_width, mask_height) = mask_rust_img.dimensions();

            // Validate mask size matches paste image size (Pillow behavior)
            let (paste_width, paste_height) = paste_image.dimensions();
            if mask_width != paste_width || mask_height != paste_height {
                return Err(ImgrsError::InvalidOperation(format!(
                    "Mask size {}x{} does not match paste image size {}x{}",
                    mask_width, mask_height, paste_width, paste_height
                ))
                .into());
            }

            (Some(mask_rust_img.clone()), (0, 0))
        } else {
            (None, (0, 0))
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
                                if target_x >= 0
                                    && target_y >= 0
                                    && (target_x as u32) < base_width
                                    && (target_y as u32) < base_height
                                {
                                    let pixel = paste.get_pixel(x, y);

                                    // Enhanced mask handling - support both L and RGBA masks
                                    if let Some(ref mask) = mask_image {
                                        let mask_alpha = get_mask_alpha(mask, x, y);

                                        if mask_alpha > 0.0 {
                                            let base_pixel =
                                                base.get_pixel(target_x as u32, target_y as u32);
                                            let blended = Rgb([
                                                ((1.0 - mask_alpha) * base_pixel.0[0] as f32
                                                    + mask_alpha * pixel.0[0] as f32)
                                                    as u8,
                                                ((1.0 - mask_alpha) * base_pixel.0[1] as f32
                                                    + mask_alpha * pixel.0[1] as f32)
                                                    as u8,
                                                ((1.0 - mask_alpha) * base_pixel.0[2] as f32
                                                    + mask_alpha * pixel.0[2] as f32)
                                                    as u8,
                                            ]);
                                            base.put_pixel(
                                                target_x as u32,
                                                target_y as u32,
                                                blended,
                                            );
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
                                if target_x >= 0
                                    && target_y >= 0
                                    && (target_x as u32) < base_width
                                    && (target_y as u32) < base_height
                                {
                                    let pixel = paste.get_pixel(x, y);
                                    let mut final_alpha = pixel.0[3] as f32 / 255.0;

                                    // Apply mask if provided (mask combines with alpha channel)
                                    if let Some(ref mask) = mask_image {
                                        let mask_alpha = get_mask_alpha(mask, x, y);
                                        final_alpha = (final_alpha * mask_alpha).min(1.0);
                                    }

                                    if final_alpha > 0.0 {
                                        let base_pixel =
                                            base.get_pixel(target_x as u32, target_y as u32);
                                        let blended = Rgba([
                                            ((1.0 - final_alpha) * base_pixel.0[0] as f32
                                                + final_alpha * pixel.0[0] as f32)
                                                as u8,
                                            ((1.0 - final_alpha) * base_pixel.0[1] as f32
                                                + final_alpha * pixel.0[1] as f32)
                                                as u8,
                                            ((1.0 - final_alpha) * base_pixel.0[2] as f32
                                                + final_alpha * pixel.0[2] as f32)
                                                as u8,
                                            base_pixel.0[3], // Keep base alpha for now
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
                                if target_x >= 0
                                    && target_y >= 0
                                    && (target_x as u32) < base_width
                                    && (target_y as u32) < base_height
                                {
                                    let pixel = paste_rgba.get_pixel(x, y);
                                    let mut final_alpha = pixel.0[3] as f32 / 255.0;

                                    // Apply mask if provided
                                    if let Some(ref mask) = mask_image {
                                        let mask_alpha = get_mask_alpha(mask, x, y);
                                        final_alpha = (final_alpha * mask_alpha).min(1.0);
                                    }

                                    if final_alpha > 0.0 {
                                        let base_pixel =
                                            result_rgba.get_pixel(target_x as u32, target_y as u32);
                                        let blended = Rgba([
                                            ((1.0 - final_alpha) * base_pixel.0[0] as f32
                                                + final_alpha * pixel.0[0] as f32)
                                                as u8,
                                            ((1.0 - final_alpha) * base_pixel.0[1] as f32
                                                + final_alpha * pixel.0[1] as f32)
                                                as u8,
                                            ((1.0 - final_alpha) * base_pixel.0[2] as f32
                                                + final_alpha * pixel.0[2] as f32)
                                                as u8,
                                            base_pixel.0[3], // Keep base alpha
                                        ]);
                                        result_rgba.put_pixel(
                                            target_x as u32,
                                            target_y as u32,
                                            blended,
                                        );
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

    pub fn composite_impl(&mut self, other: &mut Self, mode: &str) -> PyResult<Self> {
        let format = self.format;
        let dest_image = self.get_image()?;
        let source_image = other.get_image()?;

        let blend_func = get_blend_func(mode)?;

        let result = blending::composite_images(dest_image, source_image, blend_func)?;

        Ok(PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
    }

    pub fn blend_impl(&mut self, mode: &str, other: Option<&mut Self>, mask: Option<&mut Self>, position: Option<(i32, i32)>) -> PyResult<Self> {
        let format = self.format;
        let base_image = self.get_image()?;

        // If no other image provided, this is a no-op
        let other_image = match other {
            Some(img) => img.get_image()?,
            None => return Ok(PyImage {
                lazy_image: LazyImage::Loaded(base_image.clone()),
                format,
            }),
        };

        let blend_func = get_blend_func(mode)?;

        // Handle mask
        let mask_image = match mask {
            Some(mask_img) => Some(mask_img.get_image()?.clone()),
            None => None,
        };

        let (x_offset, y_offset) = position.unwrap_or((0, 0));

        let result = Python::with_gil(|py| {
            py.allow_threads(|| {
                blend_with_position_and_mask(base_image, other_image, &blend_func, mask_image.as_ref(), x_offset, y_offset)
            })
        })?;

        Ok(PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
    }

}

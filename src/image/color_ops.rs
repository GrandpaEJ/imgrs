// Enhanced color operations module
use crate::errors::ImgrsError;
use crate::filters::blur;
use crate::image::core::{LazyImage, PyImage};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

/// Color operations implementation
impl crate::image::core::PyImage {
    // Alpha channel operations
    pub fn set_alpha_impl(&mut self, alpha: f32) -> Result<Self, ImgrsError> {
        let image = self.get_image()?; // mut
        let target_alpha = (alpha.clamp(0.0, 1.0) * 255.0) as u8;

        // Convert to RGBA if not already
        let rgba_image = image.to_rgba8();
        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());

        for y in 0..rgba_image.height() {
            for x in 0..rgba_image.width() {
                let pixel = rgba_image.get_pixel(x, y);
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                let original_alpha = pixel[3];

                // Only modify alpha, keep RGB unchanged
                // If original pixel was fully transparent (alpha=0), keep it transparent
                // If original pixel had some transparency, scale it proportionally
                let new_alpha = if original_alpha == 0 {
                    0 // Keep fully transparent pixels transparent
                } else {
                    (original_alpha as f32 * target_alpha as f32 / 255.0).round() as u8
                };

                result.put_pixel(x, y, Rgba([r, g, b, new_alpha]));
            }
        }

        Ok(PyImage {
            lazy_image: crate::image::core::LazyImage::Loaded(DynamicImage::ImageRgba8(result)),
            format: self.format.clone(),
        })
    }

    pub fn get_alpha_impl(&mut self) -> f32 {
        if let Ok(image) = self.get_image() {
            match image {
                DynamicImage::ImageRgba8(rgba_img) => {
                    let mut total_alpha = 0.0;
                    let mut count = 0;
                    for y in 0..rgba_img.height() {
                        for x in 0..rgba_img.width() {
                            total_alpha += rgba_img.get_pixel(x, y)[3] as f32;
                            count += 1;
                        }
                    }
                    if count > 0 {
                        (total_alpha / count as f32) / 255.0
                    } else {
                        0.0
                    }
                }
                _ => 0.0,
            }
        } else {
            0.0
        }
    }

    pub fn add_transparency_impl(
        &mut self,
        color: (u8, u8, u8),
        tolerance: u8,
    ) -> Result<Self, ImgrsError> {
        let image = self.get_image()?; // mut
        let rgba_image = image.to_rgba8();
        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());

        for y in 0..rgba_image.height() {
            for x in 0..rgba_image.width() {
                let pixel = rgba_image.get_pixel(x, y);
                let distance = color_distance((pixel[0], pixel[1], pixel[2]), color);

                if distance <= tolerance as f32 {
                    result.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], 0]));
                } else {
                    result.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], pixel[3]]));
                }
            }
        }

        Ok(PyImage {
            lazy_image: LazyImage::Loaded(DynamicImage::ImageRgba8(result)),
            format: self.format.clone(),
        })
    }

    pub fn remove_transparency_impl(
        &mut self,
        background_color: Option<(u8, u8, u8)>,
    ) -> Result<Self, ImgrsError> {
        let image = self.get_image()?;
        let bg_color = background_color.unwrap_or((255, 255, 255));
        let rgba_image = image.to_rgba8();
        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());

        for y in 0..rgba_image.height() {
            for x in 0..rgba_image.width() {
                let pixel = rgba_image.get_pixel(x, y);
                let alpha = pixel[3] as f32 / 255.0;

                let final_r = (bg_color.0 as f32 * (1.0 - alpha) + pixel[0] as f32 * alpha) as u8;
                let final_g = (bg_color.1 as f32 * (1.0 - alpha) + pixel[1] as f32 * alpha) as u8;
                let final_b = (bg_color.2 as f32 * (1.0 - alpha) + pixel[2] as f32 * alpha) as u8;

                result.put_pixel(x, y, Rgba([final_r, final_g, final_b, 255]));
            }
        }

        Ok(PyImage {
            lazy_image: LazyImage::Loaded(DynamicImage::ImageRgba8(result)),
            format: self.format.clone(),
        })
    }

    // Advanced masking system
    pub fn apply_mask_impl(
        &mut self,
        mask: DynamicImage,
        invert: bool,
    ) -> Result<Self, ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let mask_rgba = mask.to_rgba8();

        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());

        for y in 0..rgba_image.height().min(mask_rgba.height()) {
            for x in 0..rgba_image.width().min(mask_rgba.width()) {
                let pixel = rgba_image.get_pixel(x, y);
                let mask_pixel = mask_rgba.get_pixel(x, y);

                let mask_alpha = if invert {
                    255 - mask_pixel[3]
                } else {
                    mask_pixel[3]
                };

                let final_alpha = (pixel[3] as f32 * mask_alpha as f32 / 255.0) as u8;
                result.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], final_alpha]));
            }
        }

        Ok(PyImage {
            lazy_image: LazyImage::Loaded(DynamicImage::ImageRgba8(result)),
            format: self.format.clone(),
        })
    }

    pub fn create_gradient_mask_impl(
        &mut self,
        direction: &str,
        start_opacity: f32,
        end_opacity: f32,
    ) -> Result<DynamicImage, ImgrsError> {
        let (width, height) = if let Ok(image) = self.get_image() {
            image.dimensions()
        } else {
            (100, 100) // Default size
        };

        let mut mask = ImageBuffer::new(width, height);
        let start_opacity = start_opacity.clamp(0.0, 1.0);
        let end_opacity = end_opacity.clamp(0.0, 1.0);

        match direction {
            "horizontal" => {
                for y in 0..height {
                    for x in 0..width {
                        let ratio = x as f32 / width as f32;
                        let alpha = (start_opacity + (end_opacity - start_opacity) * ratio) * 255.0;
                        mask.put_pixel(x, y, Rgba([255, 255, 255, alpha as u8]));
                    }
                }
            }
            "vertical" => {
                for y in 0..height {
                    for x in 0..width {
                        let ratio = y as f32 / height as f32;
                        let alpha = (start_opacity + (end_opacity - start_opacity) * ratio) * 255.0;
                        mask.put_pixel(x, y, Rgba([255, 255, 255, alpha as u8]));
                    }
                }
            }
            "radial" => {
                let center_x = width / 2;
                let center_y = height / 2;
                let max_distance = ((width * width + height * height) as f32).sqrt() / 2.0;

                for y in 0..height {
                    for x in 0..width {
                        let distance = ((x as f32 - center_x as f32).powi(2)
                            + (y as f32 - center_y as f32).powi(2))
                        .sqrt();
                        let ratio = (distance / max_distance).min(1.0);
                        let alpha = (start_opacity + (end_opacity - start_opacity) * ratio) * 255.0;
                        mask.put_pixel(x, y, Rgba([255, 255, 255, alpha as u8]));
                    }
                }
            }
            "diagonal" => {
                for y in 0..height {
                    for x in 0..width {
                        let ratio = (x + y) as f32 / (width + height) as f32;
                        let alpha = (start_opacity + (end_opacity - start_opacity) * ratio) * 255.0;
                        mask.put_pixel(x, y, Rgba([255, 255, 255, alpha as u8]));
                    }
                }
            }
            _ => {
                // Default to vertical
                for y in 0..height {
                    for x in 0..width {
                        let ratio = y as f32 / height as f32;
                        let alpha = (start_opacity + (end_opacity - start_opacity) * ratio) * 255.0;
                        mask.put_pixel(x, y, Rgba([255, 255, 255, alpha as u8]));
                    }
                }
            }
        }

        Ok(DynamicImage::ImageRgba8(mask))
    }

    pub fn create_color_mask_impl(
        &mut self,
        target_color: (u8, u8, u8),
        tolerance: u8,
        feather: u32,
    ) -> Result<DynamicImage, ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut mask = ImageBuffer::new(width, height);

        // First pass: create base mask
        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_image.get_pixel(x, y);
                let distance = color_distance((pixel[0], pixel[1], pixel[2]), target_color);

                let alpha = if distance <= tolerance as f32 { 255 } else { 0 };

                mask.put_pixel(x, y, Rgba([255, 255, 255, alpha]));
            }
        }

        // Apply feathering if specified
        if feather > 0 {
            let mask_dynamic = DynamicImage::ImageRgba8(mask);
            let blurred_mask = blur(&mask_dynamic, feather as f32)?;
            mask = blurred_mask.to_rgba8();
        }

        Ok(DynamicImage::ImageRgba8(mask))
    }

    pub fn create_luminance_mask_impl(&mut self, invert: bool) -> Result<DynamicImage, ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut mask = ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_image.get_pixel(x, y);
                let luminance = (pixel[0] as f32 * 0.299
                    + pixel[1] as f32 * 0.587
                    + pixel[2] as f32 * 0.114) as u8;

                let alpha = if invert { 255 - luminance } else { luminance };

                mask.put_pixel(x, y, Rgba([255, 255, 255, alpha]));
            }
        }

        Ok(DynamicImage::ImageRgba8(mask))
    }

    pub fn combine_masks_impl(
        &mut self,
        masks: Vec<DynamicImage>,
        operation: &str,
    ) -> Result<DynamicImage, ImgrsError> {
        if masks.is_empty() {
            return Err(ImgrsError::InvalidOperation(
                "No masks provided".to_string(),
            ));
        }

        let mut result = masks[0].to_rgba8();

        for mask in &masks[1..] {
            let mask_rgba = mask.to_rgba8();
            let (width, height) = result.dimensions();
            let mask_dims = mask_rgba.dimensions();
            let width = width.min(mask_dims.0);
            let height = height.min(mask_dims.1);

            match operation {
                "multiply" => {
                    for y in 0..height {
                        for x in 0..width {
                            let result_pixel = result.get_pixel(x, y);
                            let mask_pixel = mask_rgba.get_pixel(x, y);
                            let combined_alpha =
                                ((result_pixel[3] as f32 * mask_pixel[3] as f32) / 255.0) as u8;
                            result.put_pixel(x, y, Rgba([255, 255, 255, combined_alpha]));
                        }
                    }
                }
                "add" => {
                    for y in 0..height {
                        for x in 0..width {
                            let result_pixel = result.get_pixel(x, y);
                            let mask_pixel = mask_rgba.get_pixel(x, y);
                            let combined_alpha = result_pixel[3] + mask_pixel[3];
                            result.put_pixel(x, y, Rgba([255, 255, 255, combined_alpha]));
                        }
                    }
                }
                "subtract" => {
                    for y in 0..height {
                        for x in 0..width {
                            let result_pixel = result.get_pixel(x, y);
                            let mask_pixel = mask_rgba.get_pixel(x, y);
                            let combined_alpha =
                                (result_pixel[3] as i16 - mask_pixel[3] as i16).max(0) as u8;
                            result.put_pixel(x, y, Rgba([255, 255, 255, combined_alpha]));
                        }
                    }
                }
                "overlay" => {
                    for y in 0..height {
                        for x in 0..width {
                            let result_pixel = result.get_pixel(x, y);
                            let mask_pixel = mask_rgba.get_pixel(x, y);
                            let alpha1 = result_pixel[3] as f32 / 255.0;
                            let alpha2 = mask_pixel[3] as f32 / 255.0;
                            let combined_alpha = alpha1 + alpha2 * (1.0 - alpha1);
                            result.put_pixel(
                                x,
                                y,
                                Rgba([255, 255, 255, (combined_alpha * 255.0) as u8]),
                            );
                        }
                    }
                }
                _ => {
                    return Err(ImgrsError::InvalidOperation(format!(
                        "Unknown mask operation: {}",
                        operation
                    )));
                }
            }
        }

        Ok(DynamicImage::ImageRgba8(result))
    }

    // Enhanced color operations
    pub fn extract_color_impl(
        &mut self,
        target_color: (u8, u8, u8),
        tolerance: u8,
    ) -> Result<Self, ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());

        for y in 0..rgba_image.height() {
            for x in 0..rgba_image.width() {
                let pixel = rgba_image.get_pixel(x, y);
                let distance = color_distance((pixel[0], pixel[1], pixel[2]), target_color);

                if distance <= tolerance as f32 {
                    result.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], pixel[3]]));
                } else {
                    result.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                }
            }
        }

        Ok(PyImage {
            lazy_image: LazyImage::Loaded(DynamicImage::ImageRgba8(result)),
            format: self.format.clone(),
        })
    }

    pub fn color_quantize_impl(&mut self, levels: u8) -> Result<Self, ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());

        let step = 255.0 / (levels as f32 - 1.0);

        for y in 0..rgba_image.height() {
            for x in 0..rgba_image.width() {
                let pixel = rgba_image.get_pixel(x, y);

                let quantized_r = ((pixel[0] as f32 / step).round() * step) as u8;
                let quantized_g = ((pixel[1] as f32 / step).round() * step) as u8;
                let quantized_b = ((pixel[2] as f32 / step).round() * step) as u8;

                result.put_pixel(
                    x,
                    y,
                    Rgba([quantized_r, quantized_g, quantized_b, pixel[3]]),
                );
            }
        }

        Ok(PyImage {
            lazy_image: LazyImage::Loaded(DynamicImage::ImageRgba8(result)),
            format: self.format.clone(),
        })
    }

    pub fn color_shift_impl(&mut self, shift_amount: f32) -> Result<Self, ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());

        let shift = (shift_amount * 255.0) as i16;

        for y in 0..rgba_image.height() {
            for x in 0..rgba_image.width() {
                let pixel = rgba_image.get_pixel(x, y);

                let shifted_r = (pixel[0] as i16 + shift).clamp(0, 255) as u8;
                let shifted_g = (pixel[1] as i16 + shift).clamp(0, 255) as u8;
                let shifted_b = (pixel[2] as i16 + shift).clamp(0, 255) as u8;

                result.put_pixel(x, y, Rgba([shifted_r, shifted_g, shifted_b, pixel[3]]));
            }
        }

        Ok(PyImage {
            lazy_image: LazyImage::Loaded(DynamicImage::ImageRgba8(result)),
            format: self.format.clone(),
        })
    }

    pub fn selective_desaturate_impl(
        &mut self,
        target_color: (u8, u8, u8),
        tolerance: u8,
        desaturate_factor: f32,
    ) -> Result<Self, ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());

        for y in 0..rgba_image.height() {
            for x in 0..rgba_image.width() {
                let pixel = rgba_image.get_pixel(x, y);
                let distance = color_distance((pixel[0], pixel[1], pixel[2]), target_color);

                if distance <= tolerance as f32 {
                    // Desaturate this pixel
                    let gray = (pixel[0] as f32 * 0.299
                        + pixel[1] as f32 * 0.587
                        + pixel[2] as f32 * 0.114) as u8;

                    let final_r = (pixel[0] as f32 * (1.0 - desaturate_factor)
                        + gray as f32 * desaturate_factor) as u8;
                    let final_g = (pixel[1] as f32 * (1.0 - desaturate_factor)
                        + gray as f32 * desaturate_factor) as u8;
                    let final_b = (pixel[2] as f32 * (1.0 - desaturate_factor)
                        + gray as f32 * desaturate_factor) as u8;

                    result.put_pixel(x, y, Rgba([final_r, final_g, final_b, pixel[3]]));
                } else {
                    result.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], pixel[3]]));
                }
            }
        }

        Ok(PyImage {
            lazy_image: LazyImage::Loaded(DynamicImage::ImageRgba8(result)),
            format: self.format.clone(),
        })
    }

    pub fn color_match_impl(
        &mut self,
        reference_image: DynamicImage,
        strength: f32,
    ) -> Result<Self, ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let ref_rgba = reference_image.to_rgba8();

        let strength = strength.clamp(0.0, 1.0);
        let (width, height) = rgba_image.dimensions();
        let (ref_width, ref_height) = ref_rgba.dimensions();
        let width = width.min(ref_width);
        let height = height.min(ref_height);

        let mut result = ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_image.get_pixel(x, y);
                let ref_pixel = ref_rgba.get_pixel(x, y);

                // Simple color matching: blend towards reference color
                let matched_r =
                    (pixel[0] as f32 * (1.0 - strength) + ref_pixel[0] as f32 * strength) as u8;
                let matched_g =
                    (pixel[1] as f32 * (1.0 - strength) + ref_pixel[1] as f32 * strength) as u8;
                let matched_b =
                    (pixel[2] as f32 * (1.0 - strength) + ref_pixel[2] as f32 * strength) as u8;

                result.put_pixel(x, y, Rgba([matched_r, matched_g, matched_b, pixel[3]]));
            }
        }

        Ok(PyImage {
            lazy_image: LazyImage::Loaded(DynamicImage::ImageRgba8(result)),
            format: self.format.clone(),
        })
    }
}

// Utility functions
fn color_distance(color1: (u8, u8, u8), color2: (u8, u8, u8)) -> f32 {
    let dr = color1.0 as f32 - color2.0 as f32;
    let dg = color1.1 as f32 - color2.1 as f32;
    let db = color1.2 as f32 - color2.2 as f32;
    (dr * dr + dg * dg + db * db).sqrt()
}

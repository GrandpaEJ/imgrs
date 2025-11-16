// Advanced gradient and pattern operations
use image::{DynamicImage, ImageBuffer, Rgba, GenericImageView};
use crate::errors::ImgrsError;
use super::blending::composite::composite;
use super::blending::modes::BlendMode;

impl crate::image::core::PyImage {
    
    pub fn apply_gradient_overlay(&mut self, color: (u8, u8, u8, u8), direction: &str, opacity: f32) -> Result<Self, ImgrsError> {
        let gradient_mask = self.create_gradient_mask(direction, opacity, opacity)?;
        let mask = gradient_mask.to_rgba8();
        let image = self.get_image_mut()?;
        let rgba_image = image.to_rgba8();
        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());
        
        for y in 0..rgba_image.height() {
            for x in 0..rgba_image.width() {
                let pixel = rgba_image.get_pixel(x, y);
                let mask_pixel = mask.get_pixel(x, y);
                let mask_alpha = (mask_pixel[3] as f32 / 255.0) * (color.3 as f32 / 255.0);
                
                let blended_r = (pixel[0] as f32 * (1.0 - mask_alpha) + color.0 as f32 * mask_alpha) as u8;
                let blended_g = (pixel[1] as f32 * (1.0 - mask_alpha) + color.1 as f32 * mask_alpha) as u8;
                let blended_b = (pixel[2] as f32 * (1.0 - mask_alpha) + color.2 as f32 * mask_alpha) as u8;
                
                result.put_pixel(x, y, Rgba([blended_r, blended_g, blended_b, pixel[3]]));
            }
        }
        
        *image = DynamicImage::ImageRgba8(result);
        Ok(self.clone())
    }
    
    pub fn create_stripe_pattern(&mut self, color: (u8, u8, u8, u8), width: u32, spacing: u32, angle: f32) -> Result<DynamicImage, ImgrsError> {
        let (width, height) = if let Ok(image) = self.get_image() {
            image.dimensions()
        } else {
            (100, 100)
        };
        
        let mut pattern = ImageBuffer::new(width, height);
        let angle_rad = angle.to_radians();
        
        // Create stripes in direction perpendicular to angle
        for y in 0..height {
            for x in 0..width {
                let distance = ((x as f32 * angle_rad.cos() + y as f32 * angle_rad.sin()) as i32 % ((width + spacing) as i32)) as u32;
                
                let is_stripe = distance < width;
                let alpha = if is_stripe {
                    color.3
                } else {
                    0
                };
                
                pattern.put_pixel(x, y, Rgba([color.0, color.1, color.2, alpha]));
            }
        }
        
        Ok(DynamicImage::ImageRgba8(pattern))
    }
    
    pub fn create_checker_pattern(&mut self, color1: (u8, u8, u8, u8), color2: (u8, u8, u8, u8), size: u32) -> Result<DynamicImage, ImgrsError> {
        let (width, height) = if let Ok(image) = self.get_image() {
            image.dimensions()
        } else {
            (100, 100)
        };
        
        let mut pattern = ImageBuffer::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let check_x = (x / size) % 2;
                let check_y = (y / size) % 2;
                
                let use_color1 = (check_x + check_y) % 2 == 0;
                let (r, g, b, a) = if use_color1 { color1 } else { color2 };
                
                pattern.put_pixel(x, y, Rgba([r, g, b, a]));
            }
        }
        
        Ok(DynamicImage::ImageRgba8(pattern))
    }
    
    pub fn split_alpha(&mut self) -> Result<(DynamicImage, DynamicImage), ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        
        let mut rgb_image = ImageBuffer::new(width, height);
        let mut alpha_image = ImageBuffer::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_image.get_pixel(x, y);
                
                // RGB image (alpha set to 255)
                rgb_image.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], 255]));
                
                // Alpha image (grayscale from alpha channel)
                let gray = pixel[3];
                alpha_image.put_pixel(x, y, Rgba([gray, gray, gray, 255]));
            }
        }
        
        Ok((
            DynamicImage::ImageRgba8(rgb_image),
            DynamicImage::ImageRgba8(alpha_image)
        ))
    }
    
    pub fn merge_alpha(&mut self, alpha_image: DynamicImage) -> Result<Self, ImgrsError> {
        let mut image = self.get_image_mut()?;
        let rgba_image = image.to_rgba8();
        let alpha_rgba = alpha_image.to_rgba8();
        
        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());
        
        for y in 0..rgba_image.height().min(alpha_rgba.height()) {
            for x in 0..rgba_image.width().min(alpha_rgba.width()) {
                let pixel = rgba_image.get_pixel(x, y);
                let alpha_pixel = alpha_rgba.get_pixel(x, y);
                
                result.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], alpha_pixel[0]]));
            }
        }
        
        *image = DynamicImage::ImageRgba8(result);
        Ok(self.clone())
    }
    
    pub fn alpha_to_color(&mut self, background_color: (u8, u8, u8)) -> Result<Self, ImgrsError> {
        let mut image = self.get_image_mut()?;
        let rgba_image = image.to_rgba8();
        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());
        
        for y in 0..rgba_image.height() {
            for x in 0..rgba_image.width() {
                let pixel = rgba_image.get_pixel(x, y);
                let alpha = pixel[3] as f32 / 255.0;
                
                let final_r = (background_color.0 as f32 * (1.0 - alpha) + pixel[0] as f32 * alpha) as u8;
                let final_g = (background_color.1 as f32 * (1.0 - alpha) + pixel[1] as f32 * alpha) as u8;
                let final_b = (background_color.2 as f32 * (1.0 - alpha) + pixel[2] as f32 * alpha) as u8;
                
                result.put_pixel(x, y, Rgba([final_r, final_g, final_b, 255]));
            }
        }
        
        *image = DynamicImage::ImageRgba8(result);
        Ok(self.clone())
    }
    
    pub fn blend_with(&mut self, other_image: DynamicImage, mode: &str, opacity: f32) -> Result<Self, ImgrsError> {
        let blend_mode = match mode {
            "normal" => BlendMode::Normal,
            "multiply" => BlendMode::Multiply,
            "screen" => BlendMode::Screen,
            "overlay" => BlendMode::Overlay,
            "soft_light" => BlendMode::SoftLight,
            "hard_light" => BlendMode::HardLight,
            "color_dodge" => BlendMode::ColorDodge,
            "color_burn" => BlendMode::ColorBurn,
            "darken" => BlendMode::Darken,
            "lighten" => BlendMode::Lighten,
            "difference" => BlendMode::Difference,
            "exclusion" => BlendMode::Exclusion,
            _ => BlendMode::Normal,
        };
        
        let mut image = self.get_image_mut()?;
        let blended = composite(&image, &other_image, blend_mode, opacity)?;
        
        *image = blended;
        Ok(self.clone())
    }
    
    pub fn overlay_with(&mut self, overlay: DynamicImage, mode: &str, opacity: f32, position: Option<(i32, i32)>) -> Result<Self, ImgrsError> {
        let blend_mode = match mode {
            "normal" => BlendMode::Normal,
            "multiply" => BlendMode::Multiply,
            "screen" => BlendMode::Screen,
            "overlay" => BlendMode::Overlay,
            "soft_light" => BlendMode::SoftLight,
            "hard_light" => BlendMode::HardLight,
            "color_dodge" => BlendMode::ColorDodge,
            "color_burn" => BlendMode::ColorBurn,
            "darken" => BlendMode::Darken,
            "lighten" => BlendMode::Lighten,
            "difference" => BlendMode::Difference,
            "exclusion" => BlendMode::Exclusion,
            _ => BlendMode::Normal,
        };
        
        let mut image = self.get_image_mut()?;
        let rgba_image = image.to_rgba8();
        let overlay_rgba = overlay.to_rgba8();
        
        let pos = position.unwrap_or((
            (rgba_image.width() as i32 - overlay_rgba.width() as i32) / 2,
            (rgba_image.height() as i32 - overlay_rgba.height() as i32) / 2,
        ));
        
        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());
        
        // Copy base image
        for y in 0..rgba_image.height() {
            for x in 0..rgba_image.width() {
                result.put_pixel(x, y, rgba_image.get_pixel(x, y));
            }
        }
        
        // Apply overlay at position
        for y in 0..overlay_rgba.height() {
            for x in 0..overlay_rgba.width() {
                let target_x = x as i32 + pos.0;
                let target_y = y as i32 + pos.1;
                
                if target_x >= 0 && target_y >= 0 && 
                   target_x as u32 < rgba_image.width() && 
                   target_y as u32 < rgba_image.height() {
                    
                    let base_pixel = result.get_pixel(target_x as u32, target_y as u32);
                    let overlay_pixel = overlay_rgba.get_pixel(x, y);
                    
                    if overlay_pixel[3] > 0 {
                        let base_r = base_pixel[0] as f32 / 255.0;
                        let base_g = base_pixel[1] as f32 / 255.0;
                        let base_b = base_pixel[2] as f32 / 255.0;
                        
                        let overlay_r = overlay_pixel[0] as f32 / 255.0;
                        let overlay_g = overlay_pixel[1] as f32 / 255.0;
                        let overlay_b = overlay_pixel[2] as f32 / 255.0;
                        
                        let (blended_r, blended_g, blended_b) = match blend_mode {
                            BlendMode::Normal => (overlay_r, overlay_g, overlay_b),
                            BlendMode::Multiply => (base_r * overlay_r, base_g * overlay_g, base_b * overlay_b),
                            BlendMode::Screen => (1.0 - (1.0 - base_r) * (1.0 - overlay_r),
                                                1.0 - (1.0 - base_g) * (1.0 - overlay_g),
                                                1.0 - (1.0 - base_b) * (1.0 - overlay_b)),
                            _ => (overlay_r, overlay_g, overlay_b),
                        };
                        
                        let final_r = (base_r * (1.0 - opacity) + blended_r * opacity) * 255.0;
                        let final_g = (base_g * (1.0 - opacity) + blended_g * opacity) * 255.0;
                        let final_b = (base_b * (1.0 - opacity) + blended_b * opacity) * 255.0;
                        
                        result.put_pixel(target_x as u32, target_y as u32, 
                                       Rgba([final_r as u8, final_g as u8, final_b as u8, 255]));
                    }
                }
            }
        }
        
        *image = DynamicImage::ImageRgba8(result);
        Ok(self.clone())
    }
}
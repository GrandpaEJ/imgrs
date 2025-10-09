use image::{DynamicImage, ImageBuffer, Rgba, GenericImageView};
use crate::errors::ImgrsError;

/// Create a shadow mask from an image
pub fn create_shadow_mask(image: &DynamicImage, color: (u8, u8, u8, u8)) -> Result<DynamicImage, ImgrsError> {
    let (width, height) = image.dimensions();
    let mut mask = ImageBuffer::new(width, height);
    
    match image {
        DynamicImage::ImageRgba8(rgba_img) => {
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgba_img.get_pixel(x, y);
                    let alpha = pixel[3];
                    
                    // Use original alpha with shadow color
                    let shadow_alpha = ((alpha as f32 / 255.0) * (color.3 as f32 / 255.0) * 255.0) as u8;
                    mask.put_pixel(x, y, Rgba([color.0, color.1, color.2, shadow_alpha]));
                }
            }
        }
        _ => {
            // For non-RGBA images, create solid shadow
            for y in 0..height {
                for x in 0..width {
                    mask.put_pixel(x, y, Rgba([color.0, color.1, color.2, color.3]));
                }
            }
        }
    }
    
    Ok(DynamicImage::ImageRgba8(mask))
}

/// Create an inverted mask for inner shadows
pub fn create_inverted_mask(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    let (width, height) = image.dimensions();
    let mut mask = ImageBuffer::new(width, height);
    
    match image {
        DynamicImage::ImageRgba8(rgba_img) => {
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgba_img.get_pixel(x, y);
                    let alpha = 255 - pixel[3]; // Invert alpha
                    mask.put_pixel(x, y, Rgba([255, 255, 255, alpha]));
                }
            }
        }
        _ => {
            // For non-RGBA images, create empty mask
            for y in 0..height {
                for x in 0..width {
                    mask.put_pixel(x, y, Rgba([255, 255, 255, 0]));
                }
            }
        }
    }
    
    Ok(DynamicImage::ImageRgba8(mask))
}

/// Apply offset to an image
pub fn apply_offset(image: &DynamicImage, offset_x: i32, offset_y: i32) -> Result<DynamicImage, ImgrsError> {
    let (width, height) = image.dimensions();
    let mut result = ImageBuffer::new(width, height);
    
    match image {
        DynamicImage::ImageRgba8(rgba_img) => {
            // Fill with transparent pixels
            for y in 0..height {
                for x in 0..width {
                    result.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                }
            }
            
            // Copy pixels with offset
            for y in 0..height {
                for x in 0..width {
                    let src_x = x as i32 - offset_x;
                    let src_y = y as i32 - offset_y;
                    
                    if src_x >= 0 && src_y >= 0 && (src_x as u32) < width && (src_y as u32) < height {
                        let pixel = rgba_img.get_pixel(src_x as u32, src_y as u32);
                        result.put_pixel(x, y, *pixel);
                    }
                }
            }
        }
        _ => {
            return Err(ImgrsError::InvalidOperation(
                "Offset only supported for RGBA images".to_string()
            ));
        }
    }
    
    Ok(DynamicImage::ImageRgba8(result))
}

/// Apply intensity to an image
pub fn apply_intensity(image: &DynamicImage, intensity: f32) -> Result<DynamicImage, ImgrsError> {
    let (width, height) = image.dimensions();
    let mut result = ImageBuffer::new(width, height);
    
    match image {
        DynamicImage::ImageRgba8(rgba_img) => {
            for y in 0..height {
                for x in 0..width {
                    let pixel = rgba_img.get_pixel(x, y);
                    let new_alpha = (pixel[3] as f32 * intensity).min(255.0) as u8;
                    result.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], new_alpha]));
                }
            }
        }
        _ => {
            return Err(ImgrsError::InvalidOperation(
                "Intensity only supported for RGBA images".to_string()
            ));
        }
    }
    
    Ok(DynamicImage::ImageRgba8(result))
}

/// Simple image pasting function
pub fn paste_image(base: &DynamicImage, overlay: &DynamicImage, x: i32, y: i32) -> Result<DynamicImage, ImgrsError> {
    let mut result = base.clone();
    let (overlay_width, overlay_height) = overlay.dimensions();
    
    match (&mut result, overlay) {
        (DynamicImage::ImageRgba8(base_img), DynamicImage::ImageRgba8(overlay_img)) => {
            let (base_width, base_height) = base_img.dimensions();
            
            for oy in 0..overlay_height {
                for ox in 0..overlay_width {
                    let target_x = x + ox as i32;
                    let target_y = y + oy as i32;
                    
                    if target_x >= 0 && target_y >= 0 
                        && (target_x as u32) < base_width 
                        && (target_y as u32) < base_height {
                        
                        let overlay_pixel = overlay_img.get_pixel(ox, oy);
                        let alpha = overlay_pixel[3] as f32 / 255.0;
                        
                        if alpha > 0.0 {
                            let base_pixel = base_img.get_pixel(target_x as u32, target_y as u32);
                            let blended = Rgba([
                                ((1.0 - alpha) * base_pixel[0] as f32 + alpha * overlay_pixel[0] as f32) as u8,
                                ((1.0 - alpha) * base_pixel[1] as f32 + alpha * overlay_pixel[1] as f32) as u8,
                                ((1.0 - alpha) * base_pixel[2] as f32 + alpha * overlay_pixel[2] as f32) as u8,
                                ((1.0 - alpha) * base_pixel[3] as f32 + alpha * 255.0) as u8,
                            ]);
                            base_img.put_pixel(target_x as u32, target_y as u32, blended);
                        }
                    }
                }
            }
        }
        _ => {
            return Err(ImgrsError::InvalidOperation(
                "Pasting only supported for RGBA images".to_string()
            ));
        }
    }
    
    Ok(result)
}

/// Multiply blend mode
pub fn multiply_blend(base: &DynamicImage, overlay: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    let mut result = base.clone();
    let (width, height) = base.dimensions();
    
    match (&mut result, overlay) {
        (DynamicImage::ImageRgba8(base_img), DynamicImage::ImageRgba8(overlay_img)) => {
            for y in 0..height {
                for x in 0..width {
                    let base_pixel = base_img.get_pixel(x, y);
                    let overlay_pixel = overlay_img.get_pixel(x, y);
                    
                    let blended = Rgba([
                        ((base_pixel[0] as f32 * overlay_pixel[0] as f32) / 255.0) as u8,
                        ((base_pixel[1] as f32 * overlay_pixel[1] as f32) / 255.0) as u8,
                        ((base_pixel[2] as f32 * overlay_pixel[2] as f32) / 255.0) as u8,
                        base_pixel[3],
                    ]);
                    
                    base_img.put_pixel(x, y, blended);
                }
            }
        }
        _ => {
            return Err(ImgrsError::InvalidOperation(
                "Multiply blend only supported for RGBA images".to_string()
            ));
        }
    }
    
    Ok(result)
}


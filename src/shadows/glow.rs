use image::{DynamicImage, GenericImageView};
use crate::errors::ImgrsError;
use crate::filters::blur;
use super::utils::{create_shadow_mask, apply_intensity, paste_image};

/// Apply glow effect to an image
pub fn glow(
    image: &DynamicImage,
    blur_radius: f32,
    glow_color: (u8, u8, u8, u8),
    intensity: f32,
) -> Result<DynamicImage, ImgrsError> {
    let (width, height) = image.dimensions();
    
    // Calculate expanded canvas size
    let glow_padding = (blur_radius * 2.0) as u32;
    let new_width = width + glow_padding * 2;
    let new_height = height + glow_padding * 2;
    
    // Create expanded canvas
    let mut canvas = DynamicImage::new_rgba8(new_width, new_height);
    
    // Create glow mask
    let glow_mask = create_shadow_mask(image, glow_color)?;
    
    // Blur the glow
    let blurred_glow = if blur_radius > 0.0 {
        blur(&glow_mask, blur_radius)?
    } else {
        glow_mask
    };
    
    // Apply intensity to glow
    let intense_glow = apply_intensity(&blurred_glow, intensity)?;
    
    // Position glow on canvas
    let glow_x = glow_padding as i32;
    let glow_y = glow_padding as i32;
    
    // Paste glow onto canvas
    canvas = paste_image(&canvas, &intense_glow, glow_x, glow_y)?;
    
    // Paste original image on top
    canvas = paste_image(&canvas, image, glow_x, glow_y)?;
    
    Ok(canvas)
}


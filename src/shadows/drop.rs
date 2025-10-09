use image::{DynamicImage, GenericImageView};
use crate::errors::ImgrsError;
use crate::filters::blur;
use super::utils::{create_shadow_mask, paste_image};

/// Apply drop shadow effect to an image
pub fn drop_shadow(
    image: &DynamicImage,
    offset_x: i32,
    offset_y: i32,
    blur_radius: f32,
    shadow_color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let (width, height) = image.dimensions();
    
    // Calculate expanded canvas size to accommodate shadow
    let shadow_padding = (blur_radius * 2.0) as u32 + offset_x.unsigned_abs() + offset_y.unsigned_abs();
    let new_width = width + shadow_padding * 2;
    let new_height = height + shadow_padding * 2;
    
    // Create expanded canvas
    let mut canvas = DynamicImage::new_rgba8(new_width, new_height);
    
    // Create shadow mask from the original image alpha channel
    let shadow_mask = create_shadow_mask(image, shadow_color)?;
    
    // Blur the shadow mask
    let blurred_shadow = if blur_radius > 0.0 {
        blur(&shadow_mask, blur_radius)?
    } else {
        shadow_mask
    };
    
    // Position shadow on canvas
    let shadow_x = shadow_padding as i32 + offset_x;
    let shadow_y = shadow_padding as i32 + offset_y;
    
    // Paste shadow onto canvas
    canvas = paste_image(&canvas, &blurred_shadow, shadow_x, shadow_y)?;
    
    // Paste original image on top
    let image_x = shadow_padding as i32;
    let image_y = shadow_padding as i32;
    canvas = paste_image(&canvas, image, image_x, image_y)?;
    
    Ok(canvas)
}


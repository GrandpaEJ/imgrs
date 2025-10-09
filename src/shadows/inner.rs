use image::DynamicImage;
use crate::errors::ImgrsError;
use crate::filters::blur;
use super::utils::{create_inverted_mask, create_shadow_mask, apply_offset, multiply_blend};

/// Apply inner shadow effect to an image
pub fn inner_shadow(
    image: &DynamicImage,
    offset_x: i32,
    offset_y: i32,
    blur_radius: f32,
    shadow_color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let mut result = image.clone();
    
    // Create inverted mask for inner shadow
    let inverted_mask = create_inverted_mask(image)?;
    
    // Create shadow from inverted mask
    let shadow_mask = create_shadow_mask(&inverted_mask, shadow_color)?;
    
    // Blur the shadow
    let blurred_shadow = if blur_radius > 0.0 {
        blur(&shadow_mask, blur_radius)?
    } else {
        shadow_mask
    };
    
    // Apply offset to shadow
    let offset_shadow = apply_offset(&blurred_shadow, offset_x, offset_y)?;
    
    // Composite shadow with original image using multiply blend mode
    result = multiply_blend(&result, &offset_shadow)?;
    
    Ok(result)
}


use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use crate::errors::ImgrsError;
use super::blur::blur;
use super::kernel::apply_convolution;

/// Apply unsharp mask sharpening
pub fn unsharp_mask(image: &DynamicImage, radius: f32, amount: f32, threshold: u8) -> Result<DynamicImage, ImgrsError> {
    // Create blurred version
    let blurred = blur(image, radius)?;
    
    match (image, &blurred) {
        (DynamicImage::ImageRgb8(original), DynamicImage::ImageRgb8(blur_img)) => {
            let (width, height) = original.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let orig_pixel = original.get_pixel(x, y);
                    let blur_pixel = blur_img.get_pixel(x, y);
                    
                    let mut output = [0u8; 3];
                    for i in 0..3 {
                        let diff = orig_pixel[i] as i16 - blur_pixel[i] as i16;
                        
                        // Only sharpen if difference exceeds threshold
                        if diff.abs() > threshold as i16 {
                            let sharpened = orig_pixel[i] as f32 + diff as f32 * amount;
                            output[i] = sharpened.max(0.0).min(255.0) as u8;
                        } else {
                            output[i] = orig_pixel[i];
                        }
                    }
                    
                    result.put_pixel(x, y, Rgb(output));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        (DynamicImage::ImageRgba8(original), DynamicImage::ImageRgba8(blur_img)) => {
            let (width, height) = original.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let orig_pixel = original.get_pixel(x, y);
                    let blur_pixel = blur_img.get_pixel(x, y);
                    
                    let mut output = [0u8; 4];
                    for i in 0..3 {
                        let diff = orig_pixel[i] as i16 - blur_pixel[i] as i16;
                        
                        if diff.abs() > threshold as i16 {
                            let sharpened = orig_pixel[i] as f32 + diff as f32 * amount;
                            output[i] = sharpened.max(0.0).min(255.0) as u8;
                        } else {
                            output[i] = orig_pixel[i];
                        }
                    }
                    output[3] = orig_pixel[3]; // Preserve alpha
                    
                    result.put_pixel(x, y, Rgba(output));
                }
            }
            
            Ok(DynamicImage::ImageRgba8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            unsharp_mask(&rgb_dynamic, radius, amount, threshold)
        }
    }
}

/// Apply high-pass filter
pub fn high_pass(image: &DynamicImage, radius: f32) -> Result<DynamicImage, ImgrsError> {
    let blurred = blur(image, radius)?;
    
    match (image, &blurred) {
        (DynamicImage::ImageRgb8(original), DynamicImage::ImageRgb8(blur_img)) => {
            let (width, height) = original.dimensions();
            let mut result = ImageBuffer::new(width, height);
            
            for y in 0..height {
                for x in 0..width {
                    let orig_pixel = original.get_pixel(x, y);
                    let blur_pixel = blur_img.get_pixel(x, y);
                    
                    let r = ((orig_pixel[0] as i16 - blur_pixel[0] as i16) + 128).max(0).min(255) as u8;
                    let g = ((orig_pixel[1] as i16 - blur_pixel[1] as i16) + 128).max(0).min(255) as u8;
                    let b = ((orig_pixel[2] as i16 - blur_pixel[2] as i16) + 128).max(0).min(255) as u8;
                    
                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }
            
            Ok(DynamicImage::ImageRgb8(result))
        }
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            high_pass(&rgb_dynamic, radius)
        }
    }
}

/// Apply edge enhancement
pub fn edge_enhance(image: &DynamicImage, strength: f32) -> Result<DynamicImage, ImgrsError> {
    let kernel = vec![
        vec![0.0, -strength, 0.0],
        vec![-strength, 1.0 + 4.0 * strength, -strength],
        vec![0.0, -strength, 0.0],
    ];
    
    apply_convolution(image, &kernel)
}

/// Apply edge enhancement (more aggressive)
pub fn edge_enhance_more(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    let kernel = vec![
        vec![-1.0, -1.0, -1.0],
        vec![-1.0, 9.0, -1.0],
        vec![-1.0, -1.0, -1.0],
    ];
    
    apply_convolution(image, &kernel)
}


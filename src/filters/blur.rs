use crate::errors::ImgrsError;
use image::DynamicImage;

/// Apply Gaussian blur to an image using optimized algorithm
pub fn blur(image: &DynamicImage, radius: f32) -> Result<DynamicImage, ImgrsError> {
    if radius <= 0.0 {
        return Ok(image.clone());
    }

    // Use imageproc's optimized Gaussian blur
    // It uses separable filters (horizontal + vertical passes) which is much faster
    let sigma = radius / 2.0; // Convert radius to sigma
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let blurred = imageproc::filter::gaussian_blur_f32(rgb_img, sigma);
            Ok(DynamicImage::ImageRgb8(blurred))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            // Split RGBA into RGB + Alpha, blur RGB, recombine
            let (width, height) = rgba_img.dimensions();
            let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);
            let mut alpha_data = Vec::with_capacity((width * height) as usize);
            
            for pixel in rgba_img.pixels() {
                rgb_data.push(pixel[0]);
                rgb_data.push(pixel[1]);
                rgb_data.push(pixel[2]);
                alpha_data.push(pixel[3]);
            }
            
            let rgb_img = image::RgbImage::from_raw(width, height, rgb_data)
                .ok_or_else(|| ImgrsError::InvalidOperation("Failed to create RGB image".to_string()))?;
            let blurred_rgb = imageproc::filter::gaussian_blur_f32(&rgb_img, sigma);
            
            // Recombine with alpha
            let mut result = image::RgbaImage::new(width, height);
            for (i, pixel) in blurred_rgb.pixels().enumerate() {
                let x = (i as u32) % width;
                let y = (i as u32) / width;
                result.put_pixel(x, y, image::Rgba([pixel[0], pixel[1], pixel[2], alpha_data[i]]));
            }
            
            Ok(DynamicImage::ImageRgba8(result))
        }
        DynamicImage::ImageLuma8(gray_img) => {
            let blurred = imageproc::filter::gaussian_blur_f32(gray_img, sigma);
            Ok(DynamicImage::ImageLuma8(blurred))
        }
        _ => {
            // Convert to RGB, blur, and return
            let rgb_img = image.to_rgb8();
            let blurred = imageproc::filter::gaussian_blur_f32(&rgb_img, sigma);
            Ok(DynamicImage::ImageRgb8(blurred))
        }
    }
}

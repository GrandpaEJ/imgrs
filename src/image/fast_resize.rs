use image::DynamicImage;
use fast_image_resize as fr;
use crate::errors::ImgrsError;

/// Fast resize using SIMD-optimized fast_image_resize crate
pub fn fast_resize(image: &DynamicImage, width: u32, height: u32, filter: &str) -> Result<DynamicImage, ImgrsError> {
    use fr::images::Image as FrImage;
    
    // Convert filter string to fast_image_resize filter
    let _algorithm = match filter {
        "NEAREST" => fr::ResizeAlg::Nearest,
        "BILINEAR" => fr::ResizeAlg::Convolution(fr::FilterType::Bilinear),
        "BICUBIC" => fr::ResizeAlg::Convolution(fr::FilterType::CatmullRom),
        "LANCZOS" => fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3),
        _ => fr::ResizeAlg::Convolution(fr::FilterType::Bilinear),
    };
    
    // Note: Currently using default algorithm in resizer
    // TODO: Pass algorithm to resizer.resize() for custom filters
    
    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let src_width = rgb_img.width();
            let src_height = rgb_img.height();
            
            // Create source image
            let src_image = FrImage::from_vec_u8(
                src_width,
                src_height,
                rgb_img.as_raw().to_vec(),
                fr::PixelType::U8x3,
            ).map_err(|e| ImgrsError::InvalidOperation(format!("Fast resize error: {}", e)))?;
            
            // Create destination image
            let mut dst_image = FrImage::new(
                width,
                height,
                fr::PixelType::U8x3,
            );
            
            // Resize with SIMD
            let mut resizer = fr::Resizer::new();
            resizer.resize(&src_image, &mut dst_image, None)
                .map_err(|e| ImgrsError::InvalidOperation(format!("Resize failed: {}", e)))?;
            
            // Convert back to DynamicImage
            let result_buffer = image::RgbImage::from_raw(
                width,
                height,
                dst_image.into_vec(),
            ).ok_or_else(|| ImgrsError::InvalidOperation("Failed to create result image".to_string()))?;
            
            Ok(DynamicImage::ImageRgb8(result_buffer))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let src_width = rgba_img.width();
            let src_height = rgba_img.height();
            
            // Create source image
            let src_image = FrImage::from_vec_u8(
                src_width,
                src_height,
                rgba_img.as_raw().to_vec(),
                fr::PixelType::U8x4,
            ).map_err(|e| ImgrsError::InvalidOperation(format!("Fast resize error: {}", e)))?;
            
            // Create destination image
            let mut dst_image = FrImage::new(
                width,
                height,
                fr::PixelType::U8x4,
            );
            
            // Resize with SIMD
            let mut resizer = fr::Resizer::new();
            resizer.resize(&src_image, &mut dst_image, None)
                .map_err(|e| ImgrsError::InvalidOperation(format!("Resize failed: {}", e)))?;
            
            // Convert back to DynamicImage
            let result_buffer = image::RgbaImage::from_raw(
                width,
                height,
                dst_image.into_vec(),
            ).ok_or_else(|| ImgrsError::InvalidOperation("Failed to create result image".to_string()))?;
            
            Ok(DynamicImage::ImageRgba8(result_buffer))
        }
        // For other formats, convert to RGB first
        _ => {
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            fast_resize(&rgb_dynamic, width, height, filter)
        }
    }
}


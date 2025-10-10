use image::{DynamicImage, ImageBuffer, Luma};
use crate::errors::ImgrsError;

/// Apply morphological dilation
pub fn dilate(image: &DynamicImage, radius: u32) -> Result<DynamicImage, ImgrsError> {
    let gray_img = image.to_luma8();
    let (width, height) = gray_img.dimensions();
    let mut result = ImageBuffer::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let mut max_val = 0u8;
            
            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    if dx * dx + dy * dy <= (radius * radius) as i32 {
                        let nx = (x as i32 + dx).max(0).min(width as i32 - 1) as u32;
                        let ny = (y as i32 + dy).max(0).min(height as i32 - 1) as u32;
                        let val = gray_img.get_pixel(nx, ny)[0];
                        max_val = max_val.max(val);
                    }
                }
            }
            
            result.put_pixel(x, y, Luma([max_val]));
        }
    }
    
    Ok(DynamicImage::ImageLuma8(result))
}

/// Apply morphological erosion
pub fn erode(image: &DynamicImage, radius: u32) -> Result<DynamicImage, ImgrsError> {
    let gray_img = image.to_luma8();
    let (width, height) = gray_img.dimensions();
    let mut result = ImageBuffer::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let mut min_val = 255u8;
            
            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    if dx * dx + dy * dy <= (radius * radius) as i32 {
                        let nx = (x as i32 + dx).max(0).min(width as i32 - 1) as u32;
                        let ny = (y as i32 + dy).max(0).min(height as i32 - 1) as u32;
                        let val = gray_img.get_pixel(nx, ny)[0];
                        min_val = min_val.min(val);
                    }
                }
            }
            
            result.put_pixel(x, y, Luma([min_val]));
        }
    }
    
    Ok(DynamicImage::ImageLuma8(result))
}

/// Apply morphological opening (erosion followed by dilation)
pub fn opening(image: &DynamicImage, radius: u32) -> Result<DynamicImage, ImgrsError> {
    let eroded = erode(image, radius)?;
    dilate(&eroded, radius)
}

/// Apply morphological closing (dilation followed by erosion)
pub fn closing(image: &DynamicImage, radius: u32) -> Result<DynamicImage, ImgrsError> {
    let dilated = dilate(image, radius)?;
    erode(&dilated, radius)
}

/// Apply morphological gradient (dilation - erosion)
pub fn morphological_gradient(image: &DynamicImage, radius: u32) -> Result<DynamicImage, ImgrsError> {
    let dilated = dilate(image, radius)?;
    let eroded = erode(image, radius)?;
    
    if let (DynamicImage::ImageLuma8(dil_img), DynamicImage::ImageLuma8(ero_img)) = (&dilated, &eroded) {
        let (width, height) = dil_img.dimensions();
        let mut result = ImageBuffer::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let dil_val = dil_img.get_pixel(x, y)[0] as i16;
                let ero_val = ero_img.get_pixel(x, y)[0] as i16;
                let diff = (dil_val - ero_val).max(0).min(255) as u8;
                result.put_pixel(x, y, Luma([diff]));
            }
        }
        
        Ok(DynamicImage::ImageLuma8(result))
    } else {
        Err(ImgrsError::InvalidOperation("Morphological gradient failed".to_string()))
    }
}

/// Apply top hat transform (original - opening)
    #[allow(dead_code)]
pub fn top_hat(image: &DynamicImage, radius: u32) -> Result<DynamicImage, ImgrsError> {
    let opened = opening(image, radius)?;
    let gray_img = image.to_luma8();
    
    if let DynamicImage::ImageLuma8(open_img) = &opened {
        let (width, height) = gray_img.dimensions();
        let mut result = ImageBuffer::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let orig_val = gray_img.get_pixel(x, y)[0] as i16;
                let open_val = open_img.get_pixel(x, y)[0] as i16;
                let diff = (orig_val - open_val).max(0).min(255) as u8;
                result.put_pixel(x, y, Luma([diff]));
            }
        }
        
        Ok(DynamicImage::ImageLuma8(result))
    } else {
        Err(ImgrsError::InvalidOperation("Top hat transform failed".to_string()))
    }
}

/// Apply black hat transform (closing - original)
    #[allow(dead_code)]
pub fn black_hat(image: &DynamicImage, radius: u32) -> Result<DynamicImage, ImgrsError> {
    let closed = closing(image, radius)?;
    let gray_img = image.to_luma8();
    
    if let DynamicImage::ImageLuma8(close_img) = &closed {
        let (width, height) = gray_img.dimensions();
        let mut result = ImageBuffer::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let close_val = close_img.get_pixel(x, y)[0] as i16;
                let orig_val = gray_img.get_pixel(x, y)[0] as i16;
                let diff = (close_val - orig_val).max(0).min(255) as u8;
                result.put_pixel(x, y, Luma([diff]));
            }
        }
        
        Ok(DynamicImage::ImageLuma8(result))
    } else {
        Err(ImgrsError::InvalidOperation("Black hat transform failed".to_string()))
    }
}


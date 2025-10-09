use image::{DynamicImage, ImageBuffer, Luma};
use crate::errors::ImgrsError;
use super::kernel::apply_convolution;

/// Apply edge detection filter (Sobel operator)
pub fn edge_detect(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    // Convert to grayscale first for edge detection
    let gray_img = image.to_luma8();
    let gray_dynamic = DynamicImage::ImageLuma8(gray_img);

    // Sobel X kernel
    let sobel_x = vec![
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 0.0, 2.0],
        vec![-1.0, 0.0, 1.0],
    ];

    // Sobel Y kernel
    let sobel_y = vec![
        vec![-1.0, -2.0, -1.0],
        vec![0.0, 0.0, 0.0],
        vec![1.0, 2.0, 1.0],
    ];

    let edge_x = apply_convolution(&gray_dynamic, &sobel_x)?;
    let edge_y = apply_convolution(&gray_dynamic, &sobel_y)?;

    // Combine X and Y gradients
    if let (DynamicImage::ImageLuma8(x_img), DynamicImage::ImageLuma8(y_img)) = (&edge_x, &edge_y) {
        let (width, height) = x_img.dimensions();
        let mut result = ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let x_val = x_img.get_pixel(x, y)[0] as f32;
                let y_val = y_img.get_pixel(x, y)[0] as f32;
                let magnitude = (x_val * x_val + y_val * y_val).sqrt();
                let value = magnitude.min(255.0) as u8;
                result.put_pixel(x, y, Luma([value]));
            }
        }

        Ok(DynamicImage::ImageLuma8(result))
    } else {
        Err(ImgrsError::InvalidOperation("Edge detection failed".to_string()))
    }
}

/// Apply emboss filter to an image
pub fn emboss(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    let kernel = vec![
        vec![-2.0, -1.0, 0.0],
        vec![-1.0, 1.0, 1.0],
        vec![0.0, 1.0, 2.0],
    ];

    apply_convolution(image, &kernel)
}


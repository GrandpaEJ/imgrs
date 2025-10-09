use image::{DynamicImage, ImageBuffer, Luma};
use crate::errors::ImgrsError;
use super::kernel::apply_convolution;

/// Apply Prewitt edge detection
pub fn prewitt_edge_detect(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    let gray_img = image.to_luma8();
    let gray_dynamic = DynamicImage::ImageLuma8(gray_img);

    let prewitt_x = vec![
        vec![-1.0, 0.0, 1.0],
        vec![-1.0, 0.0, 1.0],
        vec![-1.0, 0.0, 1.0],
    ];

    let prewitt_y = vec![
        vec![-1.0, -1.0, -1.0],
        vec![0.0, 0.0, 0.0],
        vec![1.0, 1.0, 1.0],
    ];

    let edge_x = apply_convolution(&gray_dynamic, &prewitt_x)?;
    let edge_y = apply_convolution(&gray_dynamic, &prewitt_y)?;

    combine_gradients(&edge_x, &edge_y)
}

/// Apply Scharr edge detection (more accurate than Sobel)
pub fn scharr_edge_detect(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    let gray_img = image.to_luma8();
    let gray_dynamic = DynamicImage::ImageLuma8(gray_img);

    let scharr_x = vec![
        vec![-3.0, 0.0, 3.0],
        vec![-10.0, 0.0, 10.0],
        vec![-3.0, 0.0, 3.0],
    ];

    let scharr_y = vec![
        vec![-3.0, -10.0, -3.0],
        vec![0.0, 0.0, 0.0],
        vec![3.0, 10.0, 3.0],
    ];

    let edge_x = apply_convolution(&gray_dynamic, &scharr_x)?;
    let edge_y = apply_convolution(&gray_dynamic, &scharr_y)?;

    combine_gradients(&edge_x, &edge_y)
}

/// Apply Roberts Cross edge detection
pub fn roberts_cross_edge_detect(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    let gray_img = image.to_luma8();
    let (width, height) = gray_img.dimensions();
    let mut result = ImageBuffer::new(width, height);

    for y in 0..height.saturating_sub(1) {
        for x in 0..width.saturating_sub(1) {
            let p1 = gray_img.get_pixel(x, y)[0] as f32;
            let p2 = gray_img.get_pixel(x + 1, y)[0] as f32;
            let p3 = gray_img.get_pixel(x, y + 1)[0] as f32;
            let p4 = gray_img.get_pixel(x + 1, y + 1)[0] as f32;

            let gx = p1 - p4;
            let gy = p2 - p3;
            let magnitude = (gx * gx + gy * gy).sqrt();
            let value = magnitude.min(255.0) as u8;

            result.put_pixel(x, y, Luma([value]));
        }
    }

    Ok(DynamicImage::ImageLuma8(result))
}

/// Apply Laplacian edge detection
pub fn laplacian_edge_detect(image: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    let gray_img = image.to_luma8();
    let gray_dynamic = DynamicImage::ImageLuma8(gray_img);

    let laplacian = vec![
        vec![0.0, -1.0, 0.0],
        vec![-1.0, 4.0, -1.0],
        vec![0.0, -1.0, 0.0],
    ];

    apply_convolution(&gray_dynamic, &laplacian)
}

/// Apply Laplacian of Gaussian (LoG) edge detection
pub fn laplacian_of_gaussian(image: &DynamicImage, sigma: f32) -> Result<DynamicImage, ImgrsError> {
    let gray_img = image.to_luma8();
    let gray_dynamic = DynamicImage::ImageLuma8(gray_img);

    // Create LoG kernel
    let size = (sigma * 6.0).ceil() as usize | 1; // Ensure odd size
    let center = size / 2;
    let mut kernel = vec![vec![0.0; size]; size];
    let sigma_sq = sigma * sigma;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center as f32;
            let dy = y as f32 - center as f32;
            let r_sq = dx * dx + dy * dy;
            
            kernel[y][x] = -1.0 / (std::f32::consts::PI * sigma_sq.powi(2)) *
                (1.0 - r_sq / (2.0 * sigma_sq)) *
                (-r_sq / (2.0 * sigma_sq)).exp();
        }
    }

    apply_convolution(&gray_dynamic, &kernel)
}

/// Apply simplified Canny edge detection
pub fn canny_edge_detect(image: &DynamicImage, low_threshold: f32, high_threshold: f32) -> Result<DynamicImage, ImgrsError> {
    // Step 1: Apply Gaussian blur
    use super::blur::blur;
    let blurred = blur(image, 1.0)?;
    
    // Step 2: Calculate gradients using Sobel
    let gray_img = blurred.to_luma8();
    let gray_dynamic = DynamicImage::ImageLuma8(gray_img.clone());

    let sobel_x = vec![
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 0.0, 2.0],
        vec![-1.0, 0.0, 1.0],
    ];

    let sobel_y = vec![
        vec![-1.0, -2.0, -1.0],
        vec![0.0, 0.0, 0.0],
        vec![1.0, 2.0, 1.0],
    ];

    let edge_x = apply_convolution(&gray_dynamic, &sobel_x)?;
    let edge_y = apply_convolution(&gray_dynamic, &sobel_y)?;

    // Step 3: Calculate gradient magnitude and direction
    if let (DynamicImage::ImageLuma8(x_img), DynamicImage::ImageLuma8(y_img)) = (&edge_x, &edge_y) {
        let (width, height) = x_img.dimensions();
        let mut magnitude = ImageBuffer::new(width, height);
        let mut direction = vec![vec![0.0f32; width as usize]; height as usize];

        for y in 0..height {
            for x in 0..width {
                let gx = x_img.get_pixel(x, y)[0] as f32;
                let gy = y_img.get_pixel(x, y)[0] as f32;
                let mag = (gx * gx + gy * gy).sqrt();
                magnitude.put_pixel(x, y, Luma([mag.min(255.0) as u8]));
                direction[y as usize][x as usize] = gy.atan2(gx);
            }
        }

        // Step 4: Non-maximum suppression
        let mut suppressed = ImageBuffer::new(width, height);
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let angle = direction[y as usize][x as usize];
                let mag = magnitude.get_pixel(x, y)[0] as f32;
                
                // Quantize angle to 4 directions
                let angle_deg = angle.to_degrees();
                let (dx, dy) = if (-22.5..=22.5).contains(&angle_deg) || angle_deg.abs() >= 157.5 {
                    (1, 0) // Horizontal
                } else if (22.5..=67.5).contains(&angle_deg) {
                    (1, 1) // Diagonal /
                } else if (67.5..=112.5).contains(&angle_deg) {
                    (0, 1) // Vertical
                } else {
                    (1, -1) // Diagonal \
                };

                let mag1 = magnitude.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32)[0] as f32;
                let mag2 = magnitude.get_pixel((x as i32 - dx) as u32, (y as i32 - dy) as u32)[0] as f32;

                if mag >= mag1 && mag >= mag2 {
                    suppressed.put_pixel(x, y, Luma([mag as u8]));
                } else {
                    suppressed.put_pixel(x, y, Luma([0]));
                }
            }
        }

        // Step 5: Double thresholding and edge tracking
        let mut result = ImageBuffer::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let val = suppressed.get_pixel(x, y)[0] as f32;
                if val >= high_threshold {
                    result.put_pixel(x, y, Luma([255]));
                } else if val >= low_threshold {
                    result.put_pixel(x, y, Luma([128])); // Weak edge
                } else {
                    result.put_pixel(x, y, Luma([0]));
                }
            }
        }

        // Connect weak edges to strong edges
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if result.get_pixel(x, y)[0] == 128 {
                    let mut has_strong_neighbor = false;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if result.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32)[0] == 255 {
                                has_strong_neighbor = true;
                                break;
                            }
                        }
                    }
                    if has_strong_neighbor {
                        result.put_pixel(x, y, Luma([255]));
                    } else {
                        result.put_pixel(x, y, Luma([0]));
                    }
                }
            }
        }

        Ok(DynamicImage::ImageLuma8(result))
    } else {
        Err(ImgrsError::InvalidOperation("Canny edge detection failed".to_string()))
    }
}

/// Helper function to combine X and Y gradients
fn combine_gradients(edge_x: &DynamicImage, edge_y: &DynamicImage) -> Result<DynamicImage, ImgrsError> {
    if let (DynamicImage::ImageLuma8(x_img), DynamicImage::ImageLuma8(y_img)) = (edge_x, edge_y) {
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


use image::{DynamicImage, ImageBuffer, Rgb, Rgba, Luma};
use crate::errors::ImgrsError;

/// Gaussian blur kernel for different radii
pub fn gaussian_kernel(radius: f32) -> Vec<Vec<f32>> {
    let size = (radius * 2.0).ceil() as usize * 2 + 1;
    let center = size / 2;
    let mut kernel = vec![vec![0.0; size]; size];
    let sigma = radius / 3.0;
    let two_sigma_sq = 2.0 * sigma * sigma;
    let mut sum = 0.0;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center as f32;
            let dy = y as f32 - center as f32;
            let distance_sq = dx * dx + dy * dy;
            let value = (-distance_sq / two_sigma_sq).exp();
            kernel[y][x] = value;
            sum += value;
        }
    }

    // Normalize kernel
    for y in 0..size {
        for x in 0..size {
            kernel[y][x] /= sum;
        }
    }

    kernel
}

/// Apply a convolution kernel to an image
pub fn apply_convolution(image: &DynamicImage, kernel: &[Vec<f32>]) -> Result<DynamicImage, ImgrsError> {
    let kernel_size = kernel.len();
    let kernel_center = kernel_size / 2;

    match image {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (width, height) = rgb_img.dimensions();
            let mut result = ImageBuffer::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let mut r_sum = 0.0;
                    let mut g_sum = 0.0;
                    let mut b_sum = 0.0;

                    for ky in 0..kernel_size {
                        for kx in 0..kernel_size {
                            let img_x = x as i32 + kx as i32 - kernel_center as i32;
                            let img_y = y as i32 + ky as i32 - kernel_center as i32;

                            // Handle edge cases by clamping coordinates
                            let img_x = img_x.max(0).min(width as i32 - 1) as u32;
                            let img_y = img_y.max(0).min(height as i32 - 1) as u32;

                            let pixel = rgb_img.get_pixel(img_x, img_y);
                            let kernel_val = kernel[ky][kx];

                            r_sum += pixel[0] as f32 * kernel_val;
                            g_sum += pixel[1] as f32 * kernel_val;
                            b_sum += pixel[2] as f32 * kernel_val;
                        }
                    }

                    let r = r_sum.max(0.0).min(255.0) as u8;
                    let g = g_sum.max(0.0).min(255.0) as u8;
                    let b = b_sum.max(0.0).min(255.0) as u8;

                    result.put_pixel(x, y, Rgb([r, g, b]));
                }
            }

            Ok(DynamicImage::ImageRgb8(result))
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (width, height) = rgba_img.dimensions();
            let mut result = ImageBuffer::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let mut r_sum = 0.0;
                    let mut g_sum = 0.0;
                    let mut b_sum = 0.0;
                    let mut a_sum = 0.0;

                    for ky in 0..kernel_size {
                        for kx in 0..kernel_size {
                            let img_x = x as i32 + kx as i32 - kernel_center as i32;
                            let img_y = y as i32 + ky as i32 - kernel_center as i32;

                            let img_x = img_x.max(0).min(width as i32 - 1) as u32;
                            let img_y = img_y.max(0).min(height as i32 - 1) as u32;

                            let pixel = rgba_img.get_pixel(img_x, img_y);
                            let kernel_val = kernel[ky][kx];

                            r_sum += pixel[0] as f32 * kernel_val;
                            g_sum += pixel[1] as f32 * kernel_val;
                            b_sum += pixel[2] as f32 * kernel_val;
                            a_sum += pixel[3] as f32 * kernel_val;
                        }
                    }

                    let r = r_sum.max(0.0).min(255.0) as u8;
                    let g = g_sum.max(0.0).min(255.0) as u8;
                    let b = b_sum.max(0.0).min(255.0) as u8;
                    let a = a_sum.max(0.0).min(255.0) as u8;

                    result.put_pixel(x, y, Rgba([r, g, b, a]));
                }
            }

            Ok(DynamicImage::ImageRgba8(result))
        }
        DynamicImage::ImageLuma8(gray_img) => {
            let (width, height) = gray_img.dimensions();
            let mut result = ImageBuffer::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let mut sum = 0.0;

                    for ky in 0..kernel_size {
                        for kx in 0..kernel_size {
                            let img_x = x as i32 + kx as i32 - kernel_center as i32;
                            let img_y = y as i32 + ky as i32 - kernel_center as i32;

                            let img_x = img_x.max(0).min(width as i32 - 1) as u32;
                            let img_y = img_y.max(0).min(height as i32 - 1) as u32;

                            let pixel = gray_img.get_pixel(img_x, img_y);
                            let kernel_val = kernel[ky][kx];

                            sum += pixel[0] as f32 * kernel_val;
                        }
                    }

                    let value = sum.max(0.0).min(255.0) as u8;
                    result.put_pixel(x, y, Luma([value]));
                }
            }

            Ok(DynamicImage::ImageLuma8(result))
        }
        _ => {
            // Convert to RGB and apply filter
            let rgb_img = image.to_rgb8();
            let rgb_dynamic = DynamicImage::ImageRgb8(rgb_img);
            apply_convolution(&rgb_dynamic, kernel)
        }
    }
}


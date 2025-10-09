use image::DynamicImage;
use crate::errors::ImgrsError;
use super::kernel::{gaussian_kernel, apply_convolution};

/// Apply Gaussian blur to an image
pub fn blur(image: &DynamicImage, radius: f32) -> Result<DynamicImage, ImgrsError> {
    if radius <= 0.0 {
        return Ok(image.clone());
    }

    let kernel = gaussian_kernel(radius);
    apply_convolution(image, &kernel)
}


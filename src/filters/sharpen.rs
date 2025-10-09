use image::DynamicImage;
use crate::errors::ImgrsError;
use super::kernel::apply_convolution;

/// Apply sharpening filter to an image
pub fn sharpen(image: &DynamicImage, strength: f32) -> Result<DynamicImage, ImgrsError> {
    let kernel = vec![
        vec![0.0, -strength, 0.0],
        vec![-strength, 1.0 + 4.0 * strength, -strength],
        vec![0.0, -strength, 0.0],
    ];

    apply_convolution(image, &kernel)
}


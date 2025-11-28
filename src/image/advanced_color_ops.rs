// Advanced gradient and pattern operations
use crate::errors::ImgrsError;
use crate::image::core::LazyImage; // {, PyImage};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

impl crate::image::core::PyImage {
    pub fn apply_gradient_overlay_impl(
        &mut self,
        color: (u8, u8, u8, u8),
        direction: &str,
        opacity: f32,
    ) -> Result<Self, ImgrsError> {
        let gradient_mask = self.create_gradient_mask_impl(direction, opacity, opacity)?;
        let mask = gradient_mask.to_rgba8();
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());

        for y in 0..rgba_image.height() {
            for x in 0..rgba_image.width() {
                let pixel = rgba_image.get_pixel(x, y);
                let mask_pixel = mask.get_pixel(x, y);
                let mask_alpha = (mask_pixel[3] as f32 / 255.0) * (color.3 as f32 / 255.0);

                let blended_r =
                    (pixel[0] as f32 * (1.0 - mask_alpha) + color.0 as f32 * mask_alpha) as u8;
                let blended_g =
                    (pixel[1] as f32 * (1.0 - mask_alpha) + color.1 as f32 * mask_alpha) as u8;
                let blended_b =
                    (pixel[2] as f32 * (1.0 - mask_alpha) + color.2 as f32 * mask_alpha) as u8;

                result.put_pixel(x, y, Rgba([blended_r, blended_g, blended_b, pixel[3]]));
            }
        }

        self.lazy_image = LazyImage::Loaded(DynamicImage::ImageRgba8(result));
        Ok(self.clone())
    }

    pub fn create_stripe_pattern_impl(
        &mut self,
        color: (u8, u8, u8, u8),
        _width: u32,
        spacing: u32,
        angle: f32,
    ) -> Result<DynamicImage, ImgrsError> {
        let (width, height) = if let Ok(image) = self.get_image() {
            image.dimensions()
        } else {
            (100, 100)
        };

        let mut pattern = ImageBuffer::new(width, height);
        let angle_rad = angle.to_radians();

        // Create stripes in direction perpendicular to angle
        for y in 0..height {
            for x in 0..width {
                let distance = ((x as f32 * angle_rad.cos() + y as f32 * angle_rad.sin()) as i32
                    % ((width + spacing) as i32)) as u32;

                let is_stripe = distance < _width;
                let alpha = if is_stripe { color.3 } else { 0 };

                pattern.put_pixel(x, y, Rgba([color.0, color.1, color.2, alpha]));
            }
        }

        Ok(DynamicImage::ImageRgba8(pattern))
    }

    pub fn create_checker_pattern_impl(
        &mut self,
        color1: (u8, u8, u8, u8),
        color2: (u8, u8, u8, u8),
        size: u32,
    ) -> Result<DynamicImage, ImgrsError> {
        let (width, height) = if let Ok(image) = self.get_image() {
            image.dimensions()
        } else {
            (100, 100)
        };

        let mut pattern = ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let check_x = (x / size) % 2;
                let check_y = (y / size) % 2;

                let use_color1 = (check_x + check_y) % 2 == 0;
                let (r, g, b, a) = if use_color1 { color1 } else { color2 };

                pattern.put_pixel(x, y, Rgba([r, g, b, a]));
            }
        }

        Ok(DynamicImage::ImageRgba8(pattern))
    }

    pub fn split_alpha_impl(&mut self) -> Result<(Self, Self), ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let (width, height) = rgba_image.dimensions();

        let mut rgb_image = ImageBuffer::new(width, height);
        let mut alpha_image = ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_image.get_pixel(x, y);

                // RGB image (alpha set to 255)
                rgb_image.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], 255]));

                // Alpha image (grayscale from alpha channel)
                let gray = pixel[3];
                alpha_image.put_pixel(x, y, Rgba([gray, gray, gray, 255]));
            }
        }

        let rgb_pyimage =
            crate::image::core::PyImage::new_from_image(DynamicImage::ImageRgba8(rgb_image), None);
        let alpha_pyimage = crate::image::core::PyImage::new_from_image(
            DynamicImage::ImageRgba8(alpha_image),
            None,
        );

        Ok((rgb_pyimage, alpha_pyimage))
    }

    pub fn merge_alpha_impl(&mut self, alpha_image: DynamicImage) -> Result<Self, ImgrsError> {
        let image = self.get_image_mut()?; // mut
        let rgba_image = image.to_rgba8();
        let alpha_rgba = alpha_image.to_rgba8();

        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());

        for y in 0..rgba_image.height().min(alpha_rgba.height()) {
            for x in 0..rgba_image.width().min(alpha_rgba.width()) {
                let pixel = rgba_image.get_pixel(x, y);
                let alpha_pixel = alpha_rgba.get_pixel(x, y);

                result.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], alpha_pixel[0]]));
            }
        }

        self.lazy_image = LazyImage::Loaded(DynamicImage::ImageRgba8(result));
        Ok(self.clone())
    }

    pub fn alpha_to_color_impl(
        &mut self,
        background_color: (u8, u8, u8),
    ) -> Result<Self, ImgrsError> {
        let image = self.get_image()?;
        let rgba_image = image.to_rgba8();
        let mut result = ImageBuffer::new(rgba_image.width(), rgba_image.height());

        for y in 0..rgba_image.height() {
            for x in 0..rgba_image.width() {
                let pixel = rgba_image.get_pixel(x, y);
                let alpha = pixel[3] as f32 / 255.0;

                let final_r =
                    (background_color.0 as f32 * (1.0 - alpha) + pixel[0] as f32 * alpha) as u8;
                let final_g =
                    (background_color.1 as f32 * (1.0 - alpha) + pixel[1] as f32 * alpha) as u8;
                let final_b =
                    (background_color.2 as f32 * (1.0 - alpha) + pixel[2] as f32 * alpha) as u8;

                result.put_pixel(x, y, Rgba([final_r, final_g, final_b, 255]));
            }
        }

        self.lazy_image = LazyImage::Loaded(DynamicImage::ImageRgba8(result));
        Ok(self.clone())
    }


}

use pyo3::prelude::*;
use crate::{filters, css_filters};
use super::core::{PyImage, LazyImage};

impl PyImage {
    pub fn blur_impl(&mut self, radius: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                filters::blur(image, radius)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn sharpen_impl(&mut self, strength: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                filters::sharpen(image, strength)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn edge_detect_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                filters::edge_detect(image)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn emboss_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                filters::emboss(image)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn brightness_impl(&mut self, adjustment: i16) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                filters::brightness(image, adjustment)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn contrast_impl(&mut self, factor: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                filters::contrast(image, factor)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    // Advanced Blur Effects
    pub fn box_blur_impl(&mut self, radius: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::box_blur(image, radius))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn motion_blur_impl(&mut self, size: u32, angle: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::motion_blur(image, size, angle))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn median_blur_impl(&mut self, radius: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::median_blur(image, radius))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn bilateral_blur_impl(&mut self, radius: u32, sigma_color: f32, sigma_space: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::bilateral_blur(image, radius, sigma_color, sigma_space))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn radial_blur_impl(&mut self, strength: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::radial_blur(image, strength))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn zoom_blur_impl(&mut self, strength: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::zoom_blur(image, strength))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    // Advanced Edge Detection
    pub fn prewitt_edge_detect_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::prewitt_edge_detect(image))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn scharr_edge_detect_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::scharr_edge_detect(image))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn roberts_cross_edge_detect_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::roberts_cross_edge_detect(image))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn laplacian_edge_detect_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::laplacian_edge_detect(image))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn laplacian_of_gaussian_impl(&mut self, sigma: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::laplacian_of_gaussian(image, sigma))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn canny_edge_detect_impl(&mut self, low_threshold: f32, high_threshold: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::canny_edge_detect(image, low_threshold, high_threshold))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    // Advanced Sharpening
    pub fn unsharp_mask_impl(&mut self, radius: f32, amount: f32, threshold: u8) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::unsharp_mask(image, radius, amount, threshold))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn high_pass_impl(&mut self, radius: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::high_pass(image, radius))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn edge_enhance_impl(&mut self, strength: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::edge_enhance(image, strength))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn edge_enhance_more_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::edge_enhance_more(image))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    // Stylistic Effects
    pub fn oil_painting_impl(&mut self, radius: u32, intensity: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::oil_painting(image, radius, intensity))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn posterize_filter_impl(&mut self, levels: u8) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::posterize(image, levels))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn pixelate_impl(&mut self, pixel_size: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::pixelate(image, pixel_size))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn mosaic_impl(&mut self, tile_size: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::mosaic(image, tile_size))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn cartoon_impl(&mut self, num_levels: u8, edge_threshold: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::cartoon(image, num_levels, edge_threshold))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn sketch_impl(&mut self, detail_level: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::sketch(image, detail_level))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn solarize_impl(&mut self, threshold: u8) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::solarize(image, threshold))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    // Noise Effects
    pub fn add_gaussian_noise_impl(&mut self, mean: f32, stddev: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::add_gaussian_noise(image, mean, stddev))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn add_salt_pepper_noise_impl(&mut self, amount: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::add_salt_pepper_noise(image, amount))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn denoise_impl(&mut self, radius: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::denoise(image, radius))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    // Morphological Operations
    pub fn dilate_impl(&mut self, radius: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::dilate(image, radius))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn erode_impl(&mut self, radius: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::erode(image, radius))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn morphological_opening_impl(&mut self, radius: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::opening(image, radius))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn morphological_closing_impl(&mut self, radius: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::closing(image, radius))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn morphological_gradient_impl(&mut self, radius: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::morphological_gradient(image, radius))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    // Artistic Effects
    pub fn vignette_impl(&mut self, strength: f32, radius: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::vignette(image, strength, radius))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn halftone_impl(&mut self, dot_size: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::halftone(image, dot_size))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn pencil_sketch_impl(&mut self, detail: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::pencil_sketch(image, detail))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn watercolor_impl(&mut self, iterations: u32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::watercolor(image, iterations))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn glitch_impl(&mut self, intensity: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::glitch(image, intensity))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    // Color Effects
    pub fn duotone_impl(&mut self, shadow: (u8, u8, u8), highlight: (u8, u8, u8)) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::duotone(image, shadow, highlight))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn color_splash_impl(&mut self, target_hue: f32, tolerance: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::color_splash(image, target_hue, tolerance))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn chromatic_aberration_impl(&mut self, strength: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::chromatic_aberration(image, strength))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    // Auto-Enhancement Features
    pub fn histogram_equalization_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::histogram_equalization(image))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn auto_contrast_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::auto_contrast(image))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn auto_brightness_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::auto_brightness(image))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn auto_enhance_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::auto_enhance(image))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn exposure_adjust_impl(&mut self, exposure: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::exposure_adjust(image, exposure))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn auto_level_impl(&mut self, black_clip: f32, white_clip: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::auto_level(image, black_clip, white_clip))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn normalize_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::normalize(image))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn smart_enhance_impl(&mut self, strength: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::smart_enhance(image, strength))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    pub fn auto_white_balance_impl(&mut self) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        Python::with_gil(|py| {
            py.allow_threads(|| filters::auto_white_balance(image))
        }).map(|filtered| PyImage { lazy_image: LazyImage::Loaded(filtered), format }).map_err(|e| e.into())
    }

    // CSS-like filters
    pub fn sepia_impl(&mut self, amount: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                css_filters::sepia(image, amount)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn grayscale_filter_impl(&mut self, amount: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                css_filters::grayscale(image, amount)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn invert_impl(&mut self, amount: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                css_filters::invert(image, amount)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn hue_rotate_impl(&mut self, degrees: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                css_filters::hue_rotate(image, degrees)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }

    pub fn saturate_impl(&mut self, amount: f32) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                css_filters::saturate(image, amount)
            })
        }).map(|filtered| PyImage {
            lazy_image: LazyImage::Loaded(filtered),
            format,
        }).map_err(|e| e.into())
    }
}


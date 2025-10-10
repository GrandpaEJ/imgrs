use pyo3::prelude::*;
use super::core::PyImage;

#[pymethods]
impl PyImage {
    // Constructor methods (from constructors.rs)
    #[new]
    fn py_new() -> Self {
        Self::new_default()
    }

    #[staticmethod]
    #[pyo3(signature = (mode, size, color=None))]
    fn new(mode: &str, size: (u32, u32), color: Option<(u8, u8, u8, u8)>) -> PyResult<Self> {
        Self::new_with_mode(mode, size, color)
    }

    #[staticmethod]
    fn open(path_or_bytes: &Bound<'_, PyAny>) -> PyResult<Self> {
        Self::open_impl(path_or_bytes)
    }

    #[staticmethod]
    #[pyo3(signature = (array, _mode=None))]
    fn fromarray(array: &Bound<'_, PyAny>, _mode: Option<&str>) -> PyResult<Self> {
        Self::fromarray_impl(array, _mode)
    }

    #[staticmethod]
    #[pyo3(signature = (mode, size, data))]
    fn frombytes(mode: &str, size: (u32, u32), data: &[u8]) -> PyResult<Self> {
        Self::frombytes_impl(mode, size, data)
    }

    // I/O methods (from io.rs)
    #[pyo3(signature = (path_or_buffer, format=None))]
    fn save(&mut self, path_or_buffer: &Bound<'_, PyAny>, format: Option<String>) -> PyResult<()> {
        self.save_impl(path_or_buffer, format)
    }

    fn to_bytes(&mut self) -> PyResult<Py<pyo3::types::PyBytes>> {
        self.to_bytes_impl()
    }

    // Property methods (from properties.rs)
    #[getter]
    fn size(&mut self) -> PyResult<(u32, u32)> {
        self.size_impl()
    }

    #[getter]
    fn width(&mut self) -> PyResult<u32> {
        self.width_impl()
    }

    #[getter]
    fn height(&mut self) -> PyResult<u32> {
        self.height_impl()
    }

    #[getter]
    fn mode(&mut self) -> PyResult<String> {
        self.mode_impl()
    }

    #[getter]
    fn format(&self) -> Option<String> {
        self.format_impl()
    }

    fn __repr__(&mut self) -> String {
        self.repr_impl()
    }

    // Transform methods (from transform.rs)
    #[pyo3(signature = (size, resample=None))]
    fn resize(&mut self, size: (u32, u32), resample: Option<String>) -> PyResult<Self> {
        self.resize_impl(size, resample)
    }

    fn crop(&mut self, box_coords: (u32, u32, u32, u32)) -> PyResult<Self> {
        self.crop_impl(box_coords)
    }

    fn rotate(&mut self, angle: f64) -> PyResult<Self> {
        self.rotate_impl(angle)
    }

    fn transpose(&mut self, method: String) -> PyResult<Self> {
        self.transpose_impl(method)
    }

    // Manipulation methods (from manipulation.rs)
    fn copy(&self) -> Self {
        self.copy_impl()
    }

    fn convert(&mut self, mode: &str) -> PyResult<Self> {
        self.convert_impl(mode)
    }

    fn split(&mut self) -> PyResult<Vec<Self>> {
        self.split_impl()
    }

    #[pyo3(signature = (other, position=None, mask=None))]
    fn paste(&mut self, other: &mut Self, position: Option<(i32, i32)>, mask: Option<Self>) -> PyResult<Self> {
        self.paste_impl(other, position, mask)
    }

    // Filter methods (from filters.rs)
    fn blur(&mut self, radius: f32) -> PyResult<Self> {
        self.blur_impl(radius)
    }

    fn sharpen(&mut self, strength: f32) -> PyResult<Self> {
        self.sharpen_impl(strength)
    }

    fn edge_detect(&mut self) -> PyResult<Self> {
        self.edge_detect_impl()
    }

    fn emboss(&mut self) -> PyResult<Self> {
        self.emboss_impl()
    }

    fn brightness(&mut self, adjustment: i16) -> PyResult<Self> {
        self.brightness_impl(adjustment)
    }

    fn contrast(&mut self, factor: f32) -> PyResult<Self> {
        self.contrast_impl(factor)
    }

    fn sepia(&mut self, amount: f32) -> PyResult<Self> {
        self.sepia_impl(amount)
    }

    fn grayscale_filter(&mut self, amount: f32) -> PyResult<Self> {
        self.grayscale_filter_impl(amount)
    }

    fn invert(&mut self, amount: f32) -> PyResult<Self> {
        self.invert_impl(amount)
    }

    fn hue_rotate(&mut self, degrees: f32) -> PyResult<Self> {
        self.hue_rotate_impl(degrees)
    }

    fn saturate(&mut self, amount: f32) -> PyResult<Self> {
        self.saturate_impl(amount)
    }

    // Advanced Blur Filters
    fn box_blur(&mut self, radius: u32) -> PyResult<Self> {
        self.box_blur_impl(radius)
    }

    fn motion_blur(&mut self, size: u32, angle: f32) -> PyResult<Self> {
        self.motion_blur_impl(size, angle)
    }

    fn median_blur(&mut self, radius: u32) -> PyResult<Self> {
        self.median_blur_impl(radius)
    }

    fn bilateral_blur(&mut self, radius: u32, sigma_color: f32, sigma_space: f32) -> PyResult<Self> {
        self.bilateral_blur_impl(radius, sigma_color, sigma_space)
    }

    fn radial_blur(&mut self, strength: f32) -> PyResult<Self> {
        self.radial_blur_impl(strength)
    }

    fn zoom_blur(&mut self, strength: f32) -> PyResult<Self> {
        self.zoom_blur_impl(strength)
    }

    // Advanced Edge Detection
    fn prewitt_edge_detect(&mut self) -> PyResult<Self> {
        self.prewitt_edge_detect_impl()
    }

    fn scharr_edge_detect(&mut self) -> PyResult<Self> {
        self.scharr_edge_detect_impl()
    }

    fn roberts_cross_edge_detect(&mut self) -> PyResult<Self> {
        self.roberts_cross_edge_detect_impl()
    }

    fn laplacian_edge_detect(&mut self) -> PyResult<Self> {
        self.laplacian_edge_detect_impl()
    }

    fn laplacian_of_gaussian(&mut self, sigma: f32) -> PyResult<Self> {
        self.laplacian_of_gaussian_impl(sigma)
    }

    fn canny_edge_detect(&mut self, low_threshold: f32, high_threshold: f32) -> PyResult<Self> {
        self.canny_edge_detect_impl(low_threshold, high_threshold)
    }

    // Advanced Sharpening
    fn unsharp_mask(&mut self, radius: f32, amount: f32, threshold: u8) -> PyResult<Self> {
        self.unsharp_mask_impl(radius, amount, threshold)
    }

    fn high_pass(&mut self, radius: f32) -> PyResult<Self> {
        self.high_pass_impl(radius)
    }

    fn edge_enhance(&mut self, strength: f32) -> PyResult<Self> {
        self.edge_enhance_impl(strength)
    }

    fn edge_enhance_more(&mut self) -> PyResult<Self> {
        self.edge_enhance_more_impl()
    }

    // Stylistic Effects
    fn oil_painting(&mut self, radius: u32, intensity: u32) -> PyResult<Self> {
        self.oil_painting_impl(radius, intensity)
    }

    fn pixelate(&mut self, pixel_size: u32) -> PyResult<Self> {
        self.pixelate_impl(pixel_size)
    }

    fn mosaic(&mut self, tile_size: u32) -> PyResult<Self> {
        self.mosaic_impl(tile_size)
    }

    fn posterize_filter(&mut self, levels: u8) -> PyResult<Self> {
        self.posterize_filter_impl(levels)
    }

    fn cartoon(&mut self, num_levels: u8, edge_threshold: f32) -> PyResult<Self> {
        self.cartoon_impl(num_levels, edge_threshold)
    }

    fn sketch(&mut self, detail_level: f32) -> PyResult<Self> {
        self.sketch_impl(detail_level)
    }

    fn solarize(&mut self, threshold: u8) -> PyResult<Self> {
        self.solarize_impl(threshold)
    }

    // Noise Effects
    fn add_gaussian_noise(&mut self, mean: f32, stddev: f32) -> PyResult<Self> {
        self.add_gaussian_noise_impl(mean, stddev)
    }

    fn add_salt_pepper_noise(&mut self, amount: f32) -> PyResult<Self> {
        self.add_salt_pepper_noise_impl(amount)
    }

    fn denoise(&mut self, radius: u32) -> PyResult<Self> {
        self.denoise_impl(radius)
    }

    // Morphological Operations
    fn dilate(&mut self, radius: u32) -> PyResult<Self> {
        self.dilate_impl(radius)
    }

    fn erode(&mut self, radius: u32) -> PyResult<Self> {
        self.erode_impl(radius)
    }

    fn morphological_opening(&mut self, radius: u32) -> PyResult<Self> {
        self.morphological_opening_impl(radius)
    }

    fn morphological_closing(&mut self, radius: u32) -> PyResult<Self> {
        self.morphological_closing_impl(radius)
    }

    fn morphological_gradient(&mut self, radius: u32) -> PyResult<Self> {
        self.morphological_gradient_impl(radius)
    }

    // Artistic Effects
    fn vignette(&mut self, strength: f32, radius: f32) -> PyResult<Self> {
        self.vignette_impl(strength, radius)
    }

    fn halftone(&mut self, dot_size: u32) -> PyResult<Self> {
        self.halftone_impl(dot_size)
    }

    fn pencil_sketch(&mut self, detail: f32) -> PyResult<Self> {
        self.pencil_sketch_impl(detail)
    }

    fn watercolor(&mut self, iterations: u32) -> PyResult<Self> {
        self.watercolor_impl(iterations)
    }

    fn glitch(&mut self, intensity: f32) -> PyResult<Self> {
        self.glitch_impl(intensity)
    }

    // Color Effects
    fn duotone(&mut self, shadow: (u8, u8, u8), highlight: (u8, u8, u8)) -> PyResult<Self> {
        self.duotone_impl(shadow, highlight)
    }

    fn color_splash(&mut self, target_hue: f32, tolerance: f32) -> PyResult<Self> {
        self.color_splash_impl(target_hue, tolerance)
    }

    fn chromatic_aberration(&mut self, strength: f32) -> PyResult<Self> {
        self.chromatic_aberration_impl(strength)
    }

    // Emoji Effects
    fn add_emoji(&mut self, emoji_name: &str, x: i32, y: i32, size: u32, opacity: f32) -> PyResult<Self> {
        self.add_emoji_impl(emoji_name, x, y, size, opacity)
    }

    fn add_emoji_text(&mut self, emoji: &str, x: i32, y: i32, size: u32, opacity: f32) -> PyResult<Self> {
        self.add_emoji_text_impl(emoji, x, y, size, opacity)
    }

    fn add_emoji_quick(&mut self, emoji_name: &str, x: i32, y: i32, size: u32) -> PyResult<Self> {
        self.add_emoji_quick_impl(emoji_name, x, y, size)
    }

    fn add_emojis(&mut self, emojis: Vec<(String, i32, i32, u32, f32)>) -> PyResult<Self> {
        self.add_emojis_batch_impl(emojis)
    }

    // Auto-Enhancement Features
    fn histogram_equalization(&mut self) -> PyResult<Self> {
        self.histogram_equalization_impl()
    }

    fn auto_contrast(&mut self) -> PyResult<Self> {
        self.auto_contrast_impl()
    }

    fn auto_brightness(&mut self) -> PyResult<Self> {
        self.auto_brightness_impl()
    }

    fn auto_enhance(&mut self) -> PyResult<Self> {
        self.auto_enhance_impl()
    }

    fn exposure_adjust(&mut self, exposure: f32) -> PyResult<Self> {
        self.exposure_adjust_impl(exposure)
    }

    fn auto_level(&mut self, black_clip: f32, white_clip: f32) -> PyResult<Self> {
        self.auto_level_impl(black_clip, white_clip)
    }

    fn normalize(&mut self) -> PyResult<Self> {
        self.normalize_impl()
    }

    fn smart_enhance(&mut self, strength: f32) -> PyResult<Self> {
        self.smart_enhance_impl(strength)
    }

    fn auto_white_balance(&mut self) -> PyResult<Self> {
        self.auto_white_balance_impl()
    }

    // Metadata Operations
    fn get_metadata(&mut self, path: String) -> PyResult<Py<pyo3::types::PyDict>> {
        self.get_metadata_impl(path)
    }

    fn get_metadata_summary(&mut self, path: String) -> PyResult<String> {
        self.get_metadata_summary_impl(path)
    }

    fn has_exif(&mut self, path: String) -> PyResult<bool> {
        self.has_exif_impl(path)
    }

    fn has_gps(&mut self, path: String) -> PyResult<bool> {
        self.has_gps_impl(path)
    }

    // Rich text methods (from text_ops.rs)
    #[pyo3(signature = (text, x, y, size=32.0, color=(0, 0, 0, 255), font_path=None))]
    fn text(
        &self,
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_path: Option<&str>,
    ) -> PyResult<Self> {
        Ok(self.draw_rich_text_impl(text, x, y, size, color, font_path)?)
    }

    #[pyo3(signature = (
        text, x, y,
        size=32.0,
        color=(0, 0, 0, 255),
        font_path=None,
        align=None,
        background=None,
        outline=None,
        shadow=None,
        opacity=None,
        line_spacing=None,
        letter_spacing=None,
        max_width=None,
        rotation=None
    ))]
    fn text_styled(
        &self,
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_path: Option<&str>,
        align: Option<&str>,
        background: Option<(u8, u8, u8, u8)>,
        outline: Option<(u8, u8, u8, u8, f32)>,
        shadow: Option<(i32, i32, u8, u8, u8, u8)>,
        opacity: Option<f32>,
        line_spacing: Option<f32>,
        letter_spacing: Option<f32>,
        max_width: Option<u32>,
        rotation: Option<f32>,
    ) -> PyResult<Self> {
        Ok(self.draw_rich_text_styled_impl(
            text, x, y, size, color, font_path, align, background,
            outline, shadow, opacity, line_spacing, letter_spacing,
            max_width, rotation
        )?)
    }

    #[pyo3(signature = (text, y, size=32.0, color=(0, 0, 0, 255), font_path=None))]
    fn text_centered(
        &self,
        text: &str,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_path: Option<&str>,
    ) -> PyResult<Self> {
        Ok(self.draw_rich_text_centered_impl(text, y, size, color, font_path)?)
    }

    #[pyo3(signature = (text, x, y, size=32.0, color=(0, 0, 0, 255), font_path=None, line_spacing=None, align=None))]
    fn text_multiline(
        &self,
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_path: Option<&str>,
        line_spacing: Option<f32>,
        align: Option<&str>,
    ) -> PyResult<Self> {
        Ok(self.draw_rich_text_multiline_impl(text, x, y, size, color, font_path, line_spacing, align)?)
    }

    // Text measurement methods (from text_ops.rs)
    #[staticmethod]
    #[pyo3(signature = (text, size=32.0, font_path=None))]
    fn get_text_size(
        text: &str,
        size: f32,
        font_path: Option<&str>,
    ) -> PyResult<(u32, u32)> {
        Ok(Self::get_text_size_impl(text, size, font_path)?)
    }

    #[staticmethod]
    #[pyo3(signature = (text, size=32.0, line_spacing=1.2, font_path=None))]
    fn get_multiline_text_size(
        text: &str,
        size: f32,
        line_spacing: f32,
        font_path: Option<&str>,
    ) -> PyResult<(u32, u32, usize)> {
        Ok(Self::get_multiline_text_size_impl(text, size, line_spacing, font_path)?)
    }

    #[staticmethod]
    #[pyo3(signature = (text, x, y, size=32.0, font_path=None))]
    fn get_text_box(
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        font_path: Option<&str>,
    ) -> PyResult<PyObject> {
        Ok(Self::get_text_box_impl(text, x, y, size, font_path)?)
    }

    // Pixel operation methods (from pixel_ops.rs)
    fn getpixel(&mut self, x: u32, y: u32) -> PyResult<(u8, u8, u8, u8)> {
        self.getpixel_impl(x, y)
    }

    fn putpixel(&mut self, x: u32, y: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        self.putpixel_impl(x, y, color)
    }

    fn histogram(&mut self) -> PyResult<(Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>)> {
        self.histogram_impl()
    }

    fn dominant_color(&mut self) -> PyResult<(u8, u8, u8, u8)> {
        self.dominant_color_impl()
    }

    fn average_color(&mut self) -> PyResult<(u8, u8, u8, u8)> {
        self.average_color_impl()
    }

    fn replace_color(&mut self, target_color: (u8, u8, u8, u8), replacement_color: (u8, u8, u8, u8), tolerance: u8) -> PyResult<Self> {
        self.replace_color_impl(target_color, replacement_color, tolerance)
    }

    fn threshold(&mut self, threshold_value: u8) -> PyResult<Self> {
        self.threshold_impl(threshold_value)
    }

    fn posterize(&mut self, levels: u8) -> PyResult<Self> {
        self.posterize_impl(levels)
    }

    // Drawing methods (from drawing.rs)
    fn draw_rectangle(&mut self, x: i32, y: i32, width: u32, height: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        self.draw_rectangle_impl(x, y, width, height, color)
    }

    fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        self.draw_circle_impl(center_x, center_y, radius, color)
    }

    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        self.draw_line_impl(x0, y0, x1, y1, color)
    }

    fn draw_text(&mut self, text: &str, x: i32, y: i32, color: (u8, u8, u8, u8), scale: u32) -> PyResult<Self> {
        self.draw_text_impl(text, x, y, color, scale)
    }

    // Effect methods (from effects.rs)
    fn drop_shadow(&mut self, offset_x: i32, offset_y: i32, blur_radius: f32, shadow_color: (u8, u8, u8, u8)) -> PyResult<Self> {
        self.drop_shadow_impl(offset_x, offset_y, blur_radius, shadow_color)
    }

    fn inner_shadow(&mut self, offset_x: i32, offset_y: i32, blur_radius: f32, shadow_color: (u8, u8, u8, u8)) -> PyResult<Self> {
        self.inner_shadow_impl(offset_x, offset_y, blur_radius, shadow_color)
    }

    fn glow(&mut self, blur_radius: f32, glow_color: (u8, u8, u8, u8), intensity: f32) -> PyResult<Self> {
        self.glow_impl(blur_radius, glow_color, intensity)
    }
}

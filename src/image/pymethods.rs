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

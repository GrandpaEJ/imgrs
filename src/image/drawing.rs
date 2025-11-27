use super::core::{LazyImage, PyImage};
use crate::drawing;
use crate::text;
use pyo3::prelude::*;

impl PyImage {
    pub fn draw_rectangle_impl(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        color: (u8, u8, u8, u8),
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| drawing::draw_rectangle(image, x, y, width, height, color))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    pub fn draw_circle_impl(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: u32,
        color: (u8, u8, u8, u8),
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| drawing::draw_circle(image, center_x, center_y, radius, color))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    pub fn draw_line_impl(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        color: (u8, u8, u8, u8),
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| py.allow_threads(|| drawing::draw_line(image, x0, y0, x1, y1, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format,
            })
            .map_err(|e| e.into())
    }

    pub fn draw_star_impl(
        &mut self,
        center_x: i32,
        center_y: i32,
        outer_radius: u32,
        inner_radius: u32,
        points: u32,
        color: (u8, u8, u8, u8),
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                drawing::draw_star(
                    image,
                    center_x,
                    center_y,
                    outer_radius,
                    inner_radius,
                    points,
                    color,
                )
            })
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    pub fn draw_triangle_impl(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        color: (u8, u8, u8, u8),
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| drawing::draw_triangle(image, x1, y1, x2, y2, x3, y3, color))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    pub fn draw_polygon_impl(
        &mut self,
        points: Vec<(i32, i32)>,
        color: (u8, u8, u8, u8),
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| py.allow_threads(|| drawing::draw_polygon(image, points, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format,
            })
            .map_err(|e| e.into())
    }

    pub fn draw_ellipse_impl(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius_x: u32,
        radius_y: u32,
        color: (u8, u8, u8, u8),
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                drawing::draw_ellipse(image, center_x, center_y, radius_x, radius_y, color)
            })
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    pub fn draw_regular_polygon_impl(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: u32,
        sides: u32,
        rotation: f32,
        color: (u8, u8, u8, u8),
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| {
                drawing::draw_regular_polygon(
                    image, center_x, center_y, radius, sides, rotation, color,
                )
            })
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    pub fn draw_text_impl(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        color: (u8, u8, u8, u8),
        scale: u32,
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| text::draw_text(image, text, x, y, color, scale))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    pub fn add_text_impl(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        size: u32,
        color: (u8, u8, u8, u8),
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| text::add_text(image, text, (x, y), size, color))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    pub fn add_text_styled_impl(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        size: u32,
        color: (u8, u8, u8, u8),
        outline_r: u8,
        outline_g: u8,
        outline_b: u8,
        outline_a: u8,
        outline_width: f32,
        shadow_offset_x: i32,
        shadow_offset_y: i32,
        shadow_r: u8,
        shadow_g: u8,
        shadow_b: u8,
        shadow_a: u8,
        background_r: u8,
        background_g: u8,
        background_b: u8,
        background_a: u8,
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        let style = text::TextStyle {
            outline: if outline_width > 0.0 {
                Some((outline_r, outline_g, outline_b, outline_a, outline_width))
            } else {
                None
            },
            shadow: if shadow_offset_x != 0 || shadow_offset_y != 0 {
                Some((shadow_offset_x, shadow_offset_y, shadow_r, shadow_g, shadow_b, shadow_a))
            } else {
                None
            },
            background: if background_a > 0 {
                Some((background_r, background_g, background_b, background_a))
            } else {
                None
            },
        };

        Python::with_gil(|py| {
            py.allow_threads(|| text::add_text_styled(image, text, (x, y), size, color, style))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    pub fn add_text_multiline_impl(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        size: u32,
        color: (u8, u8, u8, u8),
        line_spacing: f32,
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        Python::with_gil(|py| {
            py.allow_threads(|| text::add_text_multiline(image, text, (x, y), size, color, line_spacing))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }
}

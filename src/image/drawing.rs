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
        font_path: Option<String>,
        anchor: Option<String>,
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;
        
        let font_path = font_path.as_ref().map(|p| std::path::Path::new(p));
        let text_anchor = anchor.as_ref().and_then(|s| crate::text::styles::TextAnchor::from_str(s));

        Python::with_gil(|py| {
            py.allow_threads(|| text::draw_text(image, text, x, y, scale as f32, color, font_path, text_anchor))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    pub fn draw_text_styled_impl(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_path: Option<String>,
        background: Option<(u8, u8, u8, u8)>,
        align: Option<String>,
        outline: Option<(u8, u8, u8, u8, f32)>,
        shadow: Option<(i32, i32, u8, u8, u8, u8)>,
        opacity: Option<f32>,
        max_width: Option<u32>,
        rotation: Option<f32>,
        anchor: Option<String>,
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        let font_path = font_path.as_ref().map(|p| std::path::Path::new(p));
        let text_anchor = anchor.as_ref().and_then(|s| crate::text::styles::TextAnchor::from_str(s));

        // Create TextStyle from parameters
        let mut style = crate::text::styles::TextStyle::new()
            .with_size(size)
            .with_color(color.0, color.1, color.2, color.3);

        if let Some(bg) = background {
            style = style.with_background(bg.0, bg.1, bg.2, bg.3);
        }

        if let Some(align_str) = align {
            let text_align = match align_str.as_str() {
                "center" => crate::text::styles::TextAlign::Center,
                "right" => crate::text::styles::TextAlign::Right,
                _ => crate::text::styles::TextAlign::Left,
            };
            style = style.with_align(text_align);
        }

        if let Some((or, og, ob, oa, width)) = outline {
            style = style.with_outline(or, og, ob, oa, width);
        }

        if let Some((sx, sy, sr, sg, sb, sa)) = shadow {
            style = style.with_shadow(sx, sy, sr, sg, sb, sa);
        }

        if let Some(opacity) = opacity {
            style = style.with_opacity(opacity);
        }

        if let Some(max_width) = max_width {
            style = style.with_max_width(max_width);
        }

        if let Some(rotation) = rotation {
            style = style.with_rotation(rotation);
        }

        Python::with_gil(|py| {
            py.allow_threads(|| text::draw_text_styled(image, text, x, y, &style, font_path, text_anchor))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    pub fn draw_text_multiline_impl(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_path: Option<String>,
        line_spacing: Option<f32>,
        align: Option<String>,
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        let font_path = font_path.as_ref().map(|p| std::path::Path::new(p));

        let mut style = crate::text::styles::TextStyle::new()
            .with_size(size)
            .with_color(color.0, color.1, color.2, color.3);

        if let Some(line_spacing) = line_spacing {
            style.line_spacing = line_spacing;
        }

        if let Some(align_str) = align {
            let text_align = match align_str.as_str() {
                "center" => crate::text::styles::TextAlign::Center,
                "right" => crate::text::styles::TextAlign::Right,
                _ => crate::text::styles::TextAlign::Left,
            };
            style = style.with_align(text_align);
        }

        Python::with_gil(|py| {
            py.allow_threads(|| text::draw_text_multiline(image, text, x, y, &style, font_path))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    pub fn draw_text_centered_impl(
        &mut self,
        text: &str,
        y: i32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_path: Option<String>,
        background: Option<(u8, u8, u8, u8)>,
        outline: Option<(u8, u8, u8, u8, f32)>,
        shadow: Option<(i32, i32, u8, u8, u8, u8)>,
        opacity: Option<f32>,
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        let font_path = font_path.as_ref().map(|p| std::path::Path::new(p));

        let mut style = crate::text::styles::TextStyle::new()
            .with_size(size)
            .with_color(color.0, color.1, color.2, color.3)
            .with_align(crate::text::styles::TextAlign::Center);

        if let Some(bg) = background {
            style = style.with_background(bg.0, bg.1, bg.2, bg.3);
        }

        if let Some((or, og, ob, oa, width)) = outline {
            style = style.with_outline(or, og, ob, oa, width);
        }

        if let Some((sx, sy, sr, sg, sb, sa)) = shadow {
            style = style.with_shadow(sx, sy, sr, sg, sb, sa);
        }

        if let Some(opacity) = opacity {
            style = style.with_opacity(opacity);
        }

        Python::with_gil(|py| {
            py.allow_threads(|| text::draw_text_centered(image, text, y, &style, font_path))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }

    pub fn get_text_size_impl(
        &mut self,
        text: &str,
        size: f32,
        font_path: Option<String>,
    ) -> PyResult<(u32, u32, i32, i32)> {
        let font_path = font_path.as_ref().map(|p| std::path::Path::new(p));

        Python::with_gil(|py| {
            py.allow_threads(|| text::get_text_size(text, size, font_path))
        })
        .map_err(|e| e.into())
    }

    pub fn get_multiline_text_size_impl(
        &mut self,
        text: &str,
        size: f32,
        line_spacing: f32,
        font_path: Option<String>,
    ) -> PyResult<(u32, u32, usize)> {
        let font_path = font_path.as_ref().map(|p| std::path::Path::new(p));

        Python::with_gil(|py| {
            py.allow_threads(|| text::get_multiline_text_size(text, size, line_spacing, font_path))
        })
        .map_err(|e| e.into())
    }

    pub fn get_text_box_impl(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        size: f32,
        font_path: Option<String>,
    ) -> PyResult<pyo3::PyObject> {
        let font_path = font_path.as_ref().map(|p| std::path::Path::new(p));

        Python::with_gil(|py| {
            let text_box = py.allow_threads(|| text::get_text_box(text, x, y, size, font_path))?;

            let dict = pyo3::types::PyDict::new(py);
            dict.set_item("x", text_box.x)?;
            dict.set_item("y", text_box.y)?;
            dict.set_item("width", text_box.width)?;
            dict.set_item("height", text_box.height)?;
            dict.set_item("ascent", text_box.ascent)?;
            dict.set_item("descent", text_box.descent)?;
            dict.set_item("baseline_y", text_box.baseline_y)?;
            dict.set_item("bottom_y", text_box.bottom_y)?;
            dict.set_item("right_x", text_box.right_x)?;

            Ok(dict.into_pyobject(py).unwrap().unbind().into_any())
        })
        .map_err(|e: crate::errors::ImgrsError| e.into())
    }

    pub fn draw_text_box_impl(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        size: f32,
        color: (u8, u8, u8, u8),
        font_path: Option<String>,
        background: Option<(u8, u8, u8, u8)>,
        align: Option<String>,
        vertical_align: Option<String>,
        line_spacing: Option<f32>,
        overflow: Option<bool>,
    ) -> PyResult<Self> {
        let format = self.format;
        let image = self.get_image()?;

        let font_path = font_path.as_ref().map(|p| std::path::Path::new(p));

        // Create TextStyle
        let mut text_style = crate::text::styles::TextStyle::new()
            .with_size(size)
            .with_color(color.0, color.1, color.2, color.3);

        if let Some(bg) = background {
            text_style = text_style.with_background(bg.0, bg.1, bg.2, bg.3);
        }

        if let Some(align_str) = align {
            let text_align = match align_str.as_str() {
                "center" => crate::text::styles::TextAlign::Center,
                "right" => crate::text::styles::TextAlign::Right,
                _ => crate::text::styles::TextAlign::Left,
            };
            text_style = text_style.with_align(text_align);
        }
        
        if let Some(spacing) = line_spacing {
            text_style.line_spacing = spacing;
        }

        // Create TextBoxStyle
        let v_align = if let Some(align_str) = vertical_align {
            match align_str.as_str() {
                "center" | "middle" => crate::text::styles::TextAlign::Center,
                "bottom" => crate::text::styles::TextAlign::Right, // Using Right for Bottom
                _ => crate::text::styles::TextAlign::Left, // Using Left for Top
            }
        } else {
            crate::text::styles::TextAlign::Left // Default Top
        };
        
        let style = crate::text::styles::TextBoxStyle {
            text_style,
            vertical_align: v_align,
            overflow: overflow.unwrap_or(false),
        };

        Python::with_gil(|py| {
            py.allow_threads(|| text::draw_text_box(image, text, x, y, width, height, &style, font_path))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format,
        })
        .map_err(|e| e.into())
    }
}

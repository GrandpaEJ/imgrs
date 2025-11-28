// Suppress PyO3 internal warnings about gil-refs (library issue, not ours)
#![allow(unexpected_cfgs)]
// Allow useless_conversion warnings from PyO3's PyResult type system
#![allow(clippy::useless_conversion)]

use pyo3::prelude::*;
use pyo3::types::PyModule;

mod blending;
mod css_filters;
mod drawing;
mod errors;
mod filters;
mod formats;
mod image;
mod metadata;
mod operations;
mod pixels;
mod shadows;
mod text;

pub use errors::ImgrsError;
pub use image::PyImage;

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyImage>()?;
    m.add(
        "ImgrsProcessingError",
        m.py().get_type::<errors::ImgrsProcessingError>(),
    )?;
    m.add(
        "InvalidImageError",
        m.py().get_type::<errors::InvalidImageError>(),
    )?;
    m.add(
        "UnsupportedFormatError",
        m.py().get_type::<errors::UnsupportedFormatError>(),
    )?;
    m.add(
        "ImgrsIOError",
        m.py().get_type::<errors::ImgrsIOError>(),
    )?;

    // Add shape creation functions
    m.add_function(wrap_pyfunction!(create_circle_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_rectangle_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_triangle_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_ellipse_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_star_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_square_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_diamond_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_hexagon_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_parallelogram_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_pentagon_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_octagon_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_heart_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_arrow_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_cross_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_quadrilateral_py, m)?)?;

    Ok(())
}

// Shape creation functions

#[pyfunction]
#[pyo3(signature = (size, color=(0, 0, 0, 255)))]
fn create_circle_py(size: u32, color: (u8, u8, u8, u8)) -> PyResult<PyImage> {
    PyImage::create_circle(size, color)
}

#[pyfunction]
#[pyo3(signature = (width, height, color=(0, 0, 0, 255)))]
fn create_rectangle_py(width: u32, height: u32, color: (u8, u8, u8, u8)) -> PyResult<PyImage> {
    PyImage::create_rectangle(width, height, color)
}

#[pyfunction]
#[pyo3(signature = (width, height, color=(0, 0, 0, 255)))]
fn create_triangle_py(width: u32, height: u32, color: (u8, u8, u8, u8)) -> PyResult<PyImage> {
    PyImage::create_triangle(width, height, color)
}

#[pyfunction]
#[pyo3(signature = (width, height, color=(0, 0, 0, 255)))]
fn create_ellipse_py(width: u32, height: u32, color: (u8, u8, u8, u8)) -> PyResult<PyImage> {
    PyImage::create_ellipse(width, height, color)
}

#[pyfunction]
#[pyo3(signature = (size, color=(0, 0, 0, 255)))]
fn create_star_py(size: u32, color: (u8, u8, u8, u8)) -> PyResult<PyImage> {
    PyImage::create_star(size, color)
}

#[pyfunction]
#[pyo3(signature = (size, color=(0, 0, 0, 255)))]
fn create_square_py(size: u32, color: (u8, u8, u8, u8)) -> PyResult<PyImage> {
    PyImage::create_square(size, color)
}

#[pyfunction]
#[pyo3(signature = (size, color=(0, 0, 0, 255)))]
fn create_diamond_py(size: u32, color: (u8, u8, u8, u8)) -> PyResult<PyImage> {
    PyImage::create_diamond(size, color)
}

#[pyfunction]
#[pyo3(signature = (size, color=(0, 0, 0, 255)))]
fn create_hexagon_py(size: u32, color: (u8, u8, u8, u8)) -> PyResult<PyImage> {
    PyImage::create_hexagon(size, color)
}

#[pyfunction]
#[pyo3(signature = (width, height, skew=0.2, color=(0, 0, 0, 255)))]
fn create_parallelogram_py(
    width: u32,
    height: u32,
    skew: f32,
    color: (u8, u8, u8, u8),
) -> PyResult<PyImage> {
    PyImage::create_parallelogram(width, height, skew, color)
}

#[pyfunction]
#[pyo3(signature = (size, color=(0, 0, 0, 255)))]
fn create_pentagon_py(size: u32, color: (u8, u8, u8, u8)) -> PyResult<PyImage> {
    PyImage::create_pentagon(size, color)
}

#[pyfunction]
#[pyo3(signature = (size, color=(0, 0, 0, 255)))]
fn create_octagon_py(size: u32, color: (u8, u8, u8, u8)) -> PyResult<PyImage> {
    PyImage::create_octagon(size, color)
}

#[pyfunction]
#[pyo3(signature = (size, color=(0, 0, 0, 255)))]
fn create_heart_py(size: u32, color: (u8, u8, u8, u8)) -> PyResult<PyImage> {
    PyImage::create_heart(size, color)
}

#[pyfunction]
#[pyo3(signature = (width, height, color=(0, 0, 0, 255)))]
fn create_arrow_py(width: u32, height: u32, color: (u8, u8, u8, u8)) -> PyResult<PyImage> {
    PyImage::create_arrow(width, height, color)
}

#[pyfunction]
#[pyo3(signature = (size, color=(0, 0, 0, 255)))]
fn create_cross_py(size: u32, color: (u8, u8, u8, u8)) -> PyResult<PyImage> {
    PyImage::create_cross(size, color)
}

#[pyfunction]
#[pyo3(signature = (p1, p2, p3, p4, color=(0, 0, 0, 255)))]
fn create_quadrilateral_py(
    p1: (i32, i32),
    p2: (i32, i32),
    p3: (i32, i32),
    p4: (i32, i32),
    color: (u8, u8, u8, u8),
) -> PyResult<PyImage> {
    PyImage::create_quadrilateral(p1, p2, p3, p4, color)
}

use pyo3::prelude::*;
use pyo3::types::PyModule;

mod blending;
mod css_filters;
mod drawing;
mod errors;
mod filters;
mod image;
mod formats;
mod operations;
mod pixels;
mod shadows;

pub use errors::ImgrsError;
pub use image::PyImage;

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyImage>()?;
    m.add("ImgrsProcessingError", m.py().get_type_bound::<errors::ImgrsProcessingError>())?;
    m.add("InvalidImageError", m.py().get_type_bound::<errors::InvalidImageError>())?;
    m.add("UnsupportedFormatError", m.py().get_type_bound::<errors::UnsupportedFormatError>())?;
    m.add("ImgrsIOError", m.py().get_type_bound::<errors::ImgrsIOError>())?;
    Ok(())
}

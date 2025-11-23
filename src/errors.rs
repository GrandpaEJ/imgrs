use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImgrsError {
    #[error("Invalid image data: {0}")]
    InvalidImage(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Python error: {0}")]
    Py(#[from] PyErr),
}

impl From<ImgrsError> for PyErr {
    fn from(err: ImgrsError) -> PyErr {
        match err {
            ImgrsError::InvalidImage(msg) => InvalidImageError::new_err(msg),
            ImgrsError::UnsupportedFormat(msg) => UnsupportedFormatError::new_err(msg),
            ImgrsError::Io(err) => ImgrsIOError::new_err(err.to_string()),
            ImgrsError::ImageError(err) => ImgrsProcessingError::new_err(err.to_string()),
            ImgrsError::InvalidOperation(msg) => ImgrsProcessingError::new_err(msg),
            ImgrsError::Py(err) => err,
        }
    }
}

// Python exception types
pyo3::create_exception!(imgrs_core, ImgrsProcessingError, PyException);
pyo3::create_exception!(imgrs_core, InvalidImageError, ImgrsProcessingError);
pyo3::create_exception!(imgrs_core, UnsupportedFormatError, ImgrsProcessingError);
pyo3::create_exception!(imgrs_core, ImgrsIOError, ImgrsProcessingError);

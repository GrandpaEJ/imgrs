use pyo3::prelude::*;
use pyo3::types::PyBytes;
use image::ImageFormat;
use crate::errors::ImgrsError;
use crate::formats;
use super::core::PyImage;

impl PyImage {
    pub fn save_impl(&mut self, path_or_buffer: &Bound<'_, PyAny>, format: Option<String>) -> PyResult<()> {
        if let Ok(path) = path_or_buffer.extract::<String>() {
            // Save to file path
            let save_format = if let Some(fmt) = format {
                formats::parse_format(&fmt)?
            } else {
                ImageFormat::from_path(&path)
                    .map_err(|_| ImgrsError::UnsupportedFormat(
                        "Cannot determine format from path".to_string()
                    ))?
            };
            
            // Ensure image is loaded before saving
            let image = self.get_image()?;
            
            Python::with_gil(|py| {
                py.allow_threads(|| {
                    image.save_with_format(&path, save_format)
                        .map_err(ImgrsError::ImageError)
                        .map_err(|e| e.into())
                })
            })
        } else {
            Err(ImgrsError::InvalidOperation(
                "Buffer saving not yet implemented".to_string()
            ).into())
        }
    }

    pub fn to_bytes_impl(&mut self) -> PyResult<Py<PyBytes>> {
        let image = self.get_image()?;
        Python::with_gil(|py| {
            let bytes = py.allow_threads(|| {
                image.as_bytes().to_vec()
            });
            Ok(PyBytes::new_bound(py, &bytes).into())
        })
    }
}


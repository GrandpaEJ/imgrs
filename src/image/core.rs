use crate::errors::ImgrsError;
use image::{ColorType, DynamicImage, ImageFormat};
use pyo3::prelude::*;
use std::io::Cursor;
use std::path::PathBuf;

/// Convert ColorType to PIL-compatible mode string
pub fn color_type_to_mode_string(color_type: ColorType) -> String {
    match color_type {
        ColorType::L8 => "L".to_string(),
        ColorType::La8 => "LA".to_string(),
        ColorType::Rgb8 => "RGB".to_string(),
        ColorType::Rgba8 => "RGBA".to_string(),
        ColorType::L16 => "I".to_string(),
        ColorType::La16 => "LA".to_string(),
        ColorType::Rgb16 => "RGB".to_string(),
        ColorType::Rgba16 => "RGBA".to_string(),
        ColorType::Rgb32F => "RGB".to_string(),
        ColorType::Rgba32F => "RGBA".to_string(),
        _ => "RGB".to_string(), // Default fallback
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum LazyImage {
    Loaded(DynamicImage),
    /// Image data stored as file path
    Path {
        path: PathBuf,
    },
    /// Image data stored as bytes
    Bytes {
        data: Vec<u8>,
    },
}

impl LazyImage {
    /// Ensure the image is loaded
    pub fn ensure_loaded(&mut self) -> Result<&DynamicImage, ImgrsError> {
        match self {
            LazyImage::Loaded(img) => Ok(img),
            LazyImage::Path { path } => {
                let img = image::open(path).map_err(ImgrsError::ImageError)?;
                *self = LazyImage::Loaded(img);
                match self {
                    LazyImage::Loaded(img) => Ok(img),
                    _ => unreachable!("Just set to Loaded variant"),
                }
            }
            LazyImage::Bytes { data } => {
                let cursor = Cursor::new(data);
                let reader = image::ImageReader::new(cursor)
                    .with_guessed_format()
                    .map_err(ImgrsError::Io)?;
                let img = reader.decode().map_err(ImgrsError::ImageError)?;
                *self = LazyImage::Loaded(img);
                match self {
                    LazyImage::Loaded(img) => Ok(img),
                    _ => unreachable!("Just set to Loaded variant"),
                }
            }
        }
    }
}

#[derive(Clone)]
#[pyclass(name = "RustImage")]
pub struct PyImage {
    pub(crate) lazy_image: LazyImage,
    pub(crate) format: Option<ImageFormat>,
}

impl PyImage {
    pub fn get_image(&mut self) -> Result<&DynamicImage, ImgrsError> {
        self.lazy_image.ensure_loaded()
    }

    pub fn get_image_mut(&mut self) -> Result<&mut DynamicImage, ImgrsError> {
        self.lazy_image.ensure_loaded()?;
        match self.lazy_image {
            LazyImage::Loaded(ref mut img) => Ok(img),
            _ => unreachable!("ensure_loaded should have converted to Loaded"),
        }
    }

    pub fn new_from_image(image: DynamicImage, format: Option<ImageFormat>) -> Self {
        PyImage {
            lazy_image: LazyImage::Loaded(image),
            format,
        }
    }

    // Shape creation methods
    pub fn create_circle(size: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| py.allow_threads(|| drawing::create_circle(size, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format: None,
            })
            .map_err(|e| e.into())
    }

    pub fn create_rectangle(width: u32, height: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| py.allow_threads(|| drawing::create_rectangle(width, height, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format: None,
            })
            .map_err(|e| e.into())
    }

    pub fn create_triangle(width: u32, height: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| py.allow_threads(|| drawing::create_triangle(width, height, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format: None,
            })
            .map_err(|e| e.into())
    }

    pub fn create_ellipse(width: u32, height: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| py.allow_threads(|| drawing::create_ellipse(width, height, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format: None,
            })
            .map_err(|e| e.into())
    }

    pub fn create_star(size: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| py.allow_threads(|| drawing::create_star(size, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format: None,
            })
            .map_err(|e| e.into())
    }

    pub fn create_square(size: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| py.allow_threads(|| drawing::create_square(size, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format: None,
            })
            .map_err(|e| e.into())
    }

    pub fn create_diamond(size: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| py.allow_threads(|| drawing::create_diamond(size, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format: None,
            })
            .map_err(|e| e.into())
    }

    pub fn create_hexagon(size: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| py.allow_threads(|| drawing::create_hexagon(size, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format: None,
            })
            .map_err(|e| e.into())
    }

    pub fn create_parallelogram(
        width: u32,
        height: u32,
        skew: f32,
        color: (u8, u8, u8, u8),
    ) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| {
            py.allow_threads(|| drawing::create_parallelogram(width, height, skew, color))
        })
        .map(|result| PyImage {
            lazy_image: LazyImage::Loaded(result),
            format: None,
        })
        .map_err(|e| e.into())
    }

    pub fn create_pentagon(size: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| py.allow_threads(|| drawing::create_pentagon(size, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format: None,
            })
            .map_err(|e| e.into())
    }

    pub fn create_octagon(size: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| py.allow_threads(|| drawing::create_octagon(size, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format: None,
            })
            .map_err(|e| e.into())
    }

    pub fn create_heart(size: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| py.allow_threads(|| drawing::create_heart(size, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format: None,
            })
            .map_err(|e| e.into())
    }

    pub fn create_arrow(width: u32, height: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| py.allow_threads(|| drawing::create_arrow(width, height, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format: None,
            })
            .map_err(|e| e.into())
    }

    pub fn create_cross(size: u32, color: (u8, u8, u8, u8)) -> PyResult<Self> {
        use crate::drawing;
        Python::with_gil(|py| py.allow_threads(|| drawing::create_cross(size, color)))
            .map(|result| PyImage {
                lazy_image: LazyImage::Loaded(result),
                format: None,
            })
            .map_err(|e| e.into())
    }
}

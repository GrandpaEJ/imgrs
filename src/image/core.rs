use pyo3::prelude::*;
use image::{DynamicImage, ImageFormat, ColorType};
use std::io::Cursor;
use std::path::PathBuf;
use crate::errors::ImgrsError;

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
pub enum LazyImage {
    Loaded(DynamicImage),
    /// Image data stored as file path
    Path { path: PathBuf },
    /// Image data stored as bytes
    Bytes { data: Vec<u8> },
}

impl LazyImage {
    /// Ensure the image is loaded
    pub fn ensure_loaded(&mut self) -> Result<&DynamicImage, ImgrsError> {
        match self {
            LazyImage::Loaded(img) => Ok(img),
            LazyImage::Path { path } => {
                let img = image::open(path)
                    .map_err(ImgrsError::ImageError)?;
                *self = LazyImage::Loaded(img);
                match self {
                    LazyImage::Loaded(img) => Ok(img),
                    _ => unreachable!("Just set to Loaded variant")
                }
            }
            LazyImage::Bytes { data } => {
                let cursor = Cursor::new(data);
                let reader = image::ImageReader::new(cursor).with_guessed_format()
                    .map_err(ImgrsError::Io)?;
                let img = reader.decode()
                    .map_err(ImgrsError::ImageError)?;
                *self = LazyImage::Loaded(img);
                match self {
                    LazyImage::Loaded(img) => Ok(img),
                    _ => unreachable!("Just set to Loaded variant")
                }
            }
        }
    }
}

#[derive(Clone)]
#[pyclass(name = "Image")]
pub struct PyImage {
    pub(crate) lazy_image: LazyImage,
    pub(crate) format: Option<ImageFormat>,
}

impl PyImage {
    pub fn get_image(&mut self) -> Result<&DynamicImage, ImgrsError> {
        self.lazy_image.ensure_loaded()
    }
    
    pub fn new_from_image(image: DynamicImage, format: Option<ImageFormat>) -> Self {
        PyImage {
            lazy_image: LazyImage::Loaded(image),
            format,
        }
    }
}


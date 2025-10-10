use pyo3::prelude::*;
use pyo3::types::PyBytes;
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;
use std::path::PathBuf;
use numpy::{PyArray2, PyArray3, PyArrayMethods, PyUntypedArrayMethods};
use crate::errors::ImgrsError;
use super::core::{PyImage, LazyImage};

impl PyImage {
    pub fn new_default() -> Self {
        // Create a default 1x1 RGB image for compatibility
        let image = DynamicImage::new_rgb8(1, 1);
        PyImage { 
            lazy_image: LazyImage::Loaded(image), 
            format: None 
        }
    }

    pub fn new_with_mode(mode: &str, size: (u32, u32), color: Option<(u8, u8, u8, u8)>) -> PyResult<Self> {
        let (width, height) = size;
        
        if width == 0 || height == 0 {
            return Err(ImgrsError::InvalidOperation(
                "Image dimensions must be greater than 0".to_string()
            ).into());
        }
        
        let image = match mode {
            "RGB" => {
                let (r, g, b, _) = color.unwrap_or((0, 0, 0, 255));
                let mut img = image::RgbImage::new(width, height);
                for pixel in img.pixels_mut() {
                    *pixel = image::Rgb([r, g, b]);
                }
                DynamicImage::ImageRgb8(img)
            }
            "RGBA" => {
                let (r, g, b, a) = color.unwrap_or((0, 0, 0, 255));
                let mut img = image::RgbaImage::new(width, height);
                for pixel in img.pixels_mut() {
                    *pixel = image::Rgba([r, g, b, a]);
                }
                DynamicImage::ImageRgba8(img)
            }
            "L" => {
                let (gray, _, _, _) = color.unwrap_or((0, 0, 0, 255));
                let mut img = image::GrayImage::new(width, height);
                for pixel in img.pixels_mut() {
                    *pixel = image::Luma([gray]);
                }
                DynamicImage::ImageLuma8(img)
            }
            _ => {
                return Err(ImgrsError::UnsupportedFormat(
                    format!("Unsupported mode: {}. Use 'RGB', 'RGBA', or 'L'", mode)
                ).into());
            }
        };

        Ok(PyImage {
            lazy_image: LazyImage::Loaded(image),
            format: None,
        })
    }

    pub fn open_impl(path_or_bytes: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(path) = path_or_bytes.extract::<String>() {
            // Eager loading - load image immediately like Pillow
            let path_buf = PathBuf::from(&path);
            let format = ImageFormat::from_path(&path).ok();
            
            let image = Python::with_gil(|py| {
                py.allow_threads(|| {
                    image::open(&path_buf)
                        .map_err(ImgrsError::ImageError)
                })
            })?;
            
            Ok(PyImage {
                lazy_image: LazyImage::Loaded(image),
                format
            })
        } else if let Ok(bytes) = path_or_bytes.downcast::<PyBytes>() {
            // Eager loading from bytes
            let data = bytes.as_bytes();
            
            let (image, format) = Python::with_gil(|py| {
                py.allow_threads(|| {
                    let cursor = Cursor::new(data);
                    let reader = image::ImageReader::new(cursor)
                        .with_guessed_format()
                        .map_err(ImgrsError::Io)?;
                    let fmt = reader.format();
                    let img = reader.decode().map_err(ImgrsError::ImageError)?;
                    Ok::<_, ImgrsError>((img, fmt))
                })
            })?;
            
            Ok(PyImage {
                lazy_image: LazyImage::Loaded(image),
                format
            })
        } else {
            Err(ImgrsError::InvalidOperation(
                "Expected file path (str) or bytes".to_string()
            ).into())
        }
    }

    pub fn fromarray_impl(array: &Bound<'_, PyAny>, _mode: Option<&str>) -> PyResult<Self> {
        // Try to handle 2D array (grayscale)
        if let Ok(array_2d) = array.downcast::<PyArray2<u8>>() {
            let readonly = array_2d.readonly();
            let shape = readonly.shape();
            let height = shape[0] as u32;
            let width = shape[1] as u32;

            let data: Vec<u8> = readonly.as_slice()?.to_vec();

            let image = image::GrayImage::from_raw(width, height, data)
                .ok_or_else(|| ImgrsError::InvalidOperation(
                    "Failed to create image from array data".to_string()
                ))?;

            return Ok(PyImage {
                lazy_image: LazyImage::Loaded(DynamicImage::ImageLuma8(image)),
                format: None,
            });
        }

        // Try to handle 3D array (RGB/RGBA)
        if let Ok(array_3d) = array.downcast::<PyArray3<u8>>() {
            let readonly = array_3d.readonly();
            let shape = readonly.shape();
            let height = shape[0] as u32;
            let width = shape[1] as u32;
            let channels = shape[2];

            let data = readonly.as_slice()?;

            match channels {
                3 => {
                    // RGB image
                    let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);
                    for i in 0..(width * height) as usize {
                        rgb_data.push(data[i * 3]);     // R
                        rgb_data.push(data[i * 3 + 1]); // G
                        rgb_data.push(data[i * 3 + 2]); // B
                    }

                    let image = image::RgbImage::from_raw(width, height, rgb_data)
                        .ok_or_else(|| ImgrsError::InvalidOperation(
                            "Failed to create RGB image from array data".to_string()
                        ))?;

                    Ok(PyImage {
                        lazy_image: LazyImage::Loaded(DynamicImage::ImageRgb8(image)),
                        format: None,
                    })
                }
                4 => {
                    // RGBA image
                    let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);
                    for i in 0..(width * height) as usize {
                        rgba_data.push(data[i * 4]);     // R
                        rgba_data.push(data[i * 4 + 1]); // G
                        rgba_data.push(data[i * 4 + 2]); // B
                        rgba_data.push(data[i * 4 + 3]); // A
                    }

                    let image = image::RgbaImage::from_raw(width, height, rgba_data)
                        .ok_or_else(|| ImgrsError::InvalidOperation(
                            "Failed to create RGBA image from array data".to_string()
                        ))?;

                    Ok(PyImage {
                        lazy_image: LazyImage::Loaded(DynamicImage::ImageRgba8(image)),
                        format: None,
                    })
                }
                _ => Err(ImgrsError::InvalidOperation(
                    format!("Unsupported number of channels: {}. Expected 3 (RGB) or 4 (RGBA)", channels)
                ).into())
            }
        } else {
            Err(ImgrsError::InvalidOperation(
                "Expected numpy array with shape (H, W) for grayscale or (H, W, C) for RGB/RGBA".to_string()
            ).into())
        }
    }

    /// Create image from raw bytes (no NumPy needed!)
    /// Mobile-friendly alternative to fromarray()
    pub fn frombytes_impl(
        mode: &str,
        size: (u32, u32),
        data: &[u8],
    ) -> PyResult<Self> {
        let (width, height) = size;
        
        if width == 0 || height == 0 {
            return Err(ImgrsError::InvalidOperation(
                "Image dimensions must be greater than 0".to_string()
            ).into());
        }

        let image = match mode {
            "RGB" => {
                let expected_len = (width * height * 3) as usize;
                if data.len() != expected_len {
                    return Err(ImgrsError::InvalidOperation(
                        format!("Expected {} bytes for {}x{} RGB, got {}", expected_len, width, height, data.len())
                    ).into());
                }
                
                let rgb_image = image::RgbImage::from_raw(width, height, data.to_vec())
                    .ok_or_else(|| ImgrsError::InvalidOperation(
                        "Failed to create RGB image from bytes".to_string()
                    ))?;
                
                DynamicImage::ImageRgb8(rgb_image)
            }
            "RGBA" => {
                let expected_len = (width * height * 4) as usize;
                if data.len() != expected_len {
                    return Err(ImgrsError::InvalidOperation(
                        format!("Expected {} bytes for {}x{} RGBA, got {}", expected_len, width, height, data.len())
                    ).into());
                }
                
                let rgba_image = image::RgbaImage::from_raw(width, height, data.to_vec())
                    .ok_or_else(|| ImgrsError::InvalidOperation(
                        "Failed to create RGBA image from bytes".to_string()
                    ))?;
                
                DynamicImage::ImageRgba8(rgba_image)
            }
            "L" => {
                let expected_len = (width * height) as usize;
                if data.len() != expected_len {
                    return Err(ImgrsError::InvalidOperation(
                        format!("Expected {} bytes for {}x{} grayscale, got {}", expected_len, width, height, data.len())
                    ).into());
                }
                
                let gray_image = image::GrayImage::from_raw(width, height, data.to_vec())
                    .ok_or_else(|| ImgrsError::InvalidOperation(
                        "Failed to create grayscale image from bytes".to_string()
                    ))?;
                
                DynamicImage::ImageLuma8(gray_image)
            }
            _ => {
                return Err(ImgrsError::UnsupportedFormat(
                    format!("Unsupported mode: {}. Use 'RGB', 'RGBA', or 'L'", mode)
                ).into());
            }
        };

        Ok(PyImage {
            lazy_image: LazyImage::Loaded(image),
            format: None,
        })
    }
}

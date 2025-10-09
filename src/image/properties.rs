use pyo3::prelude::*;
use super::core::{PyImage, color_type_to_mode_string};

impl PyImage {
    pub fn size_impl(&mut self) -> PyResult<(u32, u32)> {
        let img = self.get_image()?;
        Ok((img.width(), img.height()))
    }

    pub fn width_impl(&mut self) -> PyResult<u32> {
        let img = self.get_image()?;
        Ok(img.width())
    }

    pub fn height_impl(&mut self) -> PyResult<u32> {
        let img = self.get_image()?;
        Ok(img.height())
    }

    pub fn mode_impl(&mut self) -> PyResult<String> {
        let img = self.get_image()?;
        Ok(color_type_to_mode_string(img.color()))
    }

    pub fn format_impl(&self) -> Option<String> {
        self.format.map(|f| format!("{:?}", f).to_uppercase())
    }

    pub fn repr_impl(&mut self) -> String {
        match self.get_image() {
            Ok(img) => {
                let (width, height) = (img.width(), img.height());
                let mode = color_type_to_mode_string(img.color());
                let format = self.format_impl().unwrap_or_else(|| "Unknown".to_string());
                format!("<Image size={}x{} mode={} format={}>", width, height, mode, format)
            },
            Err(_) => "<Image [Error loading image]>".to_string(),
        }
    }
}


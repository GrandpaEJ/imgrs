use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::metadata;
use super::core::PyImage;

impl PyImage {
    /// Get EXIF/metadata from image file
    pub fn get_metadata_impl(&mut self, path: String) -> PyResult<Py<PyDict>> {
        Python::with_gil(|py| {
            let metadata = metadata::read_exif_from_path(&path)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
            
            let dict = PyDict::new_bound(py);
            
            // Basic info
            dict.set_item("width", metadata.basic.width)?;
            dict.set_item("height", metadata.basic.height)?;
            
            if let Some(fmt) = metadata.basic.format {
                dict.set_item("format", fmt)?;
            }
            
            // EXIF data
            if let Some(exif) = metadata.exif {
                let exif_dict = PyDict::new_bound(py);
                
                if let Some(make) = exif.make {
                    exif_dict.set_item("make", make)?;
                }
                if let Some(model) = exif.model {
                    exif_dict.set_item("model", model)?;
                }
                if let Some(dt) = exif.date_time {
                    exif_dict.set_item("datetime", dt)?;
                }
                if let Some(artist) = exif.artist {
                    exif_dict.set_item("artist", artist)?;
                }
                if let Some(copyright) = exif.copyright {
                    exif_dict.set_item("copyright", copyright)?;
                }
                if let Some(orientation) = exif.orientation {
                    exif_dict.set_item("orientation", orientation)?;
                }
                
                dict.set_item("exif", exif_dict)?;
            }
            
            // GPS data
            if let Some(gps) = metadata.gps {
                let gps_dict = PyDict::new_bound(py);
                
                if let Some(lat) = gps.latitude {
                    gps_dict.set_item("latitude", lat)?;
                }
                if let Some(lon) = gps.longitude {
                    gps_dict.set_item("longitude", lon)?;
                }
                if let Some(alt) = gps.altitude {
                    gps_dict.set_item("altitude", alt)?;
                }
                
                dict.set_item("gps", gps_dict)?;
            }
            
            // Camera info
            if let Some(camera) = metadata.camera {
                let camera_dict = PyDict::new_bound(py);
                
                if let Some(iso) = camera.iso {
                    camera_dict.set_item("iso", iso)?;
                }
                if let Some(exp) = camera.exposure_time {
                    camera_dict.set_item("exposure_time", exp)?;
                }
                if let Some(f_num) = camera.f_number {
                    camera_dict.set_item("f_number", f_num)?;
                }
                if let Some(focal) = camera.focal_length {
                    camera_dict.set_item("focal_length", focal)?;
                }
                if let Some(wb) = camera.white_balance {
                    camera_dict.set_item("white_balance", wb)?;
                }
                
                dict.set_item("camera", camera_dict)?;
            }
            
            Ok(dict.into())
        })
    }
    
    /// Get metadata summary string
    pub fn get_metadata_summary_impl(&mut self, path: String) -> PyResult<String> {
        let metadata = metadata::read_exif_from_path(&path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        
        Ok(metadata.summary())
    }
    
    /// Check if file has EXIF data
    pub fn has_exif_impl(&mut self, path: String) -> PyResult<bool> {
        let metadata = metadata::read_exif_from_path(&path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        
        Ok(metadata.has_exif())
    }
    
    /// Check if file has GPS data
    pub fn has_gps_impl(&mut self, path: String) -> PyResult<bool> {
        let metadata = metadata::read_exif_from_path(&path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        
        Ok(metadata.has_gps())
    }
}


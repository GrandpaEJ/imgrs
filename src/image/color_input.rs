use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};

#[derive(Debug, Clone, Copy)]
pub enum ColorInput {
    Rgba(u8, u8, u8, u8),
}

impl<'py> FromPyObject<'py> for ColorInput {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        // Try tuple first
        if let Ok(tuple) = ob.downcast::<PyTuple>() {
            if tuple.len() == 3 {
                let r: u8 = tuple.get_item(0)?.extract()?;
                let g: u8 = tuple.get_item(1)?.extract()?;
                let b: u8 = tuple.get_item(2)?.extract()?;
                return Ok(ColorInput::Rgba(r, g, b, 255));
            } else if tuple.len() == 4 {
                let r: u8 = tuple.get_item(0)?.extract()?;
                let g: u8 = tuple.get_item(1)?.extract()?;
                let b: u8 = tuple.get_item(2)?.extract()?;
                let a: u8 = tuple.get_item(3)?.extract()?;
                return Ok(ColorInput::Rgba(r, g, b, a));
            }
        }
        
        // Try string (hex)
        if let Ok(s) = ob.downcast::<PyString>() {
            let s_cow = s.to_string_lossy();
            let s_ref = s_cow.trim_start_matches('#');
            // println!("Parsing hex color: '{}' -> '{}'", s_cow, s_ref);
            
            if s_ref.len() == 6 {
                let r = u8::from_str_radix(&s_ref[0..2], 16).map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid hex color"))?;
                let g = u8::from_str_radix(&s_ref[2..4], 16).map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid hex color"))?;
                let b = u8::from_str_radix(&s_ref[4..6], 16).map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid hex color"))?;
                return Ok(ColorInput::Rgba(r, g, b, 255));
            } else if s_ref.len() == 8 {
                 let r = u8::from_str_radix(&s_ref[0..2], 16).map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid hex color"))?;
                let g = u8::from_str_radix(&s_ref[2..4], 16).map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid hex color"))?;
                let b = u8::from_str_radix(&s_ref[4..6], 16).map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid hex color"))?;
                let a = u8::from_str_radix(&s_ref[6..8], 16).map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid hex color"))?;
                return Ok(ColorInput::Rgba(r, g, b, a));
            }
            // Handle short hex #RGB -> RRGGBB
            if s_ref.len() == 3 {
                 let r_char = &s_ref[0..1];
                 let g_char = &s_ref[1..2];
                 let b_char = &s_ref[2..3];
                 let r = u8::from_str_radix(&format!("{}{}", r_char, r_char), 16).map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid hex color"))?;
                 let g = u8::from_str_radix(&format!("{}{}", g_char, g_char), 16).map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid hex color"))?;
                 let b = u8::from_str_radix(&format!("{}{}", b_char, b_char), 16).map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid hex color"))?;
                 return Ok(ColorInput::Rgba(r, g, b, 255));
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Expected color tuple (R, G, B) or (R, G, B, A) or hex string"))
    }
}

impl ColorInput {
    pub fn to_rgba(&self) -> (u8, u8, u8, u8) {
        match self {
            ColorInput::Rgba(r, g, b, a) => (*r, *g, *b, *a),
        }
    }
    
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            ColorInput::Rgba(r, g, b, _) => (*r, *g, *b),
        }
    }
}

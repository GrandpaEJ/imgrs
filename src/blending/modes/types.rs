/// Blend modes for compositing operations
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum BlendMode {
    // Standard blending modes
    Normal,
    Multiply,
    Screen,
    Overlay,
    SoftLight,
    HardLight,
    ColorDodge,
    ColorBurn,
    Darken,
    Lighten,
    Difference,
    Exclusion,

    // Source/Destination composite operations
    SourceOver,
    SourceIn,
    SourceOut,
    SourceAtop,
    DestinationOver,
    DestinationIn,
    DestinationOut,
    DestinationAtop,

    // Additional blending modes
    Lighter,
    Copy,
    Xor,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl BlendMode {
    /// Parse a blend mode from a string (CSS-style names)
    pub fn from_str(mode: &str) -> Result<Self, String> {
        match mode.to_lowercase().as_str() {
            "normal" => Ok(BlendMode::Normal),
            "multiply" => Ok(BlendMode::Multiply),
            "screen" => Ok(BlendMode::Screen),
            "overlay" => Ok(BlendMode::Overlay),
            "soft-light" | "softlight" => Ok(BlendMode::SoftLight),
            "hard-light" | "hardlight" => Ok(BlendMode::HardLight),
            "color-dodge" | "colordodge" => Ok(BlendMode::ColorDodge),
            "color-burn" | "colorburn" => Ok(BlendMode::ColorBurn),
            "darken" => Ok(BlendMode::Darken),
            "lighten" => Ok(BlendMode::Lighten),
            "difference" => Ok(BlendMode::Difference),
            "exclusion" => Ok(BlendMode::Exclusion),
            "source-over" | "sourceover" => Ok(BlendMode::SourceOver),
            "source-in" | "sourcein" => Ok(BlendMode::SourceIn),
            "source-out" | "sourceout" => Ok(BlendMode::SourceOut),
            "source-atop" | "sourceatop" => Ok(BlendMode::SourceAtop),
            "destination-over" | "destinationover" => Ok(BlendMode::DestinationOver),
            "destination-in" | "destinationin" => Ok(BlendMode::DestinationIn),
            "destination-out" | "destinationout" => Ok(BlendMode::DestinationOut),
            "destination-atop" | "destinationatop" => Ok(BlendMode::DestinationAtop),
            "lighter" => Ok(BlendMode::Lighter),
            "copy" => Ok(BlendMode::Copy),
            "xor" => Ok(BlendMode::Xor),
            "hue" => Ok(BlendMode::Hue),
            "saturation" => Ok(BlendMode::Saturation),
            "color" => Ok(BlendMode::Color),
            "luminosity" => Ok(BlendMode::Luminosity),
            _ => Err(format!("Unknown blend mode: {}", mode)),
        }
    }
}
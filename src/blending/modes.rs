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

/// Blend two pixels using the specified blend mode
#[allow(dead_code)]
pub fn blend_pixels(
    base: (u8, u8, u8, u8),
    overlay: (u8, u8, u8, u8),
    blend_mode: BlendMode,
    opacity: f32,
) -> (u8, u8, u8, u8) {
    let base_r = base.0 as f32 / 255.0;
    let base_g = base.1 as f32 / 255.0;
    let base_b = base.2 as f32 / 255.0;
    let base_a = base.3 as f32 / 255.0;

    let overlay_r = overlay.0 as f32 / 255.0;
    let overlay_g = overlay.1 as f32 / 255.0;
    let overlay_b = overlay.2 as f32 / 255.0;
    let overlay_a = (overlay.3 as f32 / 255.0) * opacity;

    // Handle source/destination composite operations
    match blend_mode {
        BlendMode::SourceOver => {
            // Default: source over destination
            let final_a = overlay_a + base_a * (1.0 - overlay_a);
            if final_a == 0.0 {
                return (0, 0, 0, 0);
            }
            let final_r = (overlay_r * overlay_a + base_r * base_a * (1.0 - overlay_a)) / final_a;
            let final_g = (overlay_g * overlay_a + base_g * base_a * (1.0 - overlay_a)) / final_a;
            let final_b = (overlay_b * overlay_a + base_b * base_a * (1.0 - overlay_a)) / final_a;
            return (
                (final_r * 255.0) as u8,
                (final_g * 255.0) as u8,
                (final_b * 255.0) as u8,
                (final_a * 255.0) as u8,
            );
        }
        BlendMode::SourceIn => {
            let final_a = overlay_a * base_a;
            if final_a == 0.0 {
                return (0, 0, 0, 0);
            }
            return (
                (overlay_r * 255.0) as u8,
                (overlay_g * 255.0) as u8,
                (overlay_b * 255.0) as u8,
                (final_a * 255.0) as u8,
            );
        }
        BlendMode::SourceOut => {
            let final_a = overlay_a * (1.0 - base_a);
            if final_a == 0.0 {
                return (0, 0, 0, 0);
            }
            return (
                (overlay_r * 255.0) as u8,
                (overlay_g * 255.0) as u8,
                (overlay_b * 255.0) as u8,
                (final_a * 255.0) as u8,
            );
        }
        BlendMode::SourceAtop => {
            let final_a = base_a;
            if final_a == 0.0 {
                return (0, 0, 0, 0);
            }
            let final_r = (overlay_r * overlay_a + base_r * base_a * (1.0 - overlay_a)) / final_a;
            let final_g = (overlay_g * overlay_a + base_g * base_a * (1.0 - overlay_a)) / final_a;
            let final_b = (overlay_b * overlay_a + base_b * base_a * (1.0 - overlay_a)) / final_a;
            return (
                (final_r * 255.0) as u8,
                (final_g * 255.0) as u8,
                (final_b * 255.0) as u8,
                (final_a * 255.0) as u8,
            );
        }
        BlendMode::DestinationOver => {
            let final_a = base_a + overlay_a * (1.0 - base_a);
            if final_a == 0.0 {
                return (0, 0, 0, 0);
            }
            let final_r = (base_r * base_a + overlay_r * overlay_a * (1.0 - base_a)) / final_a;
            let final_g = (base_g * base_a + overlay_g * overlay_a * (1.0 - base_a)) / final_a;
            let final_b = (base_b * base_a + overlay_b * overlay_a * (1.0 - base_a)) / final_a;
            return (
                (final_r * 255.0) as u8,
                (final_g * 255.0) as u8,
                (final_b * 255.0) as u8,
                (final_a * 255.0) as u8,
            );
        }
        BlendMode::DestinationIn => {
            let final_a = base_a * overlay_a;
            if final_a == 0.0 {
                return (0, 0, 0, 0);
            }
            return (
                (base_r * 255.0) as u8,
                (base_g * 255.0) as u8,
                (base_b * 255.0) as u8,
                (final_a * 255.0) as u8,
            );
        }
        BlendMode::DestinationOut => {
            let final_a = base_a * (1.0 - overlay_a);
            if final_a == 0.0 {
                return (0, 0, 0, 0);
            }
            return (
                (base_r * 255.0) as u8,
                (base_g * 255.0) as u8,
                (base_b * 255.0) as u8,
                (final_a * 255.0) as u8,
            );
        }
        BlendMode::DestinationAtop => {
            let final_a = overlay_a;
            if final_a == 0.0 {
                return (0, 0, 0, 0);
            }
            let final_r = (base_r * base_a + overlay_r * overlay_a * (1.0 - base_a)) / final_a;
            let final_g = (base_g * base_a + overlay_g * overlay_a * (1.0 - base_a)) / final_a;
            let final_b = (base_b * base_a + overlay_b * overlay_a * (1.0 - base_a)) / final_a;
            return (
                (final_r * 255.0) as u8,
                (final_g * 255.0) as u8,
                (final_b * 255.0) as u8,
                (final_a * 255.0) as u8,
            );
        }
        BlendMode::Copy => {
            return (
                (overlay_r * 255.0) as u8,
                (overlay_g * 255.0) as u8,
                (overlay_b * 255.0) as u8,
                (overlay_a * 255.0) as u8,
            );
        }
        BlendMode::Xor => {
            let final_a = overlay_a + base_a - 2.0 * overlay_a * base_a;
            if final_a == 0.0 {
                return (0, 0, 0, 0);
            }
            let final_r = (overlay_r * overlay_a * (1.0 - base_a)
                + base_r * base_a * (1.0 - overlay_a))
                / final_a;
            let final_g = (overlay_g * overlay_a * (1.0 - base_a)
                + base_g * base_a * (1.0 - overlay_a))
                / final_a;
            let final_b = (overlay_b * overlay_a * (1.0 - base_a)
                + base_b * base_a * (1.0 - overlay_a))
                / final_a;
            return (
                (final_r * 255.0) as u8,
                (final_g * 255.0) as u8,
                (final_b * 255.0) as u8,
                (final_a * 255.0) as u8,
            );
        }
        _ => {
            // For standard blending modes, apply color blending then composite
        }
    }

    // Standard blending modes with color operations
    let (blended_r, blended_g, blended_b) = match blend_mode {
        BlendMode::Normal => (overlay_r, overlay_g, overlay_b),
        BlendMode::Multiply => (base_r * overlay_r, base_g * overlay_g, base_b * overlay_b),
        BlendMode::Screen => (
            1.0 - (1.0 - base_r) * (1.0 - overlay_r),
            1.0 - (1.0 - base_g) * (1.0 - overlay_g),
            1.0 - (1.0 - base_b) * (1.0 - overlay_b),
        ),
        BlendMode::Overlay => (
            overlay_blend(base_r, overlay_r),
            overlay_blend(base_g, overlay_g),
            overlay_blend(base_b, overlay_b),
        ),
        BlendMode::SoftLight => (
            soft_light_blend(base_r, overlay_r),
            soft_light_blend(base_g, overlay_g),
            soft_light_blend(base_b, overlay_b),
        ),
        BlendMode::HardLight => (
            hard_light_blend(base_r, overlay_r),
            hard_light_blend(base_g, overlay_g),
            hard_light_blend(base_b, overlay_b),
        ),
        BlendMode::ColorDodge => (
            color_dodge_blend(base_r, overlay_r),
            color_dodge_blend(base_g, overlay_g),
            color_dodge_blend(base_b, overlay_b),
        ),
        BlendMode::ColorBurn => (
            color_burn_blend(base_r, overlay_r),
            color_burn_blend(base_g, overlay_g),
            color_burn_blend(base_b, overlay_b),
        ),
        BlendMode::Darken => (
            base_r.min(overlay_r),
            base_g.min(overlay_g),
            base_b.min(overlay_b),
        ),
        BlendMode::Lighten => (
            base_r.max(overlay_r),
            base_g.max(overlay_g),
            base_b.max(overlay_b),
        ),
        BlendMode::Difference => (
            (base_r - overlay_r).abs(),
            (base_g - overlay_g).abs(),
            (base_b - overlay_b).abs(),
        ),
        BlendMode::Exclusion => (
            base_r + overlay_r - 2.0 * base_r * overlay_r,
            base_g + overlay_g - 2.0 * base_g * overlay_g,
            base_b + overlay_b - 2.0 * base_b * overlay_b,
        ),
        BlendMode::Lighter => (base_r + overlay_r, base_g + overlay_g, base_b + overlay_b),
        BlendMode::Hue => {
            let (h, _, _) = rgb_to_hsl(overlay_r, overlay_g, overlay_b);
            let (_, s, l) = rgb_to_hsl(base_r, base_g, base_b);
            hsl_to_rgb(h, s, l)
        }
        BlendMode::Saturation => {
            let (_, s, _) = rgb_to_hsl(overlay_r, overlay_g, overlay_b);
            let (h, _, l) = rgb_to_hsl(base_r, base_g, base_b);
            hsl_to_rgb(h, s, l)
        }
        BlendMode::Color => {
            let (h, s, _) = rgb_to_hsl(overlay_r, overlay_g, overlay_b);
            let (_, _, l) = rgb_to_hsl(base_r, base_g, base_b);
            hsl_to_rgb(h, s, l)
        }
        BlendMode::Luminosity => {
            let (_, _, l) = rgb_to_hsl(overlay_r, overlay_g, overlay_b);
            let (h, s, _) = rgb_to_hsl(base_r, base_g, base_b);
            hsl_to_rgb(h, s, l)
        }
        _ => (overlay_r, overlay_g, overlay_b),
    };

    // Apply standard alpha compositing for blending modes
    let final_a = overlay_a + base_a * (1.0 - overlay_a);
    if final_a == 0.0 {
        return (0, 0, 0, 0);
    }

    let final_r = (blended_r * overlay_a + base_r * base_a * (1.0 - overlay_a)) / final_a;
    let final_g = (blended_g * overlay_a + base_g * base_a * (1.0 - overlay_a)) / final_a;
    let final_b = (blended_b * overlay_a + base_b * base_a * (1.0 - overlay_a)) / final_a;

    (
        (final_r.min(1.0) * 255.0) as u8,
        (final_g.min(1.0) * 255.0) as u8,
        (final_b.min(1.0) * 255.0) as u8,
        (final_a * 255.0) as u8,
    )
}

/// Overlay blend function
fn overlay_blend(base: f32, overlay: f32) -> f32 {
    if base < 0.5 {
        2.0 * base * overlay
    } else {
        1.0 - 2.0 * (1.0 - base) * (1.0 - overlay)
    }
}

/// Soft light blend function
fn soft_light_blend(base: f32, overlay: f32) -> f32 {
    if overlay < 0.5 {
        2.0 * base * overlay + base * base * (1.0 - 2.0 * overlay)
    } else {
        2.0 * base * (1.0 - overlay) + base.sqrt() * (2.0 * overlay - 1.0)
    }
}

/// Hard light blend function
fn hard_light_blend(base: f32, overlay: f32) -> f32 {
    if overlay < 0.5 {
        2.0 * base * overlay
    } else {
        1.0 - 2.0 * (1.0 - base) * (1.0 - overlay)
    }
}

/// Color dodge blend function
fn color_dodge_blend(base: f32, overlay: f32) -> f32 {
    if overlay >= 1.0 {
        1.0
    } else {
        (base / (1.0 - overlay)).min(1.0)
    }
}

/// Color burn blend function
fn color_burn_blend(base: f32, overlay: f32) -> f32 {
    if overlay <= 0.0 {
        0.0
    } else {
        1.0 - ((1.0 - base) / overlay).min(1.0)
    }
}

/// Convert RGB to HSL color space
fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;

    if delta == 0.0 {
        return (0.0, 0.0, l);
    }

    let s = if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let h = if max == r {
        ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if max == g {
        ((b - r) / delta + 2.0) / 6.0
    } else {
        ((r - g) / delta + 4.0) / 6.0
    };

    (h, s, l)
}

/// Convert HSL to RGB color space
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (l, l, l);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };

    let p = 2.0 * l - q;

    let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

    (r, g, b)
}

/// Helper function for HSL to RGB conversion
fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }

    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

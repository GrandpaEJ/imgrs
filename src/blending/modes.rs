/// Blend modes for compositing operations
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum BlendMode {
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
    let overlay_a = overlay.3 as f32 / 255.0;
    
    let (blended_r, blended_g, blended_b) = match blend_mode {
        BlendMode::Normal => (overlay_r, overlay_g, overlay_b),
        BlendMode::Multiply => (
            base_r * overlay_r,
            base_g * overlay_g,
            base_b * overlay_b,
        ),
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
    };
    
    // Apply opacity
    let final_r = base_r * (1.0 - opacity) + blended_r * opacity;
    let final_g = base_g * (1.0 - opacity) + blended_g * opacity;
    let final_b = base_b * (1.0 - opacity) + blended_b * opacity;
    
    // Combine alpha channels
    let final_a = base_a + overlay_a * (1.0 - base_a);
    
    (
        (final_r * 255.0) as u8,
        (final_g * 255.0) as u8,
        (final_b * 255.0) as u8,
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


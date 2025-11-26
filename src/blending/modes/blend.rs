/// Main blending function
use super::types::BlendMode;
use super::helpers::*;

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
    let (blended_r, blended_g, blended_b) = if super::normal::is_normal(blend_mode) {
        super::normal::apply_normal((base_r, base_g, base_b), (overlay_r, overlay_g, overlay_b))
    } else if super::multiply::is_multiply(blend_mode) {
        super::multiply::apply_multiply((base_r, base_g, base_b), (overlay_r, overlay_g, overlay_b))
    } else if super::screen::is_screen(blend_mode) {
        super::screen::apply_screen((base_r, base_g, base_b), (overlay_r, overlay_g, overlay_b))
    } else if super::overlay::is_overlay(blend_mode) {
        super::overlay::apply_overlay((base_r, base_g, base_b), (overlay_r, overlay_g, overlay_b))
    } else if super::soft_light::is_soft_light(blend_mode) {
        super::soft_light::apply_soft_light((base_r, base_g, base_b), (overlay_r, overlay_g, overlay_b))
    } else if super::hard_light::is_hard_light(blend_mode) {
        super::hard_light::apply_hard_light((base_r, base_g, base_b), (overlay_r, overlay_g, overlay_b))
    } else if super::darken::is_darken(blend_mode) {
        super::darken::apply_darken((base_r, base_g, base_b), (overlay_r, overlay_g, overlay_b))
    } else if super::lighten::is_lighten(blend_mode) {
        super::lighten::apply_lighten((base_r, base_g, base_b), (overlay_r, overlay_g, overlay_b))
    } else if super::difference::is_difference(blend_mode) {
        super::difference::apply_difference((base_r, base_g, base_b), (overlay_r, overlay_g, overlay_b))
    } else if super::exclusion::is_exclusion(blend_mode) {
        super::exclusion::apply_exclusion((base_r, base_g, base_b), (overlay_r, overlay_g, overlay_b))
    } else {
        // For remaining modes, use the original logic
        match blend_mode {
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
        }
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
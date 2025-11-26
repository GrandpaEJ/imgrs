/// Overlay blend mode implementation

use super::helpers::overlay_blend;
use super::types::BlendMode;

pub fn apply_overlay(base: (f32, f32, f32), overlay: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        overlay_blend(base.0, overlay.0),
        overlay_blend(base.1, overlay.1),
        overlay_blend(base.2, overlay.2),
    )
}

pub fn is_overlay(mode: BlendMode) -> bool {
    matches!(mode, BlendMode::Overlay)
}
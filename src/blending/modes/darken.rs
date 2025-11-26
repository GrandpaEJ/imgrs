/// Darken blend mode implementation

use super::types::BlendMode;

pub fn apply_darken(base: (f32, f32, f32), overlay: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        base.0.min(overlay.0),
        base.1.min(overlay.1),
        base.2.min(overlay.2),
    )
}

pub fn is_darken(mode: BlendMode) -> bool {
    matches!(mode, BlendMode::Darken)
}
/// Lighten blend mode implementation

use super::types::BlendMode;

pub fn apply_lighten(base: (f32, f32, f32), overlay: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        base.0.max(overlay.0),
        base.1.max(overlay.1),
        base.2.max(overlay.2),
    )
}

pub fn is_lighten(mode: BlendMode) -> bool {
    matches!(mode, BlendMode::Lighten)
}
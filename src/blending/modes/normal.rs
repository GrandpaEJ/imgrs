/// Normal blend mode implementation

use super::types::BlendMode;

pub fn apply_normal(_base: (f32, f32, f32), overlay: (f32, f32, f32)) -> (f32, f32, f32) {
    overlay
}

pub fn is_normal(mode: BlendMode) -> bool {
    matches!(mode, BlendMode::Normal)
}
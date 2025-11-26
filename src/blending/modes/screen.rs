/// Screen blend mode implementation

use super::types::BlendMode;

pub fn apply_screen(base: (f32, f32, f32), overlay: (f32, f32, f32)) -> (f32, f32, f32) {
    (
        1.0 - (1.0 - base.0) * (1.0 - overlay.0),
        1.0 - (1.0 - base.1) * (1.0 - overlay.1),
        1.0 - (1.0 - base.2) * (1.0 - overlay.2),
    )
}

pub fn is_screen(mode: BlendMode) -> bool {
    matches!(mode, BlendMode::Screen)
}
/// Helper functions for blend operations

/// Overlay blend function
pub fn overlay_blend(base: f32, overlay: f32) -> f32 {
    if base < 0.5 {
        2.0 * base * overlay
    } else {
        1.0 - 2.0 * (1.0 - base) * (1.0 - overlay)
    }
}

/// Soft light blend function
pub fn soft_light_blend(base: f32, overlay: f32) -> f32 {
    if overlay < 0.5 {
        2.0 * base * overlay + base * base * (1.0 - 2.0 * overlay)
    } else {
        2.0 * base * (1.0 - overlay) + base.sqrt() * (2.0 * overlay - 1.0)
    }
}

/// Hard light blend function
pub fn hard_light_blend(base: f32, overlay: f32) -> f32 {
    if overlay < 0.5 {
        2.0 * base * overlay
    } else {
        1.0 - 2.0 * (1.0 - base) * (1.0 - overlay)
    }
}

/// Color dodge blend function
pub fn color_dodge_blend(base: f32, overlay: f32) -> f32 {
    if overlay >= 1.0 {
        1.0
    } else {
        (base / (1.0 - overlay)).min(1.0)
    }
}

/// Color burn blend function
pub fn color_burn_blend(base: f32, overlay: f32) -> f32 {
    if overlay <= 0.0 {
        0.0
    } else {
        1.0 - ((1.0 - base) / overlay).min(1.0)
    }
}

/// Convert RGB to HSL color space
pub fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
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
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
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
pub fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
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
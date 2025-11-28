// Blending operations for image compositing

use crate::errors::ImgrsError;
use image::{DynamicImage, GenericImageView, Rgba};

pub type Pixel = (u8, u8, u8, u8);

pub fn blend_clear(_dest: Pixel, _source: Pixel) -> Pixel {
    (0, 0, 0, 0)
}

pub fn blend_source(_dest: Pixel, source: Pixel) -> Pixel {
    source
}

pub fn blend_over(dest: Pixel, source: Pixel) -> Pixel {
    let sa = source.3 as u32;
    let da = dest.3 as u32;
    let oa = sa + da * (255 - sa) / 255;
    if oa == 0 {
        return (0, 0, 0, 0);
    }
    let r = (source.0 as u32 * sa + dest.0 as u32 * da * (255 - sa) / 255) / oa;
    let g = (source.1 as u32 * sa + dest.1 as u32 * da * (255 - sa) / 255) / oa;
    let b = (source.2 as u32 * sa + dest.2 as u32 * da * (255 - sa) / 255) / oa;
    let a = oa;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_in(dest: Pixel, source: Pixel) -> Pixel {
    let r = source.0 as u32 * dest.3 as u32 / 255;
    let g = source.1 as u32 * dest.3 as u32 / 255;
    let b = source.2 as u32 * dest.3 as u32 / 255;
    let a = source.3 as u32 * dest.3 as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_out(dest: Pixel, source: Pixel) -> Pixel {
    let r = source.0 as u32 * (255 - dest.3) as u32 / 255;
    let g = source.1 as u32 * (255 - dest.3) as u32 / 255;
    let b = source.2 as u32 * (255 - dest.3) as u32 / 255;
    let a = source.3 as u32 * (255 - dest.3) as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_atop(dest: Pixel, source: Pixel) -> Pixel {
    let r = source.0 as u32 * dest.3 as u32 / 255 + dest.0 as u32 * (255 - source.3) as u32 / 255;
    let g = source.1 as u32 * dest.3 as u32 / 255 + dest.1 as u32 * (255 - source.3) as u32 / 255;
    let b = source.2 as u32 * dest.3 as u32 / 255 + dest.2 as u32 * (255 - source.3) as u32 / 255;
    let a = source.3 as u32 * dest.3 as u32 / 255 + dest.3 as u32 * (255 - source.3) as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_dest(dest: Pixel, _source: Pixel) -> Pixel {
    dest
}

pub fn blend_dest_over(dest: Pixel, source: Pixel) -> Pixel {
    let sa = source.3 as u32;
    let da = dest.3 as u32;
    let oa = da + sa * (255 - da) / 255;
    if oa == 0 {
        return (0, 0, 0, 0);
    }
    let r = (dest.0 as u32 * da + source.0 as u32 * sa * (255 - da) / 255) / oa;
    let g = (dest.1 as u32 * da + source.1 as u32 * sa * (255 - da) / 255) / oa;
    let b = (dest.2 as u32 * da + source.2 as u32 * sa * (255 - da) / 255) / oa;
    let a = oa;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_dest_in(dest: Pixel, source: Pixel) -> Pixel {
    let r = dest.0 as u32 * source.3 as u32 / 255;
    let g = dest.1 as u32 * source.3 as u32 / 255;
    let b = dest.2 as u32 * source.3 as u32 / 255;
    let a = dest.3 as u32 * source.3 as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_dest_out(dest: Pixel, source: Pixel) -> Pixel {
    let r = dest.0 as u32 * (255 - source.3) as u32 / 255;
    let g = dest.1 as u32 * (255 - source.3) as u32 / 255;
    let b = dest.2 as u32 * (255 - source.3) as u32 / 255;
    let a = dest.3 as u32 * (255 - source.3) as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_dest_atop(dest: Pixel, source: Pixel) -> Pixel {
    let r = dest.0 as u32 * source.3 as u32 / 255 + source.0 as u32 * (255 - dest.3) as u32 / 255;
    let g = dest.1 as u32 * source.3 as u32 / 255 + source.1 as u32 * (255 - dest.3) as u32 / 255;
    let b = dest.2 as u32 * source.3 as u32 / 255 + source.2 as u32 * (255 - dest.3) as u32 / 255;
    let a = dest.3 as u32 * source.3 as u32 / 255 + source.3 as u32 * (255 - dest.3) as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_xor(dest: Pixel, source: Pixel) -> Pixel {
    let r = source.0 as u32 * (255 - dest.3) as u32 / 255 + dest.0 as u32 * (255 - source.3) as u32 / 255;
    let g = source.1 as u32 * (255 - dest.3) as u32 / 255 + dest.1 as u32 * (255 - source.3) as u32 / 255;
    let b = source.2 as u32 * (255 - dest.3) as u32 / 255 + dest.2 as u32 * (255 - source.3) as u32 / 255;
    let a = source.3 as u32 * (255 - dest.3) as u32 / 255 + dest.3 as u32 * (255 - dest.3) as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_add(dest: Pixel, source: Pixel) -> Pixel {
    let r = std::cmp::min(255, dest.0 as u32 + source.0 as u32);
    let g = std::cmp::min(255, dest.1 as u32 + source.1 as u32);
    let b = std::cmp::min(255, dest.2 as u32 + source.2 as u32);
    let a = std::cmp::min(255, dest.3 as u32 + source.3 as u32);
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_saturate(dest: Pixel, source: Pixel) -> Pixel {
    let a = std::cmp::min(255, dest.3 as u32 + source.3 as u32);
    let r = dest.0 as u32 + source.0 as u32 * (255 - dest.3) as u32 / 255;
    let g = dest.1 as u32 + source.1 as u32 * (255 - dest.3) as u32 / 255;
    let b = dest.2 as u32 + source.2 as u32 * (255 - dest.3) as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_multiply(dest: Pixel, source: Pixel) -> Pixel {
    let r = dest.0 as u32 * source.0 as u32 / 255;
    let g = dest.1 as u32 * source.1 as u32 / 255;
    let b = dest.2 as u32 * source.2 as u32 / 255;
    let a = dest.3 as u32 * source.3 as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_screen(dest: Pixel, source: Pixel) -> Pixel {
    let r = 255 - (255 - dest.0 as u32) * (255 - source.0 as u32) / 255;
    let g = 255 - (255 - dest.1 as u32) * (255 - source.1 as u32) / 255;
    let b = 255 - (255 - dest.2 as u32) * (255 - source.2 as u32) / 255;
    let a = dest.3 as u32 * source.3 as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_overlay(dest: Pixel, source: Pixel) -> Pixel {
    let r = if dest.0 < 128 {
        2 * dest.0 as u32 * source.0 as u32 / 255
    } else {
        255 - 2 * (255 - dest.0 as u32) * (255 - source.0 as u32) / 255
    };
    let g = if dest.1 < 128 {
        2 * dest.1 as u32 * source.1 as u32 / 255
    } else {
        255 - 2 * (255 - dest.1 as u32) * (255 - source.1 as u32) / 255
    };
    let b = if dest.2 < 128 {
        2 * dest.2 as u32 * source.2 as u32 / 255
    } else {
        255 - 2 * (255 - dest.2 as u32) * (255 - source.2 as u32) / 255
    };
    let a = dest.3 as u32 * source.3 as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_darken(dest: Pixel, source: Pixel) -> Pixel {
    let r = std::cmp::min(dest.0, source.0);
    let g = std::cmp::min(dest.1, source.1);
    let b = std::cmp::min(dest.2, source.2);
    let a = dest.3 as u32 * source.3 as u32 / 255;
    (r, g, b, a as u8)
}

pub fn blend_lighten(dest: Pixel, source: Pixel) -> Pixel {
    let r = std::cmp::max(dest.0, source.0);
    let g = std::cmp::max(dest.1, source.1);
    let b = std::cmp::max(dest.2, source.2);
    let a = dest.3 as u32 * source.3 as u32 / 255;
    (r, g, b, a as u8)
}

pub fn blend_color_dodge(dest: Pixel, source: Pixel) -> Pixel {
    let r = if source.0 == 255 {
        255
    } else {
        std::cmp::min(255, dest.0 as u32 * 255 / (255 - source.0) as u32)
    };
    let g = if source.1 == 255 {
        255
    } else {
        std::cmp::min(255, dest.1 as u32 * 255 / (255 - source.1) as u32)
    };
    let b = if source.2 == 255 {
        255
    } else {
        std::cmp::min(255, dest.2 as u32 * 255 / (255 - source.2) as u32)
    };
    let a = dest.3 as u32 * source.3 as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_color_burn(dest: Pixel, source: Pixel) -> Pixel {
    let r = if source.0 == 0 {
        0
    } else {
        std::cmp::max(0, 255 - (255 - dest.0 as i32) * 255 / source.0 as i32)
    };
    let g = if source.1 == 0 {
        0
    } else {
        std::cmp::max(0, 255 - (255 - dest.1 as i32) * 255 / source.1 as i32)
    };
    let b = if source.2 == 0 {
        0
    } else {
        std::cmp::max(0, 255 - (255 - dest.2 as i32) * 255 / source.2 as i32)
    };
    let a = dest.3 as u32 * source.3 as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_hard_light(dest: Pixel, source: Pixel) -> Pixel {
    let r = if source.0 < 128 {
        2 * dest.0 as u32 * source.0 as u32 / 255
    } else {
        255 - 2 * (255 - dest.0 as u32) * (255 - source.0 as u32) / 255
    };
    let g = if source.1 < 128 {
        2 * dest.1 as u32 * source.1 as u32 / 255
    } else {
        255 - 2 * (255 - dest.1 as u32) * (255 - source.1 as u32) / 255
    };
    let b = if source.2 < 128 {
        2 * dest.2 as u32 * source.2 as u32 / 255
    } else {
        255 - 2 * (255 - dest.2 as u32) * (255 - source.2 as u32) / 255
    };
    let a = dest.3 as u32 * source.3 as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_soft_light(dest: Pixel, source: Pixel) -> Pixel {
    let r = if dest.0 < 128 {
        dest.0 as i32 - (255 - 2 * dest.0 as i32) * (255 - source.0 as i32) * dest.0 as i32 / (255 * 255)
    } else {
        dest.0 as i32 + (2 * dest.0 as i32 - 255) * (source.0 as i32 - 128) / 128
    };
    let g = if dest.1 < 128 {
        dest.1 as i32 - (255 - 2 * dest.1 as i32) * (255 - source.1 as i32) * dest.1 as i32 / (255 * 255)
    } else {
        dest.1 as i32 + (2 * dest.1 as i32 - 255) * (source.1 as i32 - 128) / 128
    };
    let b = if dest.2 < 128 {
        dest.2 as i32 - (255 - 2 * dest.2 as i32) * (255 - source.2 as i32) * dest.2 as i32 / (255 * 255)
    } else {
        dest.2 as i32 + (2 * dest.2 as i32 - 255) * (source.2 as i32 - 128) / 128
    };
    let a = dest.3 as u32 * source.3 as u32 / 255;
    (r.clamp(0, 255) as u8, g.clamp(0, 255) as u8, b.clamp(0, 255) as u8, a as u8)
}

pub fn blend_difference(dest: Pixel, source: Pixel) -> Pixel {
    let r = ((dest.0 as i16 - source.0 as i16).abs()) as u8;
    let g = ((dest.1 as i16 - source.1 as i16).abs()) as u8;
    let b = ((dest.2 as i16 - source.2 as i16).abs()) as u8;
    let a = dest.3 as u32 * source.3 as u32 / 255;
    (r, g, b, a as u8)
}

pub fn blend_exclusion(dest: Pixel, source: Pixel) -> Pixel {
    let r = dest.0 as u32 + source.0 as u32 - 2 * dest.0 as u32 * source.0 as u32 / 255;
    let g = dest.1 as u32 + source.1 as u32 - 2 * dest.1 as u32 * source.1 as u32 / 255;
    let b = dest.2 as u32 + source.2 as u32 - 2 * dest.2 as u32 * source.2 as u32 / 255;
    let a = dest.3 as u32 * source.3 as u32 / 255;
    (r as u8, g as u8, b as u8, a as u8)
}

pub fn blend_hsl_hue(dest: Pixel, source: Pixel) -> Pixel {
    blend_over(dest, source)
}

pub fn blend_hsl_saturation(dest: Pixel, source: Pixel) -> Pixel {
    blend_over(dest, source)
}

pub fn blend_hsl_color(dest: Pixel, source: Pixel) -> Pixel {
    blend_over(dest, source)
}

pub fn blend_hsl_luminosity(dest: Pixel, source: Pixel) -> Pixel {
    blend_over(dest, source)
}

/// Composite two images using a blend function
pub fn composite_images(dest: &DynamicImage, source: &DynamicImage, blend_func: fn(Pixel, Pixel) -> Pixel) -> Result<DynamicImage, ImgrsError> {
    let (dw, dh) = dest.dimensions();
    let (sw, sh) = source.dimensions();

    let mut result = dest.to_rgba8();
    let dest_rgba = dest.to_rgba8();
    let source_rgba = source.to_rgba8();

    for y in 0..dh {
        for x in 0..dw {
            let d = dest_rgba.get_pixel(x, y);
            let s = if x < sw && y < sh {
                *source_rgba.get_pixel(x, y)
            } else {
                Rgba([0, 0, 0, 0])
            };
            let blended = blend_func((d[0], d[1], d[2], d[3]), (s[0], s[1], s[2], s[3]));
            result.put_pixel(x, y, Rgba([blended.0, blended.1, blended.2, blended.3]));
        }
    }

    Ok(DynamicImage::ImageRgba8(result))
}
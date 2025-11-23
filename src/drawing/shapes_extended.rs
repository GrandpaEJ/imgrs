use crate::errors::ImgrsError;
use image::{DynamicImage, Rgb, Rgba};
use std::f32::consts::PI;

/// Draw a filled star on the image
pub fn draw_star(
    image: &DynamicImage,
    center_x: i32,
    center_y: i32,
    outer_radius: u32,
    inner_radius: u32,
    points: u32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    if points < 3 {
        return Err(ImgrsError::InvalidOperation(
            "Star must have at least 3 points".to_string(),
        ));
    }

    let mut result = image.clone();

    // Generate star vertices
    let mut vertices = Vec::new();
    let angle_step = PI / points as f32;

    for i in 0..(points * 2) {
        let angle = i as f32 * angle_step - PI / 2.0; // Start from top
        let radius = if i % 2 == 0 {
            outer_radius as f32
        } else {
            inner_radius as f32
        };

        let x = center_x + (radius * angle.cos()) as i32;
        let y = center_y + (radius * angle.sin()) as i32;
        vertices.push((x, y));
    }

    // Fill star using scanline algorithm
    result = fill_polygon(&result, &vertices, color)?;

    Ok(result)
}

/// Draw a filled triangle on the image
pub fn draw_triangle(
    image: &DynamicImage,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let vertices = vec![(x1, y1), (x2, y2), (x3, y3)];
    fill_polygon(image, &vertices, color)
}

/// Draw a filled polygon on the image
pub fn draw_polygon(
    image: &DynamicImage,
    points: Vec<(i32, i32)>,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    if points.len() < 3 {
        return Err(ImgrsError::InvalidOperation(
            "Polygon must have at least 3 points".to_string(),
        ));
    }
    fill_polygon(image, &points, color)
}

/// Draw a filled ellipse on the image
pub fn draw_ellipse(
    image: &DynamicImage,
    center_x: i32,
    center_y: i32,
    radius_x: u32,
    radius_y: u32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let mut result = image.clone();
    let rx = radius_x as f32;
    let ry = radius_y as f32;

    match &mut result {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (img_width, img_height) = rgb_img.dimensions();

            for y in (center_y - radius_y as i32).max(0)
                ..(center_y + radius_y as i32 + 1).min(img_height as i32)
            {
                for x in (center_x - radius_x as i32).max(0)
                    ..(center_x + radius_x as i32 + 1).min(img_width as i32)
                {
                    let dx = (x - center_x) as f32 / rx;
                    let dy = (y - center_y) as f32 / ry;
                    let distance = dx * dx + dy * dy;

                    if distance <= 1.0 {
                        rgb_img.put_pixel(x as u32, y as u32, Rgb([color.0, color.1, color.2]));
                    }
                }
            }
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (img_width, img_height) = rgba_img.dimensions();

            for y in (center_y - radius_y as i32).max(0)
                ..(center_y + radius_y as i32 + 1).min(img_height as i32)
            {
                for x in (center_x - radius_x as i32).max(0)
                    ..(center_x + radius_x as i32 + 1).min(img_width as i32)
                {
                    let dx = (x - center_x) as f32 / rx;
                    let dy = (y - center_y) as f32 / ry;
                    let distance = dx * dx + dy * dy;

                    if distance <= 1.0 {
                        let alpha = color.3 as f32 / 255.0;
                        let existing = rgba_img.get_pixel(x as u32, y as u32);

                        let blended_r =
                            ((1.0 - alpha) * existing[0] as f32 + alpha * color.0 as f32) as u8;
                        let blended_g =
                            ((1.0 - alpha) * existing[1] as f32 + alpha * color.1 as f32) as u8;
                        let blended_b =
                            ((1.0 - alpha) * existing[2] as f32 + alpha * color.2 as f32) as u8;
                        let blended_a = ((1.0 - alpha) * existing[3] as f32 + alpha * 255.0) as u8;

                        rgba_img.put_pixel(
                            x as u32,
                            y as u32,
                            Rgba([blended_r, blended_g, blended_b, blended_a]),
                        );
                    }
                }
            }
        }
        _ => {
            return Err(ImgrsError::InvalidOperation(
                "Unsupported image format for drawing".to_string(),
            ));
        }
    }

    Ok(result)
}

/// Draw a regular polygon (pentagon, hexagon, etc.)
pub fn draw_regular_polygon(
    image: &DynamicImage,
    center_x: i32,
    center_y: i32,
    radius: u32,
    sides: u32,
    rotation: f32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    if sides < 3 {
        return Err(ImgrsError::InvalidOperation(
            "Regular polygon must have at least 3 sides".to_string(),
        ));
    }

    let mut vertices = Vec::new();
    let angle_step = 2.0 * PI / sides as f32;
    let start_angle = rotation.to_radians() - PI / 2.0; // Start from top

    for i in 0..sides {
        let angle = start_angle + i as f32 * angle_step;
        let x = center_x + (radius as f32 * angle.cos()) as i32;
        let y = center_y + (radius as f32 * angle.sin()) as i32;
        vertices.push((x, y));
    }

    fill_polygon(image, &vertices, color)
}

/// Helper function to fill a polygon using scanline algorithm
fn fill_polygon(
    image: &DynamicImage,
    vertices: &[(i32, i32)],
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let mut result = image.clone();

    if vertices.len() < 3 {
        return Ok(result);
    }

    // Find bounding box
    let min_y = vertices.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = vertices.iter().map(|(_, y)| *y).max().unwrap();

    match &mut result {
        DynamicImage::ImageRgb8(rgb_img) => {
            let (img_width, img_height) = rgb_img.dimensions();

            for y in min_y.max(0)..=max_y.min(img_height as i32 - 1) {
                let mut intersections = Vec::new();

                // Find intersections with polygon edges
                for i in 0..vertices.len() {
                    let (x1, y1) = vertices[i];
                    let (x2, y2) = vertices[(i + 1) % vertices.len()];

                    if (y1 <= y && y < y2) || (y2 <= y && y < y1) {
                        let x = x1 + ((y - y1) as f32 * (x2 - x1) as f32 / (y2 - y1) as f32) as i32;
                        intersections.push(x);
                    }
                }

                intersections.sort_unstable();

                // Fill between pairs of intersections
                for chunk in intersections.chunks(2) {
                    if chunk.len() == 2 {
                        let x_start = chunk[0].max(0);
                        let x_end = chunk[1].min(img_width as i32 - 1);

                        for x in x_start..=x_end {
                            rgb_img.put_pixel(x as u32, y as u32, Rgb([color.0, color.1, color.2]));
                        }
                    }
                }
            }
        }
        DynamicImage::ImageRgba8(rgba_img) => {
            let (img_width, img_height) = rgba_img.dimensions();

            for y in min_y.max(0)..=max_y.min(img_height as i32 - 1) {
                let mut intersections = Vec::new();

                for i in 0..vertices.len() {
                    let (x1, y1) = vertices[i];
                    let (x2, y2) = vertices[(i + 1) % vertices.len()];

                    if (y1 <= y && y < y2) || (y2 <= y && y < y1) {
                        let x = x1 + ((y - y1) as f32 * (x2 - x1) as f32 / (y2 - y1) as f32) as i32;
                        intersections.push(x);
                    }
                }

                intersections.sort_unstable();

                for chunk in intersections.chunks(2) {
                    if chunk.len() == 2 {
                        let x_start = chunk[0].max(0);
                        let x_end = chunk[1].min(img_width as i32 - 1);

                        for x in x_start..=x_end {
                            let alpha = color.3 as f32 / 255.0;
                            let existing = rgba_img.get_pixel(x as u32, y as u32);

                            let blended_r =
                                ((1.0 - alpha) * existing[0] as f32 + alpha * color.0 as f32) as u8;
                            let blended_g =
                                ((1.0 - alpha) * existing[1] as f32 + alpha * color.1 as f32) as u8;
                            let blended_b =
                                ((1.0 - alpha) * existing[2] as f32 + alpha * color.2 as f32) as u8;
                            let blended_a =
                                ((1.0 - alpha) * existing[3] as f32 + alpha * 255.0) as u8;

                            rgba_img.put_pixel(
                                x as u32,
                                y as u32,
                                Rgba([blended_r, blended_g, blended_b, blended_a]),
                            );
                        }
                    }
                }
            }
        }
        _ => {
            return Err(ImgrsError::InvalidOperation(
                "Unsupported image format for drawing".to_string(),
            ));
        }
    }

    Ok(result)
}

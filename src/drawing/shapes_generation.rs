use crate::errors::ImgrsError;
use image::{DynamicImage, Rgb, Rgba};

/// Create a new image with a filled rectangle
pub fn create_rectangle(
    width: u32,
    height: u32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(width, height));

    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        // Fill background with white transparent
        for y in 0..height {
            for x in 0..width {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
        // Fill rectangle with color
        for y in 0..height {
            for x in 0..width {
                rgba_img.put_pixel(x, y, Rgba([color.0, color.1, color.2, color.3]));
            }
        }
    }

    Ok(img)
}

/// Create a new image with a filled circle
pub fn create_circle(diameter: u32, color: (u8, u8, u8, u8)) -> Result<DynamicImage, ImgrsError> {
    let size = diameter;
    let radius = diameter as f32 / 2.0;
    let center = radius;

    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(size, size));

    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        // Fill background with white transparent
        for y in 0..size {
            for x in 0..size {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
        // Draw circle
        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance <= radius {
                    rgba_img.put_pixel(x, y, Rgba([color.0, color.1, color.2, color.3]));
                }
            }
        }
    }

    Ok(img)
}

/// Create a new image with a filled triangle
pub fn create_triangle(
    width: u32,
    height: u32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(width, height));

    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        // Fill background with white transparent
        for y in 0..height {
            for x in 0..width {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
        // Draw triangle
        for y in 0..height {
            let start_x = (width as f32 * (height - y) as f32 / height as f32 / 2.0) as u32;
            let end_x = width - start_x;

            for x in start_x..end_x {
                rgba_img.put_pixel(x, y, Rgba([color.0, color.1, color.2, color.3]));
            }
        }
    }

    Ok(img)
}

/// Create a new image with a filled ellipse
pub fn create_ellipse(
    width: u32,
    height: u32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let rx = width as f32 / 2.0;
    let ry = height as f32 / 2.0;
    let center_x = rx;
    let center_y = ry;

    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(width, height));

    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        // Fill background with white transparent
        for y in 0..height {
            for x in 0..width {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
        // Draw ellipse
        for y in 0..height {
            for x in 0..width {
                let dx = (x as f32 - center_x) / rx;
                let dy = (y as f32 - center_y) / ry;
                let distance = dx * dx + dy * dy;

                if distance <= 1.0 {
                    rgba_img.put_pixel(x, y, Rgba([color.0, color.1, color.2, color.3]));
                }
            }
        }
    }

    Ok(img)
}

/// Create a new image with a filled star
pub fn create_star(size: u32, color: (u8, u8, u8, u8)) -> Result<DynamicImage, ImgrsError> {
    let points = 5;
    let outer_radius = size as f32 / 2.0;
    let inner_radius = outer_radius * 0.4;
    let center_x = size as f32 / 2.0;
    let center_y = size as f32 / 2.0;

    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(size, size));

    // Fill background with white transparent
    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        for y in 0..size {
            for x in 0..size {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
    }

    // Generate star vertices
    let mut vertices = Vec::new();
    let angle_step = std::f32::consts::PI / points as f32;

    for i in 0..(points * 2) {
        let angle = i as f32 * angle_step - std::f32::consts::PI / 2.0; // Start from top
        let radius = if i % 2 == 0 {
            outer_radius
        } else {
            inner_radius
        };

        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        vertices.push((x as i32, y as i32));
    }

    // Fill star using scanline algorithm
    fill_polygon_in_image(&mut img, &vertices, color)?;

    Ok(img)
}

/// Create a new image with a filled square
pub fn create_square(size: u32, color: (u8, u8, u8, u8)) -> Result<DynamicImage, ImgrsError> {
    create_rectangle(size, size, color)
}

/// Create a new image with a filled diamond
pub fn create_diamond(size: u32, color: (u8, u8, u8, u8)) -> Result<DynamicImage, ImgrsError> {
    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(size, size));

    let center = size as f32 / 2.0;

    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        // Fill background with white transparent
        for y in 0..size {
            for x in 0..size {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
        // Draw diamond
        for y in 0..size {
            let dy = (y as f32 - center).abs();
            let half_width = (size as f32 / 2.0) - dy;
            let start_x = (center - half_width) as u32;
            let end_x = (center + half_width) as u32;

            for x in start_x..end_x {
                if x < size {
                    rgba_img.put_pixel(x, y, Rgba([color.0, color.1, color.2, color.3]));
                }
            }
        }
    }

    Ok(img)
}

/// Create a new image with a filled hexagon
pub fn create_hexagon(size: u32, color: (u8, u8, u8, u8)) -> Result<DynamicImage, ImgrsError> {
    let radius = size as f32 / 2.0;
    let center_x = size as f32 / 2.0;
    let center_y = size as f32 / 2.0;

    let mut vertices = Vec::new();
    for i in 0..6 {
        let angle = (i as f32 * 60.0).to_radians() - std::f32::consts::PI / 2.0;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        vertices.push((x as i32, y as i32));
    }

    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(size, size));

    // Fill background with white transparent
    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        for y in 0..size {
            for x in 0..size {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
    }

    fill_polygon_in_image(&mut img, &vertices, color)?;

    Ok(img)
}

/// Create a new image with a filled parallelogram (skewed rectangle)
pub fn create_parallelogram(
    width: u32,
    height: u32,
    skew: f32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let shear = (height as f32 * skew).round() as u32;

    let img_width = width + shear;
    let img_height = height;

    let vertices = vec![
        (0, 0),
        (width as i32, 0),
        (width as i32 + shear as i32, height as i32),
        (shear as i32, height as i32),
    ];

    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(img_width, img_height));

    // Fill background with white transparent
    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        for y in 0..img_height {
            for x in 0..img_width {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
    }

    fill_polygon_in_image(&mut img, &vertices, color)?;

    Ok(img)
}

/// Create a new image with a filled pentagon
pub fn create_pentagon(size: u32, color: (u8, u8, u8, u8)) -> Result<DynamicImage, ImgrsError> {
    let radius = size as f32 / 2.0;
    let center_x = size as f32 / 2.0;
    let center_y = size as f32 / 2.0;

    let mut vertices = Vec::new();
    for i in 0..5 {
        let angle = (i as f32 * 72.0).to_radians() - std::f32::consts::PI / 2.0;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        vertices.push((x as i32, y as i32));
    }

    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(size, size));

    // Fill background with white transparent
    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        for y in 0..size {
            for x in 0..size {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
    }

    fill_polygon_in_image(&mut img, &vertices, color)?;

    Ok(img)
}

/// Create a new image with a filled octagon
pub fn create_octagon(size: u32, color: (u8, u8, u8, u8)) -> Result<DynamicImage, ImgrsError> {
    let radius = size as f32 / 2.0;
    let center_x = size as f32 / 2.0;
    let center_y = size as f32 / 2.0;

    let mut vertices = Vec::new();
    for i in 0..8 {
        let angle = (i as f32 * 45.0).to_radians() - std::f32::consts::PI / 2.0;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        vertices.push((x as i32, y as i32));
    }

    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(size, size));

    // Fill background with white transparent
    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        for y in 0..size {
            for x in 0..size {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
    }

    fill_polygon_in_image(&mut img, &vertices, color)?;

    Ok(img)
}

/// Create a new image with a filled heart shape
pub fn create_heart(size: u32, color: (u8, u8, u8, u8)) -> Result<DynamicImage, ImgrsError> {
    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(size, size));

    let center_x = size as f32 / 2.0;
    let center_y = size as f32 / 2.0;
    let radius = size as f32 / 4.0;

    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        // Fill background with white transparent
        for y in 0..size {
            for x in 0..size {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
        // Draw heart
        for y in 0..size {
            for x in 0..size {
                let dx = (x as f32 - center_x) / radius;
                let dy = (y as f32 - center_y) / radius;

                // Heart formula: (x^2 + y^2 - 1)^3 - x^2 * y^3 <= 0
                let heart_eq = (dx * dx + dy * dy - 1.0).powi(3) - dx * dx * dy * dy * dy;

                if heart_eq <= 0.0
                    && y as f32 >= center_y - radius
                    && y as f32 <= center_y + radius * 1.5
                {
                    rgba_img.put_pixel(x, y, Rgba([color.0, color.1, color.2, color.3]));
                }
            }
        }
    }

    Ok(img)
}

/// Create a new image with a filled arrow
pub fn create_arrow(
    width: u32,
    height: u32,
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(width, height));

    let shaft_width = width / 4;
    let shaft_start = height * 3 / 4;
    let arrow_width = width / 2;

    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        // Fill background with white transparent
        for y in 0..height {
            for x in 0..width {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
        // Draw shaft
        for y in shaft_start..height {
            for x in (width / 2 - shaft_width / 2)..(width / 2 + shaft_width / 2) {
                rgba_img.put_pixel(x, y, Rgba([color.0, color.1, color.2, color.3]));
            }
        }

        // Draw arrowhead
        for y in 0..shaft_start {
            let progress = 1.0 - (y as f32 / shaft_start as f32);
            let current_width = (arrow_width as f32 * progress) as u32;
            let start_x = width / 2 - current_width / 2;

            for x in start_x..(start_x + current_width) {
                rgba_img.put_pixel(x, y, Rgba([color.0, color.1, color.2, color.3]));
            }
        }
    }

    Ok(img)
}

/// Create a new image with a filled cross
pub fn create_cross(size: u32, color: (u8, u8, u8, u8)) -> Result<DynamicImage, ImgrsError> {
    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(size, size));

    let thickness = size / 6;
    let center = size / 2;

    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        // Fill background with white transparent
        for y in 0..size {
            for x in 0..size {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
        // Horizontal bar
        for y in (center - thickness / 2)..(center + thickness / 2) {
            for x in 0..size {
                rgba_img.put_pixel(x, y, Rgba([color.0, color.1, color.2, color.3]));
            }
        }

        // Vertical bar
        for y in 0..size {
            for x in (center - thickness / 2)..(center + thickness / 2) {
                rgba_img.put_pixel(x, y, Rgba([color.0, color.1, color.2, color.3]));
            }
        }
    }

    Ok(img)
}

/// Create a new image with a filled quadrilateral defined by 4 points
pub fn create_quadrilateral(
    p1: (i32, i32),
    p2: (i32, i32),
    p3: (i32, i32),
    p4: (i32, i32),
    color: (u8, u8, u8, u8),
) -> Result<DynamicImage, ImgrsError> {
    let points = [p1, p2, p3, p4];

    // Find bounding box
    let min_x = points.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = points.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = points.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = points.iter().map(|(_, y)| *y).max().unwrap();

    let width = (max_x - min_x + 1) as u32;
    let height = (max_y - min_y + 1) as u32;

    // Adjust vertices to be relative to the image
    let vertices = vec![
        (((p1.0 - min_x)), ((p1.1 - min_y))),
        (((p2.0 - min_x)), ((p2.1 - min_y))),
        (((p3.0 - min_x)), ((p3.1 - min_y))),
        (((p4.0 - min_x)), ((p4.1 - min_y))),
    ];

    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(width, height));

    // Fill background with white transparent
    if let DynamicImage::ImageRgba8(rgba_img) = &mut img {
        for y in 0..height {
            for x in 0..width {
                rgba_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
    }

    fill_polygon_in_image(&mut img, &vertices, color)?;

    Ok(img)
}

/// Helper function to fill a polygon in an image using scanline algorithm
fn fill_polygon_in_image(
    img: &mut DynamicImage,
    vertices: &[(i32, i32)],
    color: (u8, u8, u8, u8),
) -> Result<(), ImgrsError> {
    if vertices.len() < 3 {
        return Ok(());
    }

    // Find bounding box
    let min_y = vertices.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = vertices.iter().map(|(_, y)| *y).max().unwrap();

    match img {
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
                            rgba_img.put_pixel(
                                x as u32,
                                y as u32,
                                Rgba([color.0, color.1, color.2, color.3]),
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

    Ok(())
}

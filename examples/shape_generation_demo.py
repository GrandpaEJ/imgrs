#!/usr/bin/env python3
"""
Shape Generation Demo - Create various shapes without needing existing images
"""

import os

import imgrs

# Create output directory
output_dir = "examples/output/shapes"
os.makedirs(output_dir, exist_ok=True)

print("🎨 Shape Generation Demo")
print(f"Saving to: {output_dir}/")

# Create examples of all available shapes
shapes = [
    # Basic shapes
    ("circle", imgrs.Image.circle(100, (255, 0, 0, 255))),
    ("rectangle", imgrs.Image.rectangle(150, 80, (0, 255, 0, 255))),
    ("triangle", imgrs.Image.triangle(120, 100, (0, 0, 255, 255))),
    ("ellipse", imgrs.Image.ellipse(150, 80, (255, 165, 0, 255))),
    ("square", imgrs.Image.square(100, (255, 255, 0, 255))),
    ("diamond", imgrs.Image.diamond(100, (255, 0, 255, 255))),
    # Polygon shapes
    ("star", imgrs.Image.star(120, (255, 215, 0, 255))),
    ("hexagon", imgrs.Image.hexagon(100, (0, 255, 255, 255))),
    ("pentagon", imgrs.Image.pentagon(100, (255, 105, 180, 255))),
    ("octagon", imgrs.Image.octagon(100, (128, 128, 128, 255))),
    # Special shapes
    ("parallelogram", imgrs.Image.parallelogram(150, 80, 0.3, (255, 105, 255, 100))),
    ("heart", imgrs.Image.heart(120, (255, 192, 203, 255))),
    ("arrow", imgrs.Image.arrow(120, 80, (255, 69, 0, 255))),
    ("cross", imgrs.Image.cross(100, (0, 128, 0, 255))),
    # Custom quadrilateral
    (
        "quadrilateral",
        imgrs.Image.quadrilateral(
            (0, 0), (100, 0), (120, 50), (20, 50), (128, 128, 128, 255)
        ),
    ),
]

# Save each shape
for name, img in shapes:
    output_path = os.path.join(output_dir, f"{name}.png")
    img.save(output_path)
    print(f"✅ Saved {name}.png")

print("🎉 Demo complete!")

"""
Imgrs v0.2.2 Feature Demo
=========================

Showcases new features:
1. Arbitrary angle rotation
2. New shape drawing (star, triangle, polygon, ellipse, regular polygons)
"""

import os
import random

import imgrs

# Create output directory
output_dir = "output/v0_2_2_demo"
os.makedirs(output_dir, exist_ok=True)

print("🎨 Imgrs v0.2.2 Feature Demo")
print("=" * 60)

# ============================================================================
# 1. ARBITRARY ANGLE ROTATION
# ============================================================================
print("\n1️⃣  Testing Arbitrary Angle Rotation...")

# Create a simple image to rotate
img = imgrs.Image.new("RGB", (400, 400), (255, 255, 255))
img = img.draw_rectangle(150, 150, 100, 100, (255, 0, 0, 255))
# Note: Text functionality removed due to Cairo dependency
# Previously: img = img.add_text("IMGRS", (160, 180), size=30, color=(255, 255, 255, 255))
img.save(f"{output_dir}/original.png")
print("   ✅ Created original image")

# Note: Arbitrary angle rotation not implemented yet
print("   Note: Arbitrary angle rotation not implemented yet")
print("   Skipping arbitrary angle tests...")
# angles = [15, 30, 45, 60, 90, 120, 180, 270]
# for angle in angles:
#     rotated = img.rotate(float(angle))
#     rotated.save(f"{output_dir}/rotated_{angle}deg.png")
#     print(f"   ✅ Rotated {angle}° - saved")

# ============================================================================
# 2. STAR SHAPES
# ============================================================================
print("\n2️⃣  Testing Star Shapes...")

img = imgrs.Image.new("RGB", (500, 500), (30, 30, 40))

# Different star configurations
stars = [
    # (x, y, outer_r, inner_r, points, color, name)
    (100, 100, 80, 40, 5, (255, 215, 0, 255), "5-point gold"),
    (300, 100, 70, 30, 6, (255, 100, 100, 255), "6-point red"),
    (100, 300, 75, 35, 7, (100, 255, 100, 255), "7-point green"),
    (300, 300, 65, 25, 8, (100, 150, 255, 255), "8-point blue"),
]

for x, y, outer, inner, points, color, name in stars:
    img = img.draw_star(x, y, outer, inner, points, color)

img.save(f"{output_dir}/stars.png")
print("   ✅ Created star shapes")

# ============================================================================
# 3. TRIANGLES
# ============================================================================
print("\n3️⃣  Testing Triangles...")

img = imgrs.Image.new("RGB", (500, 400), (245, 245, 250))

# Different triangle types
img = img.draw_triangle(100, 50, 200, 200, 50, 150, (255, 0, 0, 200))  # Red
img = img.draw_triangle(250, 100, 400, 100, 325, 250, (0, 255, 0, 200))  # Green
img = img.draw_triangle(150, 250, 300, 250, 225, 350, (0, 0, 255, 200))  # Blue

img.save(f"{output_dir}/triangles.png")
print("   ✅ Created triangles")

# ============================================================================
# 4. CUSTOM POLYGONS
# ============================================================================
print("\n4️⃣  Testing Custom Polygons...")

img = imgrs.Image.new("RGB", (500, 400), (250, 250, 255))

# Irregular polygon (arrow shape)
arrow_points = [
    (250, 50),  # Top
    (350, 150),  # Right mid
    (300, 150),  # Inner right
    (300, 300),  # Bottom right
    (200, 300),  # Bottom left
    (200, 150),  # Inner left
    (150, 150),  # Left mid
]
img = img.draw_polygon(arrow_points, (255, 100, 50, 255))

img.save(f"{output_dir}/polygon.png")
print("   ✅ Created custom polygon")

# ============================================================================
# 5. ELLIPSES
# ============================================================================
print("\n5️⃣  Testing Ellipses...")

img = imgrs.Image.new("RGB", (500, 400), (255, 255, 255))

# Various ellipses
ellipses = [
    (150, 100, 120, 60, (255, 0, 0, 200)),  # Horizontal red
    (350, 100, 60, 90, (0, 255, 0, 200)),  # Vertical green
    (150, 280, 100, 80, (0, 0, 255, 200)),  # Blue
    (350, 280, 80, 80, (255, 200, 0, 200)),  # Circle (equal radii)
]

for x, y, rx, ry, color in ellipses:
    img = img.draw_ellipse(x, y, rx, ry, color)

img.save(f"{output_dir}/ellipses.png")
print("   ✅ Created ellipses")

# ============================================================================
# 6. REGULAR POLYGONS
# ============================================================================
print("\n6️⃣  Testing Regular Polygons...")

img = imgrs.Image.new("RGB", (600, 400), (240, 240, 245))

# Different regular polygons
polygons = [
    # (x, y, radius, sides, rotation, color, name)
    (100, 100, 80, 3, 0.0, (255, 100, 100, 255), "Triangle"),
    (250, 100, 80, 4, 45.0, (100, 255, 100, 255), "Square (rotated)"),
    (400, 100, 80, 5, 0.0, (100, 100, 255, 255), "Pentagon"),
    (500, 100, 80, 6, 0.0, (255, 255, 100, 255), "Hexagon"),
    (100, 280, 80, 7, 0.0, (255, 150, 200, 255), "Heptagon"),
    (250, 280, 80, 8, 0.0, (150, 255, 200, 255), "Octagon"),
    (400, 280, 80, 10, 0.0, (200, 150, 255, 255), "Decagon"),
    (500, 280, 80, 12, 0.0, (255, 200, 150, 255), "Dodecagon"),
]

for x, y, r, sides, rot, color, name in polygons:
    img = img.draw_regular_polygon(x, y, r, sides, color, rotation=rot)

img.save(f"{output_dir}/regular_polygons.png")
print("   ✅ Created regular polygons")

# ============================================================================
# 7. COMBINED DEMO - CREATE COOL DESIGN
# ============================================================================
print("\n7️⃣  Creating Combined Design...")

img = imgrs.Image.new("RGB", (800, 600), (20, 20, 30))

# Background stars
random.seed(42)
for _ in range(20):
    x = random.randint(50, 750)
    y = random.randint(50, 550)
    size = random.randint(10, 30)
    img = img.draw_star(x, y, size, size // 2, 5, (255, 255, 255, 100))

# Main design - concentric regular polygons
center_x, center_y = 400, 300
colors = [
    (255, 50, 50, 255),
    (255, 150, 50, 255),
    (255, 255, 50, 255),
    (50, 255, 50, 255),
    (50, 150, 255, 255),
    (150, 50, 255, 255),
]

for i, (sides, color) in enumerate(zip(range(3, 9), colors)):
    radius = 200 - i * 25
    rotation = i * 15.0
    img = img.draw_regular_polygon(
        center_x, center_y, radius, sides, color, rotation=rotation
    )

# Note: Title with rotation removed due to text functionality removal
# Previously: title_img = title_img.add_text("v0.2.2", (10, 10), size=50, color=(255, 255, 255, 255))
# title_img = title_img.rotate(15.0)
# Note: paste functionality would be used here in production

# Save final design
img.save(f"{output_dir}/combined_design.png")
print("   ✅ Created combined design")

# ============================================================================
# 8. ROTATION SHOWCASE
# ============================================================================
print("\n8️⃣  Creating Rotation Showcase...")

# Create a colorful gradient image
img = imgrs.Image.new("RGB", (300, 300), (255, 255, 255))

# Draw some shapes
img = img.draw_star(150, 100, 60, 30, 5, (255, 215, 0, 255))
img = img.draw_rectangle(100, 150, 100, 50, (255, 0, 0, 255))
img = img.draw_circle(150, 250, 40, (0, 0, 255, 255))
# Note: Text functionality removed due to Cairo dependency
# Previously: img = img.add_text("ROTATE", (90, 165), size=25, color=(255, 255, 255, 255))

# Note: Rotation sequence with arbitrary angles not implemented
print("   Note: Rotation sequence with arbitrary angles not implemented")
print("   Skipping rotation sequence...")
# angles = [0, 30, 60, 90, 120, 150, 180, 210, 240, 270, 300, 330]
# for i, angle in enumerate(angles):
#     rotated = img.rotate(float(angle))
#     rotated.save(f"{output_dir}/rotation_seq_{i:02d}_{angle}deg.png")

print("   ✅ Rotation showcase completed (shapes only)")

# ============================================================================
print("\n" + "=" * 60)
print("✅ v0.2.2 Demo Complete!")
print("=" * 60)
print(f"\n📁 Output saved to: {output_dir}/")
print("\n🎨 New Features Demonstrated:")
print("   1. Star shapes (customizable points)")
print("   2. Triangle drawing")
print("   3. Custom polygon drawing")
print("   4. Ellipse drawing")
print("   5. Regular polygons (pentagon, hexagon, etc.)")
print("   Note: Arbitrary angle rotation not implemented yet")
print("\n🚀 Shape drawing features working perfectly!")

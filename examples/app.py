"""
Create a cool cover photo for imgrs
Professional design showcasing the library
"""

import imgrs
import os

os.makedirs("examples/output/cover", exist_ok=True)

print("="*70)
print("🎨 Creating IMGRS Cover Photo...")
print("="*70)
print()

# Create a large canvas with dark gradient background
print("Creating canvas...")
canvas = imgrs.Image.new("RGBA", (1920, 1080), (15, 20, 35, 255))

# Add main title with epic styling - use textbox to center it
print("Adding main title...")
# Measure text to center it properly
title_text = "IMGRS"
title_width, title_height = imgrs.Image.get_text_size(title_text, size=220)
title_x = (1920 - title_width) // 2  # Center horizontally
title_y = 300

canvas = canvas.add_text_styled(
    title_text,
    (title_x, title_y),
    size=220,
    color=(255, 215, 0, 255),           # Gold
    outline=(255, 140, 0, 255, 12.0),   # Orange outline
    shadow=(12, 12, 0, 0, 0, 240)       # Strong shadow
)

# Add subtitle - use textbox for precise centering
print("Adding subtitle...")
subtitle = "Blazingly Fast Image Processing for Python"
sub_width, sub_height = imgrs.Image.get_text_size(subtitle, size=56)
sub_x = (1920 - sub_width) // 2
canvas = canvas.add_text(
    subtitle,
    (sub_x, 540),
    size=56,
    color=(150, 200, 255, 255)
)

# Add powered by line
powered_text = "Powered by Rust"
pow_width, pow_height = imgrs.Image.get_text_size(powered_text, size=42)
pow_x = (1920 - pow_width) // 2
canvas = canvas.add_text(
    powered_text,
    (pow_x, 620),
    size=42,
    color=(255, 150, 100, 255)
)

# Feature highlights
print("Adding features...")
features_y = 750

features = [
    ("65+ FILTERS", (400, features_y), (100, 255, 200, 255)),
    ("RICH TEXT", (800, features_y), (255, 200, 100, 255)),
    ("TEXTBOX", (1200, features_y), (200, 150, 255, 255)),
    ("EXIF DATA", (1550, features_y), (255, 100, 150, 255)),
]

for text, (x, y), color in features:
    # Use textbox to center each feature text
    feat_width, feat_height = imgrs.Image.get_text_size(text, size=38)
    feat_x = x - (feat_width // 2)
    
    canvas = canvas.add_text_styled(
        text,
        (feat_x, y),
        size=38,
        color=color,
        outline=(0, 0, 0, 255, 2.0)
    )

# Add bottom badges
print("Adding info badges...")
badges_y = 900

badges = [
    ("TTF/OTF", (300, badges_y), (200, 200, 255, 255)),
    ("RGBA Colors", (600, badges_y), (255, 200, 200, 255)),
    ("Text Alignment", (950, badges_y), (200, 255, 200, 255)),
    ("Shadows & Outlines", (1400, badges_y), (255, 255, 150, 255)),
]

for text, (x, y), color in badges:
    canvas = canvas.add_text(
        text,
        (x, y),
        size=28,
        color=color
    )

# Add footer - use textbox for centering
print("Adding footer...")
footer1 = "Python + Rust = High Performance"
f1_width, f1_height = imgrs.Image.get_text_size(footer1, size=32)
f1_x = (1920 - f1_width) // 2

canvas = canvas.add_text(
    footer1,
    (f1_x, 1000),
    size=32,
    color=(180, 180, 200, 255)
)

footer2 = "github.com/grandpaej/imgrs"
f2_width, f2_height = imgrs.Image.get_text_size(footer2, size=24)
f2_x = (1920 - f2_width) // 2

canvas = canvas.add_text(
    footer2,
    (f2_x, 1045),
    size=24,
    color=(120, 140, 180, 255)
)

# Save versions
print()
print("Saving cover photos...")
canvas.save("examples/output/cover/imgrs_cover.png")
print("✅ Full size: imgrs_cover.png (1920x1080)")

# Create thumbnail
thumb = canvas.resize((960, 540))
thumb.save("examples/output/cover/imgrs_cover_small.png")
print("✅ Thumbnail: imgrs_cover_small.png (960x540)")

# Create banner version
banner = canvas.crop((0, 200, 1920, 700))
banner.save("examples/output/cover/imgrs_banner.png")
print("✅ Banner: imgrs_banner.png (1920x500)")

print()
print("="*70)
print("🎉 COVER PHOTOS CREATED!")
print("="*70)
print()
print("Features showcased:")
print("  ✅ Rich text with gold styling and shadows")
print("  ✅ Text outlines (12px orange)")
print("  ✅ Text alignment and centering")
print("  ✅ Multiple text sizes and colors")
print("  ✅ Professional layout design")
print("  ✅ Clean, modern aesthetic")
print()
print("Files:")
print("  📸 imgrs_cover.png (Full HD - 1920x1080)")
print("  📸 imgrs_cover_small.png (HD - 960x540)")
print("  📸 imgrs_banner.png (Wide - 1920x500)")
print()
print("📁 Output: examples/output/cover/")
print("="*70)

"""
Demo of textbox features - measuring and positioning text
"""

import imgrs
import os

os.makedirs("examples/output/textbox_demo", exist_ok=True)

print("="*70)
print("📏 TEXTBOX MEASUREMENT DEMO")
print("="*70)
print()

# =============================================================================
# 1. BASIC TEXT SIZE MEASUREMENT
# =============================================================================
print("1. Basic Text Size")
text = "Hello World"
width, height = imgrs.Image.get_text_size(text, size=48)
print(f"   Text: '{text}'")
print(f"   Size: 48px")
print(f"   Dimensions: {width}x{height} pixels")
print(f"   ✅ get_text_size() working")
print()

# =============================================================================
# 2. MULTILINE TEXT SIZE
# =============================================================================
print("2. Multiline Text Size")
multiline = "Line 1\nLine 2\nLine 3"
width, height, lines = imgrs.Image.get_multiline_text_size(multiline, size=32)
print(f"   Text: 3 lines")
print(f"   Dimensions: {width}x{height} pixels")
print(f"   Line count: {lines}")
print(f"   ✅ get_multiline_text_size() working")
print()

# =============================================================================
# 3. FULL TEXT BOX INFORMATION
# =============================================================================
print("3. Full TextBox Information")
box = imgrs.Image.get_text_box("Sample", 100, 50, size=64)
print(f"   Position: ({box['x']}, {box['y']})")
print(f"   Dimensions: {box['width']}x{box['height']}")
print(f"   Baseline Y: {box['baseline_y']}")
print(f"   Bottom Y: {box['bottom_y']}")
print(f"   Right X: {box['right_x']}")
print(f"   Ascent: {box['ascent']}")
print(f"   Descent: {box['descent']}")
print(f"   ✅ get_text_box() working")
print()

# =============================================================================
# 4. DYNAMIC TEXT POSITIONING
# =============================================================================
print("4. Dynamic Text Positioning")
img = imgrs.Image.new("RGBA", (800, 600), (255, 255, 255, 255))

# Measure text first
text1 = "Title"
w1, h1 = imgrs.Image.get_text_size(text1, size=64)

# Center it horizontally
x1 = (img.width - w1) // 2
y1 = 50
img = img.add_text(text1, (x1, y1), size=64, color=(0, 0, 128, 255))

# Add subtitle below, also centered
text2 = "Subtitle"
w2, h2 = imgrs.Image.get_text_size(text2, size=32)
x2 = (img.width - w2) // 2
y2 = y1 + h1 + 20
img = img.add_text(text2, (x2, y2), size=32, color=(128, 128, 128, 255))

img.save("examples/output/textbox_demo/01_dynamic_positioning.png")
print(f"   ✅ Dynamic centering based on measured sizes")
print()

# =============================================================================
# 5. TEXT ALIGNMENT WITH BOXES
# =============================================================================
print("5. Text Boxes for Layout")
img2 = imgrs.Image.new("RGBA", (800, 400), (240, 240, 240, 255))

texts = [
    ("Top Left", 50, 50),
    ("Top Right", 750, 50),
    ("Bottom Left", 50, 350),
    ("Bottom Right", 750, 350),
]

for text, x, y in texts:
    # Get text box
    box = imgrs.Image.get_text_box(text, x, y, size=32)
    
    # Adjust position based on alignment
    if "Right" in text:
        x = x - box['width']
    if "Bottom" in text:
        y = y - box['height']
    
    img2 = img2.add_text(text, (x, y), size=32, color=(0, 0, 0, 255))

img2.save("examples/output/textbox_demo/02_corner_alignment.png")
print(f"   ✅ Corner-aligned text using measured boxes")
print()

# =============================================================================
# 6. TEXT FITTING IN BOXES
# =============================================================================
print("6. Text Fitting in Defined Area")
img3 = imgrs.Image.new("RGBA", (800, 400), (255, 255, 255, 255))

# Define a box
box_x, box_y = 100, 100
box_width, box_height = 600, 200

# Find the right size for text to fit
target_text = "FIT THIS TEXT"
for test_size in range(100, 10, -5):
    w, h = imgrs.Image.get_text_size(target_text, size=test_size)
    if w <= box_width and h <= box_height:
        # Found a size that fits!
        # Center it in the box
        text_x = box_x + (box_width - w) // 2
        text_y = box_y + (box_height - h) // 2
        img3 = img3.add_text(target_text, (text_x, text_y), size=test_size, color=(255, 0, 0, 255))
        print(f"   Optimal size: {test_size}px")
        print(f"   Text size: {w}x{h} in {box_width}x{box_height} box")
        break

img3.save("examples/output/textbox_demo/03_auto_fit.png")
print(f"   ✅ Auto-sized text to fit in box")
print()

# =============================================================================
# 7. BASELINE ALIGNMENT
# =============================================================================
print("7. Baseline Alignment")
img4 = imgrs.Image.new("RGBA", (800, 300), (255, 255, 255, 255))

# Different sized texts on same baseline
baseline_y = 150
texts_sizes = [("Big", 72), ("Medium", 48), ("Small", 24)]

x = 50
for text, size in texts_sizes:
    box = imgrs.Image.get_text_box(text, x, baseline_y, size=size)
    # Adjust y to align baselines
    y = baseline_y - box['ascent']
    img4 = img4.add_text(text, (x, y), size=size, color=(0, 0, 0, 255))
    x += box['width'] + 30

img4.save("examples/output/textbox_demo/04_baseline_align.png")
print(f"   ✅ Multiple sizes aligned on same baseline")
print()

# =============================================================================
# 8. MULTI-LINE TEXT LAYOUT
# =============================================================================
print("8. Multi-line Text Layout")
img5 = imgrs.Image.new("RGBA", (600, 500), (255, 255, 255, 255))

# Complex multi-line text
poem = """Roses are red,
Violets are blue,
imgrs is fast,
And beautiful too!"""

w, h, line_count = imgrs.Image.get_multiline_text_size(poem, size=32, line_spacing=1.5)

# Center the whole block
x = (img5.width - w) // 2
y = (img5.height - h) // 2

img5 = img5.add_text_multiline(poem, (x, y), size=32, color=(0, 0, 128, 255), line_spacing=1.5, align='center')

img5.save("examples/output/textbox_demo/05_multiline_centered.png")
print(f"   ✅ Centered multi-line text block ({line_count} lines)")
print()

print("="*70)
print("✨ TEXTBOX DEMO COMPLETE!")
print("="*70)
print()
print("Features Demonstrated:")
print("  ✅ get_text_size() - Basic text measurement")
print("  ✅ get_multiline_text_size() - Multi-line measurement")
print("  ✅ get_text_box() - Complete bounding box info")
print("  ✅ Dynamic positioning based on measurements")
print("  ✅ Corner alignment using boxes")
print("  ✅ Auto-sizing text to fit areas")
print("  ✅ Baseline alignment")
print("  ✅ Multi-line centered layouts")
print()
print(f"📁 Output: examples/output/textbox_demo/ (5 files)")
print("="*70)


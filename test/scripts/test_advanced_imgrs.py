"""
Advanced imgrs features (beyond Pillow)
Shows what's extra in imgrs
"""

from imgrs import Image

print("Advanced imgrs Features (Beyond Pillow)")
print("=" * 60)

img = Image.open("examples/img/gradient.png")

# Features NOT in standard Pillow
print("\n### Features NOT in Pillow ###")

# 1. Auto-enhancement
print("\n1. Auto Enhancement")
enhanced = img.auto_enhance()
enhanced.save("test/advanced_01_auto_enhance.png")
print("   ✅ auto_enhance() - Not in Pillow")

# 2. Motion blur
print("\n2. Motion Blur")
motion = img.motion_blur(20, 45)
motion.save("test/advanced_02_motion_blur.png")
print("   ✅ motion_blur() - Not in Pillow")

# 3. Rich text with styling
print("\n3. Rich Text Rendering")
canvas = Image.new("RGBA", (600, 300), (30, 30, 50, 255))
canvas = canvas.add_text_styled(
    "IMGRS TEXT",
    (300, 100),
    size=72,
    color=(255, 215, 0, 255),
    outline=(255, 140, 0, 255, 4.0),
    shadow=(5, 5, 0, 0, 0, 200)
)
canvas.save("test/advanced_03_text_styled.png")
print("   ✅ add_text_styled() - Not in Pillow")

# 4. Text measurement
print("\n4. Text Measurement (Textbox)")
text = "Measure Me"
width, height = Image.get_text_size(text, size=48)
box = Image.get_text_box(text, 100, 50, size=48)
print(f"   Text size: {width}x{height}")
print(f"   Baseline: {box['baseline_y']}")
print("   ✅ get_text_size() - Not in Pillow")
print("   ✅ get_text_box() - Not in Pillow")

# 5. Auto white balance
print("\n5. Auto White Balance")
balanced = img.auto_white_balance()
balanced.save("test/advanced_04_white_balance.png")
print("   ✅ auto_white_balance() - Not in Pillow")

# 6. Edge effects
print("\n6. Advanced Edge Detection")
canny = img.canny_edge_detect(50, 150)
canny.save("test/advanced_05_canny.png")
print("   ✅ canny_edge_detect() - Not in Pillow")

# 7. Artistic effects
print("\n7. Artistic Effects")
watercolor = img.watercolor(5)
watercolor.save("test/advanced_06_watercolor.png")
print("   ✅ watercolor() - Not in Pillow")

# 8. Drop shadow
print("\n8. Drop Shadow")
rgba_img = img.convert("RGBA")
shadowed = rgba_img.drop_shadow(5, 5, 10.0, (0, 0, 0, 128))
shadowed.save("test/advanced_07_shadow.png")
print("   ✅ drop_shadow() - Not in Pillow")

# 9. Histogram equalization
print("\n9. Histogram Equalization")
equalized = img.histogram_equalization()
equalized.save("test/advanced_08_hist_eq.png")
print("   ✅ histogram_equalization() - Not in Pillow")

# 10. Color splash
print("\n10. Color Splash")
splash = img.color_splash(0.0, 60.0)
splash.save("test/advanced_09_color_splash.png")
print("   ✅ color_splash() - Not in Pillow")

print("\n" + "=" * 60)
print("✅ All advanced imgrs features working!")
print("=" * 60)
print("\nWhat imgrs adds over Pillow:")
print("  • 65+ advanced filters")
print("  • Rich text rendering with TTF/OTF")
print("  • Text measurement (textbox)")
print("  • Auto-enhancement features")
print("  • Advanced artistic effects")
print("  • Drop shadows and glow")
print("  • EXIF metadata reading")
print("  • Much faster performance (Rust)")
print("=" * 60)


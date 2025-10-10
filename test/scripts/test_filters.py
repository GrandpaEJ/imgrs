"""
Common Pillow filter operations - enhanced with imgrs!
"""

from imgrs import Image

print("Testing Filters (Pillow + imgrs enhancements)")
print("=" * 60)

img = Image.open("examples/img/gradient.png")

# Pillow-compatible filters
print("\n### Pillow-Compatible Filters ###")

print("\n1. Blur")
blurred = img.blur(5.0)
blurred.save("test/filters_01_blur.png")
print("   ✅ Blur filter applied")

print("\n2. Sharpen")
sharpened = img.sharpen(1.5)
sharpened.save("test/filters_02_sharpen.png")
print("   ✅ Sharpen filter applied")

print("\n3. Edge Detection")
edges = img.edge_detect()
edges.save("test/filters_03_edges.png")
print("   ✅ Edge detection applied")

print("\n4. Emboss")
embossed = img.emboss()
embossed.save("test/filters_04_emboss.png")
print("   ✅ Emboss filter applied")

print("\n5. Brightness")
brighter = img.brightness(50)
brighter.save("test/filters_05_brightness.png")
print("   ✅ Brightness adjusted")

print("\n6. Contrast")
contrasted = img.contrast(1.5)
contrasted.save("test/filters_06_contrast.png")
print("   ✅ Contrast adjusted")

# imgrs enhanced filters
print("\n### imgrs Enhanced Filters ###")

print("\n7. Gaussian Blur")
gaussian = img.blur(10.0)  # More blur
gaussian.save("test/filters_07_gaussian.png")
print("   ✅ Gaussian blur applied")

print("\n8. Motion Blur")
motion = img.motion_blur(20, 45)
motion.save("test/filters_08_motion.png")
print("   ✅ Motion blur applied")

print("\n9. Sepia")
sepia = img.sepia()
sepia.save("test/filters_09_sepia.png")
print("   ✅ Sepia tone applied")

print("\n10. Auto Enhance")
enhanced = img.auto_enhance()
enhanced.save("test/filters_10_auto_enhance.png")
print("   ✅ Auto enhancement applied")

# Filter chaining
print("\n### Filter Chaining ###")
print("\n11. Multiple Filters")
result = img.blur(2.0).sharpen(1.2).brightness(20).contrast(1.1)
result.save("test/filters_11_chained.png")
print("   ✅ Multiple filters chained")

print("\n" + "=" * 60)
print("✅ All filter operations working!")
print("=" * 60)


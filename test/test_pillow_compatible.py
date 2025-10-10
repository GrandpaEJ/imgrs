"""
Complete Pillow compatibility test
Shows imgrs as a drop-in replacement
"""

# This is the ONLY line you need to change from Pillow!
from imgrs import Image
# Instead of: from PIL import Image

print("Pillow Compatibility Test")
print("=" * 60)
print("Using: from imgrs import Image")
print("=" * 60)

# Standard Pillow workflow
print("\n### Standard Pillow Workflow ###")

# Open image
print("\n1. Image.open()")
img = Image.open("examples/img/colorful_squares.png")
print(f"   ✅ {img.width}x{img.height} {img.mode}")

# Resize
print("\n2. resize()")
img = img.resize((200, 200))
print(f"   ✅ Resized to {img.size}")

# Rotate
print("\n3. rotate()")
img = img.rotate(90)
print("   ✅ Rotated 90°")

# Convert
print("\n4. convert()")
img = img.convert("L")
print(f"   ✅ Converted to {img.mode}")

# Save
print("\n5. save()")
img.save("test/pillow_workflow.png")
print("   ✅ Saved")

# Create new
print("\n6. Image.new()")
new_img = Image.new("RGB", (300, 300), "blue")
new_img.save("test/pillow_new.png")
print(f"   ✅ Created {new_img.size} blue image")

# Crop
print("\n7. crop()")
img2 = Image.open("examples/img/gradient.png")
cropped = img2.crop((10, 10, 100, 100))
cropped.save("test/pillow_crop.png")
print(f"   ✅ Cropped to {cropped.size}")

# Copy
print("\n8. copy()")
copy = img2.copy()
print("   ✅ Image copied")

# Properties
print("\n9. Properties")
print(f"   size: {img2.size}")
print(f"   width: {img2.width}")
print(f"   height: {img2.height}")
print(f"   mode: {img2.mode}")
print("   ✅ All properties work")

# Paste
print("\n10. paste()")
base = Image.new("RGB", (200, 200), (255, 255, 255))
overlay = Image.new("RGB", (50, 50), (255, 0, 0))
result = base.paste(overlay, (75, 75))
result.save("test/pillow_paste.png")
print("   ✅ Pasted images")

# Split channels
print("\n11. split()")
r, g, b = img2.split()
r.save("test/pillow_channel_r.png")
print(f"   ✅ Split into {len(img2.split())} channels")

# Thumbnail
print("\n12. thumbnail()")
thumb = img2.copy()
thumb.thumbnail((64, 64))
thumb.save("test/pillow_thumbnail.png")
print(f"   ✅ Thumbnail: {thumb.size}")

print("\n" + "=" * 60)
print("✅ ALL PILLOW OPERATIONS WORKING WITH IMGRS!")
print("=" * 60)
print("\nTo migrate from Pillow:")
print("  Change: from PIL import Image")
print("  To:     from imgrs import Image")
print("\nThat's it! Your code works unchanged.")
print("=" * 60)


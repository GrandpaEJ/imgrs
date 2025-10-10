"""
Common Pillow basic operations - works with imgrs!
"""

# Traditional Pillow import (but using imgrs)
from imgrs import Image

print("Testing Basic Operations (Pillow-compatible)")
print("=" * 60)

# 1. Open and save
print("\n1. Open and Save")
img = Image.open("examples/img/gradient.png")
img.save("test/output_01_open_save.png")
print(f"   ✅ Opened and saved: {img.width}x{img.height}")

# 2. Resize
print("\n2. Resize")
resized = img.resize((200, 150))
resized.save("test/output_02_resize.png")
print(f"   ✅ Resized to: {resized.width}x{resized.height}")

# 3. Crop (left, top, right, bottom)
print("\n3. Crop")
cropped = img.crop((50, 50, 200, 150))
cropped.save("test/output_03_crop.png")
print(f"   ✅ Cropped to: {cropped.width}x{cropped.height}")

# 4. Rotate
print("\n4. Rotate")
rotated = img.rotate(90)
rotated.save("test/output_04_rotate.png")
print(f"   ✅ Rotated 90°")

# 5. Thumbnail
print("\n5. Thumbnail")
thumb = img.copy()
thumb.thumbnail((100, 100))
thumb.save("test/output_05_thumbnail.png")
print(f"   ✅ Thumbnail: {thumb.width}x{thumb.height}")

# 6. Convert modes
print("\n6. Convert")
gray = img.convert("L")
gray.save("test/output_06_grayscale.png")
print(f"   ✅ Converted to grayscale")

rgba = img.convert("RGBA")
rgba.save("test/output_07_rgba.png")
print(f"   ✅ Converted to RGBA")

# 7. Properties
print("\n7. Properties")
print(f"   Size: {img.size}")
print(f"   Width: {img.width}")
print(f"   Height: {img.height}")
print(f"   Mode: {img.mode}")
print(f"   ✅ All properties accessible")

# 8. Copy
print("\n8. Copy")
copy = img.copy()
print(f"   ✅ Image copied")

# 9. Create new image
print("\n9. New Image")
new_img = Image.new("RGB", (200, 200), (255, 0, 0))
new_img.save("test/output_08_new.png")
print(f"   ✅ Created red 200x200 image")

# 10. Paste
print("\n10. Paste")
base = Image.new("RGB", (300, 300), (255, 255, 255))
overlay = Image.new("RGB", (100, 100), (0, 0, 255))
result = base.paste(overlay, (100, 100))
result.save("test/output_09_paste.png")
print(f"   ✅ Pasted blue square on white canvas")

print("\n" + "=" * 60)
print("✅ All basic Pillow operations working with imgrs!")
print("=" * 60)


# Replace this:
# from PIL import Image

# With this:
from imgrs import Image

# Your existing Pillow code works unchanged!
img = Image.open("examples/img/gradient.png")
img = img.resize((400, 300))
img.save("test/resized.jpg")

print("✅ Successfully opened, resized, and saved!")
print(f"   Final size: {img.width}x{img.height}")
print("   Saved to: test/resized.jpg")

"""
Imgrs .show() Method Demo
=========================

Demonstrates the new img.show() feature that displays images
in the default system viewer.
"""

import time

import imgrs

print("🖼️  Imgrs .show() Method Demo")
print("=" * 60)

# Create a simple test image
print("\n1. Creating test image with shapes...")
img = imgrs.Image.new("RGB", (400, 400), (240, 240, 250))

# Add some colorful shapes
img = img.draw_circle(100, 100, 80, (255, 0, 0, 255))
img = img.draw_star(300, 100, 70, 35, 5, (255, 215, 0, 255))
img = img.draw_rectangle(50, 250, 100, 100, (0, 255, 0, 255))
img = img.draw_triangle(250, 250, 350, 250, 300, 350, (0, 100, 255, 255))

# Note: Text functionality removed due to Cairo dependency
# Previously: img = img.add_text("imgrs.show()", (80, 180), size=30, color=(50, 50, 50, 255))

print("✅ Image created")

# Show the image
print("\n2. Displaying image with img.show()...")
print("   Opening default image viewer...")

try:
    img.show()
    print("✅ Image displayed!")
    print("   (Image should open in your default viewer)")
except RuntimeError as e:
    print(f"❌ Error: {e}")
    print("   Note: show() requires a GUI environment")

# Wait a moment for the viewer to open
print("\n⏸️  Waiting 2 seconds...")
time.sleep(2)

# Test with filtered image
print("\n3. Testing with filters...")
img_filtered = img.blur(3).sharpen(1.5)
img_filtered = img_filtered.brightness(30)

print("   Showing filtered image...")
try:
    img_filtered.show()
    print("✅ Filtered image displayed!")
except RuntimeError as e:
    print(f"❌ Error: {e}")

print("\n" + "=" * 60)
print("✅ Demo Complete!")
print("\n💡 Usage:")
print("   img = imgrs.Image.open('photo.jpg')")
print("   img = img.blur(5)")
print("   img.show()  # Opens in default viewer")
print("\n📝 Note:")
print("   - Works on Windows, macOS, and Linux")
print("   - Requires GUI environment")
print("   - Uses system's default image viewer")
print("=" * 60)

"""
Test script for img.show() method
"""

import sys

import imgrs

print("Testing img.show() method...")
print("=" * 50)

# Create a test image
img = imgrs.Image.new("RGB", (300, 200), "white")
img = img.draw_rectangle(50, 50, 200, 100, (255, 0, 0, 255))
img = img.add_text("TEST", 120, 85, size=40, color=(255, 255, 255, 255))

print("\n✅ Test image created")
print("   Size: 300x200")
print("   Content: Red rectangle with 'TEST' text")

# Test show() method
print("\n🖼️  Calling img.show()...")

try:
    img.show()
    print("✅ img.show() executed successfully!")
    print("   Image should have opened in your default viewer")
    print(f"   Platform: {sys.platform}")

except RuntimeError as e:
    print(f"❌ Error: {e}")
    print("   This is expected if running in a non-GUI environment")
    print("   (e.g., SSH session, Docker container, CI/CD)")

except Exception as e:
    print(f"❌ Unexpected error: {e}")
    sys.exit(1)

print("\n" + "=" * 50)
print("✅ Test complete!")

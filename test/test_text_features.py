"""
Text rendering features - imgrs exclusive!
(Pillow requires ImageDraw, imgrs has it built-in)
"""

from imgrs import Image

print("Testing Text Features (imgrs built-in)")
print("=" * 60)

# Create canvas
canvas = Image.new("RGBA", (800, 600), (255, 255, 255, 255))

# 1. Basic text
print("\n1. Basic Text")
canvas = canvas.add_text("Hello World", (50, 50), size=48, color=(0, 0, 0, 255))
canvas.save("test/text_01_basic.png")
print("   ✅ Basic text rendered")

# 2. Text with styling
print("\n2. Styled Text (outline + shadow)")
canvas = canvas.add_text_styled(
    "STYLED TEXT",
    (50, 120),
    size=56,
    color=(255, 255, 255, 255),
    outline=(0, 0, 0, 255, 3.0),
    shadow=(3, 3, 100, 100, 100, 180)
)
canvas.save("test/text_02_styled.png")
print("   ✅ Text with outline and shadow")

# 3. Centered text (using textbox)
print("\n3. Centered Text (with textbox)")
title = "Centered Title"
w, h = Image.get_text_size(title, size=64)
x = (canvas.width - w) // 2
canvas = canvas.add_text(title, (x, 250), size=64, color=(0, 0, 128, 255))
canvas.save("test/text_03_centered.png")
print(f"   ✅ Text centered using textbox ({w}x{h})")

# 4. Multi-line text
print("\n4. Multi-line Text")
multiline = "Line 1\nLine 2\nLine 3"
canvas = canvas.add_text_multiline(multiline, (50, 350), size=32, 
                                   color=(128, 0, 128, 255), line_spacing=1.5)
canvas.save("test/text_04_multiline.png")
print("   ✅ Multi-line text rendered")

# 5. Text with background
print("\n5. Text with Background")
canvas = canvas.add_text_styled(
    "With Background",
    (50, 500),
    size=40,
    color=(255, 255, 255, 255),
    background=(0, 128, 255, 200)
)
canvas.save("test/text_05_background.png")
print("   ✅ Text with background")

# Save final canvas
canvas.save("test/text_complete.png")

print("\n" + "=" * 60)
print("✅ All text features working!")
print("   (Note: Pillow requires ImageDraw import)")
print("   (imgrs has text built-in - easier to use!)")
print("=" * 60)


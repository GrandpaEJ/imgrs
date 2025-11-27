import traceback

import imgrs._core as _core
from imgrs import Image

print("Available functions in _core:")
print([attr for attr in dir(_core) if not attr.startswith("_")])

print("\nTesting create_quadrilateral_py...")
try:
    ix_direct = _core.create_quadrilateral_py(
        (0, 0), (100, 0), (120, 50), (20, 50), (0, 0, 128, 255)
    )
    print(f"ix_direct type: {type(ix_direct)}")
    print(f"ix_direct: {ix_direct}")
    print(f"ix_direct has _rust_image: {hasattr(ix_direct, '_rust_image')}")
except Exception as e:
    print(f"Direct call failed: {e}")
    traceback.print_exc()

# Test the method
print("\nTesting method call...")
ix = Image.quadrilateral((0, 0), (100, 0), (120, 50), (20, 50), (0, 0, 128, 255))

# Check type
print(f"ix type: {type(ix)}")
print(f"ix: {ix}")
print(f"ix has _rust_image: {hasattr(ix, '_rust_image')}")

# Create base image
img = Image.new("RGBA", (200, 200), (255, 255, 255, 255))
print(f"img type: {type(img)}")
print(f"img has _rust_image: {hasattr(img, '_rust_image')}")

# Paste the quadrilateral
result = img.paste(ix, (50, 50))

# Save to verify
result.save("test_quadrilateral.png")
print("Test successful! Quadrilateral created and pasted.")

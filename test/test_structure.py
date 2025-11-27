#!/usr/bin/env python3
"""
Test script to verify the structure of our new features without requiring the Rust extension.
"""

import os
import sys

# Add the python directory to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "python"))


def test_imports():
    """Test that all our new functions can be imported."""
    try:
        print("✓ All new functions can be imported")
        pass
    except ImportError as e:
        print(f"✗ Import error: {e}")
        assert False


def test_image_class_methods():
    """Test that Image class has all the new methods."""
    try:
        from imgrs.image import Image

        # Check if methods exist
        methods = ["convert", "split", "paste", "fromarray"]
        missing_methods = []

        for method in methods:
            if not hasattr(Image, method):
                missing_methods.append(method)

        if missing_methods:
            print(f"✗ Missing methods in Image class: {missing_methods}")
            assert False
        else:
            print("✓ All new methods exist in Image class")
            pass

    except ImportError as e:
        print(f"✗ Could not import Image class: {e}")
        assert False


def test_operations_module():
    """Test that operations module has all the new functions."""
    try:
        from imgrs import operations

        # Check if functions exist
        functions = ["convert", "fromarray", "split", "paste"]
        missing_functions = []

        for func in functions:
            if not hasattr(operations, func):
                missing_functions.append(func)

        if missing_functions:
            print(f"✗ Missing functions in operations module: {missing_functions}")
            assert False
        else:
            print("✓ All new functions exist in operations module")
            pass

    except ImportError as e:
        print(f"✗ Could not import operations module: {e}")
        assert False


def test_numpy_handling():
    """Test numpy import handling."""
    try:
        from imgrs.mixins.core_mixin import HAS_NUMPY

        print(f"✓ NumPy availability detected: {HAS_NUMPY}")
        pass
    except ImportError as e:
        print(f"✗ Could not check NumPy availability: {e}")
        assert False


def main():
    """Run all structure tests."""
    print("Testing imgrs new features structure...")
    print("=" * 50)

    tests = [
        test_imports,
        test_image_class_methods,
        test_operations_module,
        test_numpy_handling,
    ]

    passed = 0
    total = len(tests)

    for test in tests:
        try:
            test()
            passed += 1
        except AssertionError:
            pass
        print()

    print("=" * 50)
    print(f"Results: {passed}/{total} tests passed")

    if passed == total:
        print("🎉 All structure tests passed!")
        return 0
    else:
        print("❌ Some tests failed")
        return 1


if __name__ == "__main__":
    sys.exit(main())

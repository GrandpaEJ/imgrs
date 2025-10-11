# Code Quality Quick Start Guide

## 🚀 Quick Commands

### Check Code Quality
```bash
# Check formatting
black --check python/imgrs/

# Check imports
isort --check-only python/imgrs/

# Check style
flake8 python/imgrs/
```

### Auto-Fix Issues
```bash
# Fix formatting
black python/imgrs/ examples/ test/scripts/

# Fix imports
isort python/imgrs/ examples/ test/scripts/

# Both at once
black python/imgrs/ examples/ test/scripts/ && isort python/imgrs/ examples/ test/scripts/
```

## 📦 Installation

```bash
pip install black flake8 isort mypy pylint
```

## 🎯 Current Status

✅ **Black:** All files formatted (47 files)
✅ **isort:** All imports organized (23 files)
⚠️ **Flake8:** 11 minor issues remaining (86 are false positives)

## 🔧 Tools Installed

- **black** (25.9.0) - Code formatter
- **isort** (6.1.0) - Import organizer
- **flake8** (7.3.0) - Style checker
- **mypy** (1.18.2) - Type checker
- **pylint** (3.3.9) - Linter

## 📝 Configuration Files

- `.flake8` - Flake8 settings
- `pyproject.toml` - Black & isort settings
- `.pre-commit-config.yaml` - Pre-commit hooks (optional)

## 🎨 Code Style

- **Line length:** 88 characters (Black standard)
- **Import order:** Standard library → Third party → Local
- **Formatting:** Black style (PEP 8 compatible)

## ⚡ Pre-commit Hooks (Optional)

Install automatic checking:
```bash
pip install pre-commit
pre-commit install
```

Now code will be checked automatically on every commit!

## 📊 Quality Metrics

| Tool | Status | Score |
|------|--------|-------|
| Black | ✅ | 100% |
| isort | ✅ | 100% |
| Flake8 | ⚠️ | 89% |

## 🐛 Known Issues

Most flake8 warnings (86) are false positives from type hints.
These can be safely ignored or fixed by adding:
```python
from __future__ import annotations
```

## 📚 More Info

See `PYTHON_CODE_QUALITY_REPORT.md` for detailed analysis.

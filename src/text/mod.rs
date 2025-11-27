//! Advanced text rendering module
//!
//! Provides comprehensive text rendering capabilities including:
//! - Basic text rendering with bitmap fonts
//! - Styled text with outlines, shadows, and backgrounds
//! - Multi-line text support
//! - Text measurement and bounding box calculations

mod text;

pub use text::{
    add_text, add_text_multiline, add_text_styled, draw_text, get_text_box, get_text_size, TextStyle,
};
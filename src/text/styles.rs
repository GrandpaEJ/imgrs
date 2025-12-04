/// Text styling types and options

/// Text alignment options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// Font weight options
#[derive(Debug, Clone, Copy, PartialEq)]
    #[allow(dead_code)]
pub enum FontWeight {
    Normal,
    Bold,
    Light,
}

/// Comprehensive text styling options
#[derive(Debug, Clone)]
pub struct TextStyle {
    /// Font size in pixels
    pub size: f32,
    
    /// Text color (R, G, B, A)
    pub color: (u8, u8, u8, u8),
    
    /// Background color (optional)
    pub background: Option<(u8, u8, u8, u8)>,
    
    /// Text alignment
    pub align: TextAlign,
    
    /// Font weight
    pub weight: FontWeight,
    
    /// Line spacing multiplier (default: 1.2)
    pub line_spacing: f32,
    
    /// Letter spacing in pixels (default: 0.0)
    #[allow(dead_code)]
    pub letter_spacing: f32,
    
    /// Add outline/stroke
    pub outline: Option<(u8, u8, u8, u8, f32)>, // color + width
    
    /// Add shadow
    pub shadow: Option<(i32, i32, u8, u8, u8, u8)>, // offset_x, offset_y, color
    
    /// Opacity (0.0 to 1.0)
    pub opacity: f32,
    
    /// Maximum width for text wrapping (None = no wrap)
    pub max_width: Option<u32>,
    
    /// Rotation angle in degrees
    pub rotation: f32,
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            size: 32.0,
            color: (0, 0, 0, 255),
            background: None,
            align: TextAlign::Left,
            weight: FontWeight::Normal,
            line_spacing: 1.2,
            letter_spacing: 0.0,
            outline: None,
            shadow: None,
            opacity: 1.0,
            max_width: None,
            rotation: 0.0,
        }
    }
}

impl TextStyle {
    /// Create new text style with defaults
    pub fn new() -> Self {
        Default::default()
    }
    
    /// Set font size
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    
    /// Set text color
    pub fn with_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.color = (r, g, b, a);
        self
    }
    
    /// Set background color
    pub fn with_background(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background = Some((r, g, b, a));
        self
    }
    
    /// Set alignment
    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }
    
    /// Set font weight
    #[allow(dead_code)]
    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }
    
    /// Add outline
    pub fn with_outline(mut self, r: u8, g: u8, b: u8, a: u8, width: f32) -> Self {
        self.outline = Some((r, g, b, a, width));
        self
    }
    
    /// Add shadow
    pub fn with_shadow(mut self, offset_x: i32, offset_y: i32, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.shadow = Some((offset_x, offset_y, r, g, b, a));
        self
    }
    
    /// Set opacity
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.max(0.0).min(1.0);
        self
    }
    
    /// Set maximum width for wrapping
    pub fn with_max_width(mut self, width: u32) -> Self {
        self.max_width = Some(width);
        self
    }
    
    /// Set rotation
    pub fn with_rotation(mut self, degrees: f32) -> Self {
        self.rotation = degrees;
        self
    }
}

/// Text anchor positions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAnchor {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    BaselineLeft,
    BaselineCenter,
    BaselineRight,
}

impl Default for TextAnchor {
    fn default() -> Self {
        TextAnchor::TopLeft
    }
}

impl TextAnchor {
    /// Parse from 2-char string (e.g., "lt", "mm", "rb")
    pub fn from_str(s: &str) -> Option<Self> {
        if s.len() != 2 {
            return None;
        }
        
        let chars: Vec<char> = s.chars().collect();
        let v = chars[0]; // Vertical: l, m, s, b
        let h = chars[1]; // Horizontal: l, m, r
        
        match (v, h) {
            ('l', 'l') => Some(TextAnchor::TopLeft),
            ('l', 'm') => Some(TextAnchor::TopCenter),
            ('l', 'r') => Some(TextAnchor::TopRight),
            ('m', 'l') => Some(TextAnchor::MiddleLeft),
            ('m', 'm') => Some(TextAnchor::MiddleCenter),
            ('m', 'r') => Some(TextAnchor::MiddleRight),
            ('b', 'l') => Some(TextAnchor::BottomLeft),
            ('b', 'm') => Some(TextAnchor::BottomCenter),
            ('b', 'r') => Some(TextAnchor::BottomRight),
            ('s', 'l') => Some(TextAnchor::BaselineLeft),
            ('s', 'm') => Some(TextAnchor::BaselineCenter),
            ('s', 'r') => Some(TextAnchor::BaselineRight),
            _ => None,
        }
    }
}

/// Style for text box rendering
#[derive(Debug, Clone)]
pub struct TextBoxStyle {
    pub text_style: TextStyle,
    pub vertical_align: TextAlign, // Reusing TextAlign for vertical alignment (Left=Top, Center=Middle, Right=Bottom)
    pub overflow: bool, // true = visible/overflow, false = clip
}
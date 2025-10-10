/// Emoji type presets with Unicode characters
/// Provides easy access to commonly used emojis

#[derive(Debug, Clone, Copy)]
pub enum EmojiType {
    // Smileys & Emotion
    Smile,
    Grin,
    Joy,
    Laughing,
    HeartEyes,
    StarStruck,
    ThinkingFace,
    Wink,
    Blush,
    Cool,
    
    // Hearts
    RedHeart,
    BlueHeart,
    GreenHeart,
    YellowHeart,
    PurpleHeart,
    OrangeHeart,
    #[allow(dead_code)]
    BlackHeart,
    #[allow(dead_code)]
    WhiteHeart,
    #[allow(dead_code)]
    BrokenHeart,
    #[allow(dead_code)]
    SparklingHeart,
    
    // Gestures
    ThumbsUp,
    ThumbsDown,
    OkHand,
    Victory,
    RaisedHands,
    Clap,
    Wave,
    PointRight,
    PointLeft,
    Fire,
    
    // Nature
    Sun,
    Moon,
    Star,
    Cloud,
    Rainbow,
    Flower,
    Rose,
    Tree,
    Leaf,
    Sparkles,
    
    // Food
    Pizza,
    Burger,
    Cake,
    IceCream,
    Coffee,
    Beer,
    Fruit,
    Candy,
    Cookie,
    Donut,
    
    // Activities
    Soccer,
    Basketball,
    Party,
    Gift,
    Trophy,
    Medal,
    Camera,
    Music,
    Art,
    GameController,
    
    // Symbols
    Check,
    Cross,
    Question,
    Exclamation,
    Warning,
    Prohibited,
    Recycle,
    Atom,
    Infinity,
    ArrowRight,
}

impl EmojiType {
    /// Get the Unicode character for this emoji
    pub fn as_str(&self) -> &'static str {
        match self {
            // Smileys & Emotion
            EmojiType::Smile => "😊",
            EmojiType::Grin => "😁",
            EmojiType::Joy => "😂",
            EmojiType::Laughing => "🤣",
            EmojiType::HeartEyes => "😍",
            EmojiType::StarStruck => "🤩",
            EmojiType::ThinkingFace => "🤔",
            EmojiType::Wink => "😉",
            EmojiType::Blush => "😊",
            EmojiType::Cool => "😎",
            
            // Hearts
            EmojiType::RedHeart => "❤️",
            EmojiType::BlueHeart => "💙",
            EmojiType::GreenHeart => "💚",
            EmojiType::YellowHeart => "💛",
            EmojiType::PurpleHeart => "💜",
            EmojiType::OrangeHeart => "🧡",
            EmojiType::BlackHeart => "🖤",
            EmojiType::WhiteHeart => "🤍",
            EmojiType::BrokenHeart => "💔",
            EmojiType::SparklingHeart => "💖",
            
            // Gestures
            EmojiType::ThumbsUp => "👍",
            EmojiType::ThumbsDown => "👎",
            EmojiType::OkHand => "👌",
            EmojiType::Victory => "✌️",
            EmojiType::RaisedHands => "🙌",
            EmojiType::Clap => "👏",
            EmojiType::Wave => "👋",
            EmojiType::PointRight => "👉",
            EmojiType::PointLeft => "👈",
            EmojiType::Fire => "🔥",
            
            // Nature
            EmojiType::Sun => "☀️",
            EmojiType::Moon => "🌙",
            EmojiType::Star => "⭐",
            EmojiType::Cloud => "☁️",
            EmojiType::Rainbow => "🌈",
            EmojiType::Flower => "🌸",
            EmojiType::Rose => "🌹",
            EmojiType::Tree => "🌲",
            EmojiType::Leaf => "🍃",
            EmojiType::Sparkles => "✨",
            
            // Food
            EmojiType::Pizza => "🍕",
            EmojiType::Burger => "🍔",
            EmojiType::Cake => "🎂",
            EmojiType::IceCream => "🍦",
            EmojiType::Coffee => "☕",
            EmojiType::Beer => "🍺",
            EmojiType::Fruit => "🍎",
            EmojiType::Candy => "🍬",
            EmojiType::Cookie => "🍪",
            EmojiType::Donut => "🍩",
            
            // Activities
            EmojiType::Soccer => "⚽",
            EmojiType::Basketball => "🏀",
            EmojiType::Party => "🎉",
            EmojiType::Gift => "🎁",
            EmojiType::Trophy => "🏆",
            EmojiType::Medal => "🥇",
            EmojiType::Camera => "📷",
            EmojiType::Music => "🎵",
            EmojiType::Art => "🎨",
            EmojiType::GameController => "🎮",
            
            // Symbols
            EmojiType::Check => "✅",
            EmojiType::Cross => "❌",
            EmojiType::Question => "❓",
            EmojiType::Exclamation => "❗",
            EmojiType::Warning => "⚠️",
            EmojiType::Prohibited => "🚫",
            EmojiType::Recycle => "♻️",
            EmojiType::Atom => "⚛️",
            EmojiType::Infinity => "∞",
            EmojiType::ArrowRight => "➡️",
        }
    }
    
    /// Get all available emoji types
    #[allow(dead_code)]
    pub fn all() -> Vec<EmojiType> {
        vec![
            // Smileys
            EmojiType::Smile, EmojiType::Grin, EmojiType::Joy,
            EmojiType::Laughing, EmojiType::HeartEyes, EmojiType::StarStruck,
            EmojiType::ThinkingFace, EmojiType::Wink, EmojiType::Blush,
            EmojiType::Cool,
            // Hearts
            EmojiType::RedHeart, EmojiType::BlueHeart, EmojiType::GreenHeart,
            EmojiType::YellowHeart, EmojiType::PurpleHeart, EmojiType::OrangeHeart,
            EmojiType::BlackHeart, EmojiType::WhiteHeart, EmojiType::BrokenHeart,
            EmojiType::SparklingHeart,
            // Gestures
            EmojiType::ThumbsUp, EmojiType::ThumbsDown, EmojiType::OkHand,
            EmojiType::Victory, EmojiType::RaisedHands, EmojiType::Clap,
            EmojiType::Wave, EmojiType::PointRight, EmojiType::PointLeft,
            EmojiType::Fire,
            // Nature
            EmojiType::Sun, EmojiType::Moon, EmojiType::Star,
            EmojiType::Cloud, EmojiType::Rainbow, EmojiType::Flower,
            EmojiType::Rose, EmojiType::Tree, EmojiType::Leaf,
            EmojiType::Sparkles,
            // Food
            EmojiType::Pizza, EmojiType::Burger, EmojiType::Cake,
            EmojiType::IceCream, EmojiType::Coffee, EmojiType::Beer,
            EmojiType::Fruit, EmojiType::Candy, EmojiType::Cookie,
            EmojiType::Donut,
            // Activities
            EmojiType::Soccer, EmojiType::Basketball, EmojiType::Party,
            EmojiType::Gift, EmojiType::Trophy, EmojiType::Medal,
            EmojiType::Camera, EmojiType::Music, EmojiType::Art,
            EmojiType::GameController,
            // Symbols
            EmojiType::Check, EmojiType::Cross, EmojiType::Question,
            EmojiType::Exclamation, EmojiType::Warning, EmojiType::Prohibited,
            EmojiType::Recycle, EmojiType::Atom, EmojiType::Infinity,
            EmojiType::ArrowRight,
        ]
    }
    
    /// Get emoji type by name (case-insensitive)
    pub fn from_name(name: &str) -> Option<EmojiType> {
        match name.to_lowercase().as_str() {
            "smile" => Some(EmojiType::Smile),
            "grin" => Some(EmojiType::Grin),
            "joy" => Some(EmojiType::Joy),
            "laughing" => Some(EmojiType::Laughing),
            "hearteyes" | "heart_eyes" => Some(EmojiType::HeartEyes),
            "starstruck" | "star_struck" => Some(EmojiType::StarStruck),
            "thinking" | "thinkingface" => Some(EmojiType::ThinkingFace),
            "wink" => Some(EmojiType::Wink),
            "blush" => Some(EmojiType::Blush),
            "cool" => Some(EmojiType::Cool),
            
            "redheart" | "heart" => Some(EmojiType::RedHeart),
            "blueheart" => Some(EmojiType::BlueHeart),
            "greenheart" => Some(EmojiType::GreenHeart),
            "yellowheart" => Some(EmojiType::YellowHeart),
            "purpleheart" => Some(EmojiType::PurpleHeart),
            "orangeheart" => Some(EmojiType::OrangeHeart),
            
            "thumbsup" | "thumbs_up" | "like" => Some(EmojiType::ThumbsUp),
            "thumbsdown" | "thumbs_down" => Some(EmojiType::ThumbsDown),
            "ok" | "okhand" => Some(EmojiType::OkHand),
            "victory" | "peace" => Some(EmojiType::Victory),
            "raisedhands" | "raised_hands" => Some(EmojiType::RaisedHands),
            "clap" => Some(EmojiType::Clap),
            "wave" => Some(EmojiType::Wave),
            "pointright" | "point_right" => Some(EmojiType::PointRight),
            "pointleft" | "point_left" => Some(EmojiType::PointLeft),
            "fire" => Some(EmojiType::Fire),
            
            "sun" => Some(EmojiType::Sun),
            "moon" => Some(EmojiType::Moon),
            "star" => Some(EmojiType::Star),
            "cloud" => Some(EmojiType::Cloud),
            "rainbow" => Some(EmojiType::Rainbow),
            "flower" => Some(EmojiType::Flower),
            "rose" => Some(EmojiType::Rose),
            "tree" => Some(EmojiType::Tree),
            "leaf" => Some(EmojiType::Leaf),
            "sparkles" => Some(EmojiType::Sparkles),
            
            "pizza" => Some(EmojiType::Pizza),
            "burger" => Some(EmojiType::Burger),
            "cake" => Some(EmojiType::Cake),
            "icecream" | "ice_cream" => Some(EmojiType::IceCream),
            "coffee" => Some(EmojiType::Coffee),
            "beer" => Some(EmojiType::Beer),
            "fruit" | "apple" => Some(EmojiType::Fruit),
            "candy" => Some(EmojiType::Candy),
            "cookie" => Some(EmojiType::Cookie),
            "donut" => Some(EmojiType::Donut),
            
            "soccer" | "football" => Some(EmojiType::Soccer),
            "basketball" => Some(EmojiType::Basketball),
            "party" => Some(EmojiType::Party),
            "gift" => Some(EmojiType::Gift),
            "trophy" => Some(EmojiType::Trophy),
            "medal" => Some(EmojiType::Medal),
            "camera" => Some(EmojiType::Camera),
            "music" => Some(EmojiType::Music),
            "art" => Some(EmojiType::Art),
            "gamecontroller" | "game_controller" | "game" => Some(EmojiType::GameController),
            
            "check" | "checkmark" => Some(EmojiType::Check),
            "cross" | "x" => Some(EmojiType::Cross),
            "question" => Some(EmojiType::Question),
            "exclamation" | "!" => Some(EmojiType::Exclamation),
            "warning" => Some(EmojiType::Warning),
            "prohibited" | "no" => Some(EmojiType::Prohibited),
            "recycle" => Some(EmojiType::Recycle),
            "atom" => Some(EmojiType::Atom),
            "infinity" => Some(EmojiType::Infinity),
            "arrowright" | "arrow_right" | "arrow" => Some(EmojiType::ArrowRight),
            
            _ => None,
        }
    }
}


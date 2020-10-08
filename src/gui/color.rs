

#[derive(Debug, Copy, Clone, Default)]
pub struct Color {
    pub red : u8,
    pub green: u8,
    pub blue: u8
}

impl Color {
    pub const fn from_rgb(r:u8,g:u8,b:u8) -> Color {
        Color {
            red: r,
            green: g,
            blue: b
        }
    }

    pub const WHITE: Color = Color::from_rgb(255,255,255);

    pub const LIGHT_GRAY: Color = Color::from_rgb(200, 200, 200);
    pub const DARK_GRAY: Color = Color::from_rgb(50, 50, 50);

    pub const RED: Color = Color::from_rgb(255, 0, 0);
    pub const LIGHT_RED: Color = Color::from_rgb(255,200,200);
    pub const DARKER_RED: Color = Color::from_rgb(255,100,100);
    pub const DARK_RED: Color = Color::from_rgb(200, 0, 0);
}

pub struct DarkTheme {
    
}
impl DarkTheme {
    pub const BACKGROUND: Color = Color::from_rgb(0x1E,0x1E,0x1E);
    pub const BACKGROUND_LIGHT: Color = Color::from_rgb(0x25, 0x25, 0x26);
    pub const TAB_INACTIVE: Color = Color::from_rgb(0x2D, 0x2D, 0x2D);
    pub const TAB_ACTIVE: Color = Color::from_rgb(0x1e, 0x1e, 0x1e);
    pub const TITLE_BAR: Color = Color::from_rgb(0x3c, 0x3c, 0x3c);
    pub const SCROLL_FOREGROUND: Color = Color::from_rgb(0x4f, 0x4f, 0x4f);
    pub const SCROLL_BACKGROUND: Color = Color::from_rgb(0x1e, 0x1e, 0x1e);
    pub const TEXT: Color = Color::WHITE;
    pub const HIGHLIGHT: Color = Color::from_rgb(0x26, 0x4f, 0x78);
}
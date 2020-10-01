

#[derive(Debug, Copy, Clone)]
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

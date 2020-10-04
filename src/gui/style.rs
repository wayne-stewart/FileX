
use crate::gui::color::Color;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HorizontalAlign {
    Left,
    //Right,
    Center
}

#[derive(Debug, Copy, Clone)]
pub enum VerticalAlign {
    Center,
    Bottom,
    Top
}

#[derive(Debug, Copy, Clone)]
pub struct BoxSize {
    pub left:i32,
    pub top:i32,
    pub right:i32,
    pub bottom:i32
}

impl BoxSize {
    pub const fn default() -> BoxSize {
        BoxSize {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0
        }
    }
    pub const fn single(s: i32) -> BoxSize {
        BoxSize {
            left: s,
            top: s,
            right: s,
            bottom: s
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BoxStyle {
    pub border_color: Color,
    pub border_size: BoxSize,
    pub padding_size: BoxSize,
    pub background_color: Color,
    pub text_color: Color,
    pub highlight_color: Color,
    pub text_highlight_color: Color,
    pub font_size: f32,
    pub vertical_align: VerticalAlign,
    pub horizontal_align: HorizontalAlign

}

impl BoxStyle {
    pub const fn default() -> BoxStyle {
        BoxStyle {
            border_color: Color::RED,
            border_size: BoxSize::single(2),
            padding_size: BoxSize::single(2),
            background_color: Color::LIGHT_RED,
            text_color: Color::RED,
            highlight_color: Color::DARK_RED,
            text_highlight_color: Color::WHITE,
            font_size: 30.0,
            vertical_align: VerticalAlign::Center,
            horizontal_align: HorizontalAlign::Left
        }
    }
    pub const fn button_default() -> BoxStyle {
        let mut style = BoxStyle::default();
        style.horizontal_align = HorizontalAlign::Center;
        style
    }
    pub const fn button_default_hot() -> BoxStyle {
        let mut style = BoxStyle::button_default();
        style.background_color = Color::DARKER_RED;
        style
    }
    pub const fn button_default_active() -> BoxStyle {
        let mut style = BoxStyle::button_default();
        style.background_color = Color::DARK_RED;
        style
    }
    pub const fn textbox_default() -> BoxStyle {
        let style = BoxStyle::default();
        style
    }
}


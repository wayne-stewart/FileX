#![allow(unused_parens)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub mod button;
pub mod draw;
pub mod textbox;
pub mod color;
pub mod control;
pub mod keyboard;
pub mod mouse;

use color::Color;

pub enum Cursor {
    NotSet,
    Arrow,
    IBeam,
    Hand
}

#[repr(C, align(4))]
#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    blue: u8,
    green: u8,
    red: u8,
    alpha: u8
}

impl Pixel {
    pub fn default() -> Pixel {
        Pixel {
            blue: 0,
            green: 0,
            red: 0,
            alpha: 0
        }
    }
}



#[derive(Debug, Copy, Clone)]
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
pub struct Rect {
    pub x:i32,
    pub y:i32,
    pub w:i32,
    pub h:i32
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

pub struct PixelBuffer {
    pub pixels: Vec<Pixel>,
    pub width: i32,
    pub height: i32
}



pub fn is_point_in_rect_a(x:i32, y:i32, left:i32, top:i32, right:i32, bottom:i32) -> bool {
    x > left && x < right && y > top && y < bottom
}

pub fn is_point_in_rect(x: i32, y: i32, bounds: Rect) -> bool {
    let right = bounds.x + bounds.w;
    let bottom = bounds.y + bounds.h;
    is_point_in_rect_a(x,y,bounds.x,bounds.y,right,bottom)
}




// fn choose<T>(cmp: bool, option1: T, option2: T) -> T {
//     match cmp {
//         true => option1,
//         false => option2
//     }
// }


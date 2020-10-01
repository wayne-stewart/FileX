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
pub mod style;

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
pub struct Rect {
    pub x:i32,
    pub y:i32,
    pub w:i32,
    pub h:i32
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


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
pub mod view;

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


#[derive(Default, Debug, Copy, Clone)]
pub struct Rect {
    pub x:i32,
    pub y:i32,
    pub w:i32,
    pub h:i32
}

// impl Rect {
//     pub fn default() -> Rect { Rect { x:0, y: 0, w: 0, h: 0 } }
// }

#[derive(Debug, Copy, Clone)]
pub enum BoundsField {
    INT(i32),
    FLOAT(f32)
}

impl Default for BoundsField {
    fn default() -> BoundsField { BoundsField::INT(0) }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Bounds {
    pub x: BoundsField,
    pub y: BoundsField,
    pub w: BoundsField,
    pub h: BoundsField
}

impl Bounds {
    pub fn get_rect(&self, width: i32, height: i32) -> Rect {
        Rect {
            x: match self.x { BoundsField::INT(a) => a, BoundsField::FLOAT(a) => (a * width as f32) as i32 },
            y: match self.y { BoundsField::INT(a) => a, BoundsField::FLOAT(a) => (a * height as f32) as i32 },
            w: match self.w { BoundsField::INT(a) => a, BoundsField::FLOAT(a) => (a * width as f32) as i32 },
            h: match self.h { BoundsField::INT(a) => a, BoundsField::FLOAT(a) => (a * height as f32) as i32 }
        }
    }

    pub fn int(x: i32, y: i32, w: i32, h: i32) -> Bounds {
        Bounds {
            x: BoundsField::INT(x),
            y: BoundsField::INT(y),
            w: BoundsField::INT(w),
            h: BoundsField::INT(h)
        }
    }

    pub fn variable_horizontal(x: f32, y: i32, w: f32, h: i32) -> Bounds {
        Bounds {
            x: BoundsField::FLOAT(x),
            y: BoundsField::INT(y),
            w: BoundsField::FLOAT(w),
            h: BoundsField::INT(h)
        }
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



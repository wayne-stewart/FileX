#![allow(unused_parens)]

pub trait Control {
    fn get_bounds(&self) -> Rect;
    fn get_hot(&self) -> bool;
    fn set_hot(&mut self, hit: bool);

    fn hit_check(&mut self, mouse_x: i32, mouse_y: i32) -> (bool, bool) {
        let hit = is_point_in_rect(mouse_x, mouse_y, self.get_bounds());
        let mut hot_changed = false;
        if self.get_hot() != hit {
            hot_changed = true;
            self.set_hot(hit);
        }
        (hot_changed, hit)
    }

    fn get_style<'a>(&'a self) -> &'a BoxStyle;
}

pub struct Button {
    pub text: &'static str,
    pub bounds: Rect,
    pub hot: bool,
    pub active: bool,
    pub on_click: Option<ButtonClick>,
    pub click_count: i32,
    pub style: BoxStyle,
    pub style_hot: BoxStyle,
    pub style_active: BoxStyle
}

impl Control for Button {
    fn get_bounds(&self) -> Rect { self.bounds }
    fn get_hot(&self) -> bool { self.hot }
    fn set_hot(&mut self, hit: bool) { self.hot = hit }

    fn get_style<'a>(&'a self) -> &'a BoxStyle {
        if self.active {
            &self.style_active
        }
        else if self.hot {
            &self.style_hot
        }
        else {
            &self.style
        }
    }
}

pub struct TextBox {
    pub text: Option<std::string::String>,
    pub placeholder: &'static str,
    pub bounds: Rect,
    pub hot: bool,
    pub active: bool,
    pub cursor_index: i32,
    pub style: BoxStyle
}

impl TextBox {
    pub fn set_text(&mut self, text: &str) {
        match &mut self.text {
            None => {
                self.text = Some(std::string::String::from(text));
            },
            Some(string) => {
                string.clear();
                string.push_str(text);
            }
        }
    }
}

impl Control for TextBox {
    fn get_bounds(&self) -> Rect { self.bounds }
    fn get_hot(&self) -> bool { self.hot }
    fn set_hot(&mut self, hit: bool) { self.hot = hit }

    fn get_style<'a>(&'a self) -> &'a BoxStyle {
        &self.style
    }
}

pub enum Cursor {
    NotSet,
    Arrow,
    IBeam,
    Hand
}

type ButtonClick = fn(&mut Button) -> ();

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

    pub const LIGHT_GRAY: Color = Color::from_rgb(200, 200, 200);
    pub const DARK_GRAY: Color = Color::from_rgb(50, 50, 50);

    pub const RED: Color = Color::from_rgb(255, 0, 0);
    pub const LIGHT_RED: Color = Color::from_rgb(255,200,200);
    pub const DARKER_RED: Color = Color::from_rgb(255,100,100);
    pub const DARK_RED: Color = Color::from_rgb(200, 0, 0);
}

#[derive(Debug, Copy, Clone)]
pub enum TextAlign {
    Left,
    Right,
    Center
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
    pub font_size: f32,
    pub vertical_align: TextAlign,
    pub horizontal_align: TextAlign

}

impl BoxStyle {
    pub const fn default() -> BoxStyle {
        BoxStyle {
            border_color: Color::RED,
            border_size: BoxSize::single(2),
            padding_size: BoxSize::single(2),
            background_color: Color::LIGHT_RED,
            text_color: Color::RED,
            font_size: 30.0,
            vertical_align: TextAlign::Center,
            horizontal_align: TextAlign::Left
        }
    }
    pub const fn button_default() -> BoxStyle {
        let mut style = BoxStyle::default();
        style.horizontal_align = TextAlign::Center;
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
        let mut style = BoxStyle::default();
        style
    }
}

pub struct PixelBuffer {
    pub pixels: Vec<Pixel>,
    pub width: i32,
    pub height: i32
}

pub fn handle_mouse_button_down(mouse_x: i32, mouse_y: i32) {
    let buttons = unsafe { &mut crate::APPLICATION_STATE.buttons };
    let textboxes = unsafe { &mut crate::APPLICATION_STATE.textboxes };
    for button in buttons {
        let hit = is_point_in_rect(mouse_x, mouse_y, button.get_bounds());
        handle_button_mouse_down(button, hit);
    }
    for textbox in textboxes {
        let hit = is_point_in_rect(mouse_x, mouse_y, textbox.get_bounds());
        handle_textbox_mouse_down(textbox, hit);
    }
}

pub fn handle_mouse_button_up(mouse_x: i32, mouse_y: i32) {
    let buttons = unsafe { &mut crate::APPLICATION_STATE.buttons };
    //let textboxes = &mut crate::APPLICATION_STATE.textboxes;
    for button in buttons {
        let hit = is_point_in_rect(mouse_x, mouse_y, button.get_bounds());
        handle_button_mouse_up(button, hit);
    }
    // for textbox in textboxes {
    //     let hit = is_point_in_rect(mouse_x, mouse_y, textbox.get_bounds());
    //     handle_textbox_mouse_up(textbox, hit);
    // }
}

pub fn handle_mouse_move(mouse_x: i32, mouse_y: i32) -> (Cursor, bool) {
    let mut is_button_hot = false;
    let mut is_textbox_hot = false;
    let mut should_update_window = false;

    let buttons = unsafe { &mut crate::APPLICATION_STATE.buttons };
    for button in buttons {
        let (hot_changed, is_hot) = button.hit_check(mouse_x, mouse_y);
        if hot_changed { should_update_window = true }
        if is_hot { is_button_hot = true }
    }

    let textboxes = unsafe { &mut crate::APPLICATION_STATE.textboxes };
    for textbox in textboxes {
        let (hot_changed, is_hot) = textbox.hit_check(mouse_x, mouse_y);
        if hot_changed { should_update_window = true }
        if is_hot { is_textbox_hot = true }
    }

    if is_button_hot { 
        return (Cursor::Hand, should_update_window); 
    }
    else if is_textbox_hot { 
        return (Cursor::IBeam, should_update_window); 
    }
    else { 
        return (Cursor::Arrow, should_update_window); 
    }
}

fn handle_textbox_mouse_down(mut textbox: &mut TextBox, hit: bool) {
    textbox.hot = hit;
    textbox.active = hit;
}

fn handle_button_mouse_down(mut button: &mut Button, hit: bool) {
    button.hot = hit;
    button.active = hit;
}

fn handle_button_mouse_up(mut button: &mut Button, hit: bool) {
    button.hot = hit;
    if button.active && hit {
        match button.on_click {
            Some(method) => method(button),
            None => { }
        }
    }
    button.active = false;
}

pub fn draw_textbox(mut buffer: &mut PixelBuffer, textbox: &TextBox, font: &fontdue::Font) {
    let left = textbox.bounds.x;
    let top = textbox.bounds.y;
    let width = textbox.bounds.w;
    let height = textbox.bounds.h;
    let style = textbox.style;
    draw_border_box(&mut buffer, &textbox.bounds, &style);
    fill_text(&mut buffer, 
        textbox.text.as_ref().unwrap(), 
        left + style.border_size.left + style.padding_size.left, 
        top + style.border_size.top + style.padding_size.top, 
        width - style.border_size.left - style.padding_size.left - style.border_size.right - style.padding_size.right, 
        height - style.border_size.top - style.padding_size.top - style.border_size.bottom - style.padding_size.bottom, 
        &font, style.font_size, 
        style.text_color, 
        style.horizontal_align,
        style.vertical_align);
    if textbox.active {
        fill_rect(&mut buffer, 
            left + style.border_size.left + style.padding_size.left,
            top + height - style.border_size.bottom - style.padding_size.bottom,
            10, // width
            2, // height
            style.text_color);
    }
}

pub fn draw_button(mut buffer: &mut PixelBuffer, button: &Button, font: &fontdue::Font) {
    let left = button.bounds.x;
    let top = button.bounds.y;
    let width = button.bounds.w;
    let height = button.bounds.h;
    let style = button.get_style();
    draw_border_box(&mut buffer, &button.bounds, &style);
    fill_text(&mut buffer, 
        button.text, 
        left + style.border_size.left + style.padding_size.left, 
        top + style.border_size.top + style.padding_size.top, 
        width - style.border_size.left - style.padding_size.left - style.border_size.right - style.padding_size.right, 
        height - style.border_size.top - style.padding_size.top - style.border_size.bottom - style.padding_size.bottom, 
        &font, style.font_size, 
        style.text_color, 
        style.horizontal_align,
        style.vertical_align);
}

fn draw_border_box(mut buffer: &mut PixelBuffer, bounds: &Rect, style: &BoxStyle) {
    let left = bounds.x;
    let top = bounds.y;
    let width = bounds.w;
    let height = bounds.h;
    fill_rect(&mut buffer, 
        left + style.border_size.left, 
        top + style.border_size.top, 
        width - style.border_size.left - style.border_size.right, 
        height - style.border_size.top - style.border_size.bottom, 
        style.background_color);
    draw_rect(&mut buffer, left, top, width, height, style.border_size, style.border_color);
}

pub fn fill_rect(buffer: &mut PixelBuffer, left: i32, top: i32, width: i32, height: i32, color: Color) {
    let right = left + width;
    let bottom = top + height;
    let stride = buffer.width;
    for y in top..bottom {
        for x in left..right {
            if  y < buffer.height && x < buffer.width {
                let offset = ((y * stride) + x) as usize;
                let pixel = &mut buffer.pixels[offset];
                pixel.red = color.red;
                pixel.green = color.green;
                pixel.blue = color.blue;
            }
        }
    }
}

fn draw_rect(buffer: &mut PixelBuffer, left: i32, top: i32, width: i32, height: i32, line_width: BoxSize, color: Color) {
    fill_rect(buffer, left, top, width, line_width.top, color); // top
    fill_rect(buffer, left + width - line_width.right, top, line_width.right, height, color); // right
    fill_rect(buffer, left, top + height - line_width.bottom, width, line_width.bottom, color); // bottom
    fill_rect(buffer, left, top, line_width.left, height, color); // left
}

fn alpha_blend_u8(c1: u8, c2 : u8, alpha: u8) -> u8 {
    let c1 = c1 as i32;
    let c2 = c2 as i32;
    let alpha = alpha as i32;
    let inv_alpha = 255 - alpha;
    let result = ((c1 * alpha) + (c2 * inv_alpha));
    return (result / 255) as u8;
}

fn fill_text(buffer: &mut PixelBuffer, text: &str,
    left: i32, top: i32, width: i32, height: i32, 
    font: &fontdue::Font, font_size: f32, color: Color,
    horizontal_align: TextAlign, vertical_align: TextAlign) {

    let buffer_stride = buffer.width;
    let cursor_bottom = top + height;
    let max_bottom = std::cmp::min(top + height, buffer.height);
    let max_right = std::cmp::min(left + width, buffer.width);
    let max_top = std::cmp::max(top, 0);
    let max_left = std::cmp::max(left, 0);
    let p_metrics = font.metrics(('p'), font_size);
    let base_line_offset = p_metrics.ymin;
    let h_align_offset = match horizontal_align {
        TextAlign::Left => left,
        TextAlign::Right => left, // I don't need this one yet so I'll wait on the implementation
        TextAlign::Center => {
            let mut string_width = 0;
            for c in text.chars() {
                string_width += font.metrics(c, font_size).advance_width as i32;
            }
            left + (width / 2) - (string_width / 2)
        }
    };
    let mut cursor_left = h_align_offset;
    for c in text.chars() {
        let (font_metrics, font_bitmap) = font.rasterize(c, font_size);
        let buffer_top = cursor_bottom - (font_metrics.ymin + font_metrics.height as i32) + base_line_offset;
        let buffer_bottom = buffer_top + font_metrics.height as i32;
        let buffer_left = cursor_left;
        let buffer_right = buffer_left + font_metrics.width as i32;
        let mut font_index = 0;
        for buffer_y in buffer_top..buffer_bottom {
            for buffer_x in buffer_left..buffer_right {
                if  is_point_in_rect_a(buffer_x, buffer_y, max_left, max_top, max_right, max_bottom) {
                    let buffer_index = (buffer_y * buffer_stride) + buffer_x;
                    let buffer_pixel = &mut buffer.pixels[buffer_index as usize];
                    let font_pixel = font_bitmap[font_index];
                    if font_pixel > 0 {
                        buffer_pixel.red = alpha_blend_u8(color.red, buffer_pixel.red, font_pixel);
                        buffer_pixel.green = alpha_blend_u8(color.green, buffer_pixel.green, font_pixel);
                        buffer_pixel.blue = alpha_blend_u8(color.blue, buffer_pixel.blue, font_pixel);
                    }
                }
                font_index += 1;
            }
        }
        cursor_left += font_metrics.advance_width as i32;
    }
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


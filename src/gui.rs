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

    pub fn insert_char_at_cursor(&mut self, c: char) {

    }

    pub fn set_cursor_index(&mut self, i: i32) {

    }

    pub fn increment_cursor_index(&mut self) {
        self.cursor_index += 1;
        let charcount = self.text.as_ref().unwrap().chars().count() as i32;
        if self.cursor_index >  charcount {
            self.cursor_index = charcount;
        }
    }

    pub fn decrement_cursor_index(&mut self) {
        self.cursor_index -= 1;
        if self.cursor_index < 0 {
            self.cursor_index = 0;
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

pub enum KeyboardInputType {
    Char,
    Escape,
    Back,
    ArrowLeft,
    ArrowUp,
    ArrowRight,
    ArrowDown
}

pub fn handle_keyboard_keydown(keytype: KeyboardInputType, c: char) {
    match keytype {
        KeyboardInputType::Char => handle_key_char(c),
        KeyboardInputType::Escape => { },
        KeyboardInputType::Back => { },
        KeyboardInputType::ArrowLeft => handle_keyboard_arrow_left(),
        KeyboardInputType::ArrowUp => { },
        KeyboardInputType::ArrowRight => handle_keyboard_arrow_right(),
        KeyboardInputType::ArrowDown => { }
    }
}

fn handle_keyboard_arrow_left() {
    let textboxes = unsafe { &mut crate::APPLICATION_STATE.textboxes };
    for textbox in textboxes {
        if textbox.active {
            textbox.decrement_cursor_index();
            break;
        }
    }
}

fn handle_keyboard_arrow_right() {
    let textboxes = unsafe { &mut crate::APPLICATION_STATE.textboxes };
    for textbox in textboxes {
        if textbox.active {
            textbox.increment_cursor_index();
            break;
        }
    }
}

fn handle_key_char(c: char) {
    let textboxes = unsafe { &mut crate::APPLICATION_STATE.textboxes };
    for textbox in textboxes {
        if textbox.active {
            let mut text = textbox.text.as_ref().unwrap().clone();
            text.push(c);
            textbox.text = Some(text);
            break;
        }
    }
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
    let style = textbox.get_style();
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
        style.vertical_align,
        textbox.cursor_index,
        textbox.active);
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
        style.vertical_align,
        0, false);
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
    horizontal_align: HorizontalAlign, vertical_align: VerticalAlign,
    cursor_index: i32, draw_cursor: bool) {

    let buffer_stride = buffer.width;
    let max_bottom = std::cmp::min(top + height, buffer.height);
    let max_right = std::cmp::min(left + width, buffer.width);
    let max_top = std::cmp::max(top, 0);
    let max_left = std::cmp::max(left, 0);
    let (font_height, _, _) = measure_string("W", font, font_size);
    let (_, text_width, _) = measure_string(text, font, font_size);
    let h_align_offset = calculate_h_align_offset(width, text_width, horizontal_align);
    let v_align_offset = calculate_v_align_offset(height, font_height, vertical_align);
    let mut cursor_left = left + h_align_offset;
    let cursor_top = top + v_align_offset;
    let mut text_char_index = 0;
    let mut cursor_pos = cursor_left;
    for c in text.chars() {
        let (font_metrics, font_bitmap) = font.rasterize(c, font_size);
        let buffer_top = cursor_top + font_height - font_metrics.height as i32 - font_metrics.ymin;
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
        if cursor_index > text_char_index as i32 {
            cursor_pos = cursor_left;
        }
        text_char_index += 1;
    }

    if draw_cursor {
        fill_rect(buffer, 
            cursor_pos,
            cursor_top - 2,
            2, // width
            font_height + 4, // height
            color);
    }
}

fn calculate_h_align_offset(container_width: i32, text_width: i32, align: HorizontalAlign) -> i32 {
    match align {
        HorizontalAlign::Left => 0,
        //HorizontalAlign::Right => left, // I don't need this one yet so I'll wait on the implementation
        HorizontalAlign::Center => {
            (container_width / 2) - (text_width / 2)
        }
    }
}

fn calculate_v_align_offset(container_height: i32, text_height: i32, align: VerticalAlign) -> i32 {
    match align {
        VerticalAlign::Center => {
            container_height / 2 - text_height / 2
        },
        VerticalAlign::Bottom =>  { 
            container_height - text_height
        },
        VerticalAlign::Top => {
            0
        }
    }
}

/*
    return a tuple of height, width, baseline
*/
fn measure_string(text: &str, font: &fontdue::Font, font_size: f32) -> (i32, i32, i32) {
    let mut height: i32 = 0;
    let mut width: i32 = 0;
    let mut ymin: i32 = 0;
    for c in text.chars() {
        let m = font.metrics(c, font_size);
        if height < m.height as i32 {
            height = m.height as i32;
        }
        width += m.advance_width as i32;
        if ymin > m.ymin {
            ymin = m.ymin;
        }
    }
    (height, width, ymin)
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


#![windows_subsystem = "windows"]
#![allow(unused_parens)]

extern crate winapi;


use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::mem;
use std::ptr::null_mut;
use std::io::Error;
use std::str::FromStr;

use self::winapi::ctypes::c_void;

use self::winapi::shared::windef:: {
    HWND,
    // HDC,
    // HBITMAP,
    RECT,
    HCURSOR
};

use self::winapi::shared::ntdef:: {
    LONG
};

use self::winapi::um::wingdi::{
    // functions
    // PatBlt,
    // BitBlt,
    StretchDIBits,
    // SetDIBitsToDevice,
    // CreateDIBSection,
    // DeleteObject,
    // CreateCompatibleDC,
    // DeleteDC,

    // structs
    // BITMAP,
    BITMAPINFO,
    BITMAPINFOHEADER,
    RGBQUAD,

    // constants
    // WHITENESS,
    // BLACKNESS,
    SRCCOPY,
    DIB_RGB_COLORS,
    BI_RGB,
};

use self::winapi::shared::minwindef::{
    UINT,
    // DWORD,
    WPARAM,
    LPARAM,
    LRESULT,
};

//use self::winapi::shared::ntdef::LPCWSTR;

use self::winapi::um::libloaderapi::{
    GetModuleHandleW,
};

use self::winapi::shared::windowsx::{
    GET_X_LPARAM,
    GET_Y_LPARAM
};

use self::winapi::um::winuser::{

    // WNDCLASS
    WNDCLASSW,
    CS_OWNDC,
    CS_HREDRAW,
    CS_VREDRAW,
    CW_USEDEFAULT,
    RegisterClassW,
    SetClassLongW,

    // CreateWindow
    WS_OVERLAPPEDWINDOW,
    WS_VISIBLE,
    CreateWindowExW,

    // Message Loop
    TranslateMessage,
    DispatchMessageW,
    GetMessageW,
    DefWindowProcW,
    PostQuitMessage,
    MSG,

    // Message Constants
    WM_CREATE,
    WM_DESTROY,
    WM_PAINT,
    WM_SIZE,
    WM_SETCURSOR,
    WM_MOUSEMOVE,
    WM_LBUTTONDOWN,
    WM_LBUTTONUP,

    // Message Box
    // MB_OK, 
    // MessageBoxW,

    // cursors
    LoadCursorW,
    SetCursor,
    GCL_HCURSOR,
    IDC_ARROW,
    // IDC_WAIT,
    IDC_HAND,
    IDC_IBEAM,

    // icons
    LoadIconW,
    IDI_APPLICATION,

    // Painting
    BeginPaint,
    EndPaint,
    PAINTSTRUCT,
    GetClientRect,
    InvalidateRect
};

#[repr(C, align(4))]
#[derive(Debug, Copy, Clone)]
struct Pixel {
    blue: u8,
    green: u8,
    red: u8,
    alpha: u8
}

impl Pixel {
    fn default() -> Pixel {
        Pixel {
            blue: 0,
            green: 0,
            red: 0,
            alpha: 0
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Color {
    red : u8,
    green: u8,
    blue: u8
}

impl Color {
    const fn from_rgb(r:u8,g:u8,b:u8) -> Color {
        Color {
            red: r,
            green: g,
            blue: b
        }
    }

    const LIGHT_GRAY: Color = Color::from_rgb(200, 200, 200);
    const DARK_GRAY: Color = Color::from_rgb(50, 50, 50);

    const RED: Color = Color::from_rgb(255, 0, 0);
    const LIGHT_RED: Color = Color::from_rgb(255,200,200);
    const DARKER_RED: Color = Color::from_rgb(255,100,100);
    const DARK_RED: Color = Color::from_rgb(200, 0, 0);
}

enum TextAlign {
    Left,
    Right,
    Center
}

#[derive(Debug, Copy, Clone)]
struct Rect {
    x:i32,
    y:i32,
    w:i32,
    h:i32
}

struct BoxStyle {
    border_color: Color,
    border_width: i32,
    background_color: Color,
    text_color: Color,
    font_size: f32
}

impl BoxStyle {
    const fn default() -> BoxStyle {
        BoxStyle {
            border_color: Color::RED,
            border_width: 2,
            background_color: Color::LIGHT_RED,
            text_color: Color::RED,
            font_size: 20.0
        }
    }
}

struct PixelBuffer {
    pixels: Vec<Pixel>,
    width: i32,
    height: i32
}

struct Window {
    handle: HWND
}

struct Win32PixelBuffer {
    info: BITMAPINFO,
    data: PixelBuffer
}

struct ApplicationState {
    buttons: Vec::<Button>,
    textboxes: Vec::<TextBox>,
    textbox_style: BoxStyle,
    button_style: BoxStyle,
    button_style_hot: BoxStyle,
    button_style_active: BoxStyle,
    //text: Option<std::string::String>
}

trait Control {
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
}

struct Button {
    text: &'static str,
    bounds: Rect,
    hot: bool,
    active: bool,
    on_click: Option<ButtonClick>,
    click_count: i32
}

impl Control for Button {
    fn get_bounds(&self) -> Rect { self.bounds }
    fn get_hot(&self) -> bool { self.hot }
    fn set_hot(&mut self, hit: bool) { self.hot = hit }
}

struct TextBox {
    text: Option<std::string::String>,
    placeholder: &'static str,
    bounds: Rect,
    hot: bool,
    active: bool
}

impl Control for TextBox {
    fn get_bounds(&self) -> Rect { self.bounds }
    fn get_hot(&self) -> bool { self.hot }
    fn set_hot(&mut self, hit: bool) { self.hot = hit }
}

enum Cursor {
    NotSet,
    Arrow,
    IBeam,
    Hand
}

type ButtonClick = fn(&mut Button) -> ();

static mut CURSOR_ARROW: HCURSOR = null_mut();
static mut CURSOR_HAND: HCURSOR = null_mut();
static mut CURSOR_IBEAM: HCURSOR = null_mut();
static mut CURSOR: Cursor = Cursor::NotSet;
static mut GLOBAL_BACK_BUFFER : Win32PixelBuffer = create_win32_compatible_pixel_buffer();
static mut FONTS : Vec::<fontdue::Font> = Vec::new();
static mut APPLICATION_STATE : ApplicationState = ApplicationState {
    buttons: vec![],
    textboxes: vec![],
    textbox_style: BoxStyle::default(),
    button_style: BoxStyle::default(),
    button_style_hot: BoxStyle::default(),
    button_style_active: BoxStyle::default(),
    //text: None
};

fn button_on_click(button: &mut Button) {
    unsafe {
        button.click_count += 1;
        APPLICATION_STATE.textboxes[0].text = Some(format!("{} was clicked {} times", button.text, button.click_count));
    }
}

fn main() {
    let font = include_bytes!("../fonts/OpenSans-Regular.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
    unsafe {
        CURSOR_ARROW = LoadCursorW(null_mut(), IDC_ARROW);
        CURSOR_HAND = LoadCursorW(null_mut(), IDC_HAND);
        CURSOR_IBEAM = LoadCursorW(null_mut(), IDC_IBEAM);
        FONTS.push(font);
        APPLICATION_STATE.buttons.push(Button {
            text: "Click Me!",
            bounds: Rect { x: 300, y: 300, w: 150, h: 40 },
            hot: false, active: false, click_count: 0,
            on_click: Some(button_on_click)
        });
        APPLICATION_STATE.buttons.push(Button {
            text: "BUY NOW",
            bounds: Rect { x: 500, y: 300, w: 150, h: 40 },
            hot: false, active: false, click_count: 0,
            on_click: Some(button_on_click)
        });
        APPLICATION_STATE.textboxes.push(TextBox {
            text: Some(String::new()),
            placeholder: "Username",
            bounds: Rect { x: 10, y: 10, w: 500, h: 40 },
            hot: false, active: false
        });
        APPLICATION_STATE.button_style_hot.background_color = Color::DARKER_RED;
        APPLICATION_STATE.button_style_active.background_color = Color::DARK_RED;
        APPLICATION_STATE.textbox_style.font_size = 30.0;
        APPLICATION_STATE.button_style.font_size = 30.0;
        APPLICATION_STATE.button_style_active.font_size = 30.0;
        APPLICATION_STATE.button_style_hot.font_size = 30.0;
        //APPLICATION_STATE.text = Some(String::from_str("Hello, World").unwrap());
    }
    let mut window = create_window("FileX", "FileX").unwrap();

    loop {
        if !handle_message(&mut window) {
            break;
        }
    }
}

fn win32_string(value : &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

fn create_window(name: &str, title: &str) -> Result<Window, Error> {
    let name = win32_string(name);
    let title = win32_string(title);
    unsafe {
        let hinstance = GetModuleHandleW(null_mut());
        let wnd_class = WNDCLASSW {
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(win32_wnd_proc),
            hInstance: hinstance,
            lpszClassName: name.as_ptr(),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
            hCursor: null_mut(), //LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut()
        };
        
        RegisterClassW(&wnd_class);

        let handle = CreateWindowExW(
            0,
            name.as_ptr(),
            title.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            700,
            400,
            null_mut(),
            null_mut(),
            hinstance,
            null_mut()
        );

        if handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(Window { handle })
        }
    }
}

fn handle_message (window: &mut Window) -> bool {
    unsafe{
        let mut msg = mem::MaybeUninit::<MSG>::zeroed().assume_init();
        if GetMessageW(&mut msg, window.handle, 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
            true
        }
        else {
            false
        }
    }
}

unsafe extern "system" fn win32_wnd_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg {
        WM_SETCURSOR => {
            match &CURSOR {
                Cursor::Hand => { SetCursor(CURSOR_HAND); 1 },
                Cursor::Arrow => { SetCursor(CURSOR_ARROW); 1 },
                Cursor::IBeam => { SetCursor(CURSOR_IBEAM); 1 },
                Cursor::NotSet => { DefWindowProcW(h_wnd, msg, w_param, l_param) }
            }
        },
        WM_MOUSEMOVE => handle_wm_mouse_move(h_wnd, msg, w_param, l_param),
        WM_LBUTTONDOWN => handle_wm_button_click(h_wnd, msg, w_param, l_param,),
        WM_LBUTTONUP => handle_wm_button_click(h_wnd, msg, w_param, l_param),
        WM_CREATE => 0,
        WM_DESTROY => { PostQuitMessage(0); 0 },
        WM_PAINT => handle_wm_paint(h_wnd),
        WM_SIZE => handle_wm_size(h_wnd),
        _ => DefWindowProcW(h_wnd, msg, w_param, l_param)
    }
}

unsafe fn update_window(h_wnd: HWND) {
    update_back_buffer(&mut GLOBAL_BACK_BUFFER.data);
    InvalidateRect(h_wnd, null_mut(), 0);
}

unsafe fn handle_wm_mouse_move(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let mouse_x = GET_X_LPARAM(l_param);
    let mouse_y = GET_Y_LPARAM(l_param);
    let mut client_rect = mem::MaybeUninit::<RECT>::zeroed().assume_init();
    GetClientRect(h_wnd, &mut client_rect);
    let is_point_in_client_rect = is_point_in_rect_a(mouse_x, mouse_y, 
        client_rect.left + 2, client_rect.top + 2, client_rect.right - 2, client_rect.bottom - 2);
    let mut is_button_hot = false;
    let mut is_textbox_hot = false;
    let mut should_update_window = false;
    if is_point_in_client_rect {
        let buttons = &mut APPLICATION_STATE.buttons;
        for button in buttons {
            let (hot_changed, is_hot) = button.hit_check(mouse_x, mouse_y);
            if hot_changed { should_update_window = true }
            if is_hot { is_button_hot = true }
        }
        let textboxes = &mut APPLICATION_STATE.textboxes;
        for textbox in textboxes {
            let (hot_changed, is_hot) = textbox.hit_check(mouse_x, mouse_y);
            if hot_changed { should_update_window = true }
            if is_hot { is_textbox_hot = true }
        }
    }

    // might have to set cursor inside a WM_SETCURSOR message
    // because this doesn't appear to be working
    match (is_button_hot, is_textbox_hot, is_point_in_client_rect) {
        (true, false, true) => match &CURSOR {
            Cursor::Hand => { },
            _ => CURSOR = Cursor::Hand
        },
        (false, true, true) => match &CURSOR {
            Cursor::IBeam => { },
            _ => CURSOR = Cursor::IBeam
        },
        (false, false, true) => match &CURSOR {
            Cursor::Arrow => {  },
            _ => CURSOR = Cursor::Arrow
        },
        _ => match &CURSOR {
            _ => CURSOR = Cursor::NotSet
        }
    }
    if should_update_window { update_window(h_wnd); }
    return 0;
}

unsafe fn handle_wm_button_click(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let mouse_x = GET_X_LPARAM(l_param);
    let mouse_y = GET_Y_LPARAM(l_param);
    let buttons = &mut APPLICATION_STATE.buttons;
    for button in buttons {
        let hit = is_point_in_rect(mouse_x, mouse_y, button.bounds);
        match msg {
            WM_LBUTTONDOWN => handle_button_mouse_down(button, hit),
            WM_LBUTTONUP => handle_button_mouse_up(button, hit),
            _ => { }
        }
    }
    let textboxes = &mut APPLICATION_STATE.textboxes;
    for textbox in textboxes {
        let hit = is_point_in_rect(mouse_x, mouse_y, textbox.bounds);
        match msg {
            WM_LBUTTONDOWN => handle_textbox_mouse_down(textbox, hit),
            _ => { }
        }
    }
    update_window(h_wnd);
    return 0;
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

fn handle_wm_paint(h_wnd: HWND) -> LRESULT {
    unsafe {
        let mut ps =  mem::MaybeUninit::<PAINTSTRUCT>::zeroed().assume_init();
        let hdc = BeginPaint(h_wnd, &mut ps);
        let width = GLOBAL_BACK_BUFFER.data.width as i32;
        let height = GLOBAL_BACK_BUFFER.data.height as i32;

        // NOTE(wayne) I'm always drawing the entire screen instead of just the rect
        // provided by BeginPaint because StretchDIBits was inverting the image
        // and doing other strange things when moving the window off screen.
        // drawing the entire backbuffer into the dc seems to work best.
        StretchDIBits(
            hdc, 
            0, 0, width, height, // destination
            0, 0, width, height, // source
            GLOBAL_BACK_BUFFER.data.pixels.as_ptr() as *const c_void,
            &GLOBAL_BACK_BUFFER.info,
            DIB_RGB_COLORS,
            SRCCOPY);
        EndPaint(h_wnd, &ps);
        return 0;
    }
}

fn handle_wm_size(h_wnd: HWND) -> LRESULT {
    // NOTE(wayne) Every time we get the wm_size message we need to reallocate
    // and render into a new back buffer
    unsafe {
        let mut client_rect = mem::MaybeUninit::<RECT>::zeroed().assume_init();
        GetClientRect(h_wnd, &mut client_rect);
        let width = client_rect.right - client_rect.left;
        let height = client_rect.bottom - client_rect.top;
        resize_offscreen_buffer(&mut GLOBAL_BACK_BUFFER, width, height);
        update_back_buffer(&mut GLOBAL_BACK_BUFFER.data);
    }
    return 0;
}

const fn create_win32_compatible_pixel_buffer() -> Win32PixelBuffer {
    let bit_count = 32;
    Win32PixelBuffer {
        info: BITMAPINFO { 
            bmiHeader: BITMAPINFOHEADER { 
                biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
                biHeight: 0,
                biWidth: 0,
                biPlanes: 1,
                biBitCount: bit_count,
                biCompression: BI_RGB,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [RGBQUAD { rgbBlue: 0, rgbGreen: 0, rgbRed: 0, rgbReserved: 0 }]}
        ,data: PixelBuffer {
            pixels: Vec::new(),
            width: 0,
            height: 0
        }
    }
}

fn resize_offscreen_buffer(buffer: &mut Win32PixelBuffer, width: i32, height: i32) {
    buffer.info.bmiHeader.biWidth = width;
    buffer.info.bmiHeader.biHeight = -height; // negative means top down DIB

    let pixel_size = (width * height) as usize;
    buffer.data.pixels = vec![Pixel::default(); pixel_size];
    buffer.data.width = width;
    buffer.data.height = height;
}

fn update_back_buffer(mut buffer: &mut PixelBuffer) {
    let width = buffer.width;
    let height = buffer.height;
    fill_rect(&mut buffer, 0, 0, width, height, Color::LIGHT_GRAY);
    fill_rect(&mut buffer, 0, height / 2 - 2, width, 4, Color::DARK_GRAY);
    fill_rect(&mut buffer, width / 2 - 2, 0, 4, height, Color::DARK_GRAY);
    // fill_rect(&mut buffer, 0, 0, 50, 50, Color::DARK_GRAY);
    fill_rect(&mut buffer, 0, height / 4 - 2, width, 4, Color::DARK_GRAY);
    fill_rect(&mut buffer, 0, height / 4 * 3 - 2, width, 4, Color::DARK_GRAY);

    let font = unsafe { &FONTS[0] };

    let textboxes = unsafe { &APPLICATION_STATE.textboxes };
    let textbox_style = unsafe { &APPLICATION_STATE.textbox_style };

    let buttons = unsafe { &APPLICATION_STATE.buttons };
    let button_style = unsafe { &APPLICATION_STATE.button_style };
    let button_style_hot = unsafe { &APPLICATION_STATE.button_style_hot };
    let button_style_active = unsafe { &APPLICATION_STATE.button_style_active };

    for textbox in textboxes {
        let left = textbox.bounds.x;
        let top = textbox.bounds.y;
        let width = textbox.bounds.w;
        let height = textbox.bounds.h;
        let style = textbox_style;
        fill_rect(&mut buffer, left + style.border_width, top + style.border_width, width - style.border_width * 2, height - style.border_width * 2, style.background_color);
        draw_rect(&mut buffer, left, top, width, height, style.border_width, style.border_color);
        fill_text(&mut buffer, textbox.text.as_ref().unwrap(), left + style.border_width, top + style.border_width, width - style.border_width * 2, height - style.border_width * 2, &font, style.font_size, style.text_color, TextAlign::Left);
    }

    for button in buttons {
        let left = button.bounds.x;
        let top = button.bounds.y;
        let width = button.bounds.w;
        let height = button.bounds.h;
        let style = choose(button.hot, button_style_hot, button_style);
        let style = choose(button.active, button_style_active, style);
        fill_rect(&mut buffer, left + style.border_width, top + style.border_width, width - style.border_width * 2, height - style.border_width * 2, style.background_color);
        draw_rect(&mut buffer, left, top, width, height, style.border_width, style.border_color);
        fill_text(&mut buffer, button.text, left + style.border_width, top + style.border_width, width - style.border_width * 2, height - style.border_width * 2, &font, style.font_size, style.text_color, TextAlign::Center);
    }
}

// fn draw_textbox(buffer: &mut PixelBuffer) {

// }

// fn draw_button(buffer: &mut PixelBuffer) {

// }

fn fill_rect(buffer: &mut PixelBuffer, left: i32, top: i32, width: i32, height: i32, color: Color) {
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

fn draw_rect(buffer: &mut PixelBuffer, left: i32, top: i32, width: i32, height: i32, line_width: i32, color: Color) {
    fill_rect(buffer, left, top, width, line_width, color); // top
    fill_rect(buffer, left + width - line_width, top, line_width, height, color); // right
    fill_rect(buffer, left, top + height - line_width, width, line_width, color); // bottom
    fill_rect(buffer, left, top, line_width, height, color); // left
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
    horizontal_align: TextAlign) {

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

fn is_point_in_rect_a(x:i32, y:i32, left:i32, top:i32, right:i32, bottom:i32) -> bool {
    x > left && x < right && y > top && y < bottom
}

fn is_point_in_rect(x: i32, y: i32, bounds: Rect) -> bool {
    let right = bounds.x + bounds.w;
    let bottom = bounds.y + bounds.h;
    is_point_in_rect_a(x,y,bounds.x,bounds.y,right,bottom)
}

fn choose<T>(cmp: bool, option1: T, option2: T) -> T {
    match cmp {
        true => option1,
        false => option2
    }
}


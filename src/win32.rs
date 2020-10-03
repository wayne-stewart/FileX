#![windows_subsystem = "windows"]
#![allow(unused_parens)]
#![allow(unused_imports)]

extern crate winapi;

use crate::gui:: {
    PixelBuffer,
    Cursor,
};
use crate::gui::is_point_in_rect_a;
use crate::gui::keyboard::KeyboardInputType;
use crate::gui::keyboard::KeyboardInputModifiers;
use crate::gui::mouse::handle_mouse_button_down;
use crate::gui::mouse::handle_mouse_button_up;
use crate::gui::keyboard::handle_keyboard_keydown;
use crate::update_back_buffer;
use crate::gui::mouse::handle_mouse_move;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::mem;
use std::ptr::null_mut;
use std::io::Error;

use self::winapi::ctypes::c_void;

use self::winapi::shared::windef:: {
    HWND,
    HDC,
    HBITMAP,
    RECT,
    HCURSOR
};

use self::winapi::shared::ntdef:: {
    LONG,
    NULL
};

use self::winapi::um::wingdi::{
    // functions
    PatBlt,
    BitBlt,
    StretchDIBits,
    SetDIBitsToDevice,
    CreateDIBSection,
    DeleteObject,
    CreateCompatibleDC,
    DeleteDC,

    // structs
    BITMAP,
    BITMAPINFO,
    BITMAPINFOHEADER,
    RGBQUAD,

    // constants
    WHITENESS,
    BLACKNESS,
    SRCCOPY,
    DIB_RGB_COLORS,
    BI_RGB,
};

use self::winapi::shared::minwindef::{
    UINT,
    DWORD,
    WPARAM,
    LPARAM,
    LRESULT,
    HGLOBAL,
    LPVOID,
    TRUE,
    FALSE
};

//use self::winapi::shared::ntdef::LPCWSTR;

use self::winapi::um::libloaderapi::{
    GetModuleHandleW,
};

use self::winapi::shared::windowsx::{
    GET_X_LPARAM,
    GET_Y_LPARAM
};

use self::winapi::um::winbase::{
    GlobalLock,
    GlobalUnlock,
    GlobalAlloc,

    GMEM_MOVEABLE,
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
    GetAsyncKeyState,
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
    WM_KEYDOWN,
    WM_KEYUP,
    WM_CHAR,

    // virtual key codes
    VK_ESCAPE,
    VK_BACK,
    VK_DELETE,
    VK_RIGHT,
    VK_LEFT,
    VK_UP,
    VK_DOWN,
    VK_SHIFT,
    VK_CONTROL,
    VK_MENU, // ALT
    VK_CAPITAL,

    // clipboard
    IsClipboardFormatAvailable,
    OpenClipboard,
    CloseClipboard,
    GetClipboardData,
    SetClipboardData,
    EmptyClipboard,
    CF_UNICODETEXT,

    // Message Box
    MB_OK, 
    MessageBoxW,

    // cursors
    LoadCursorW,
    SetCursor,
    GCL_HCURSOR,
    IDC_ARROW,
    IDC_WAIT,
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

struct Window {
    handle: HWND
}

struct Win32PixelBuffer {
    info: BITMAPINFO,
    data: PixelBuffer
}

static mut CURSOR_ARROW: HCURSOR = null_mut();
static mut CURSOR_HAND: HCURSOR = null_mut();
static mut CURSOR_IBEAM: HCURSOR = null_mut();
static mut GLOBAL_BACK_BUFFER : Win32PixelBuffer = create_win32_compatible_pixel_buffer();

pub fn platform_init() {
    unsafe {
        CURSOR_ARROW = LoadCursorW(null_mut(), IDC_ARROW);
        CURSOR_HAND = LoadCursorW(null_mut(), IDC_HAND);
        CURSOR_IBEAM = LoadCursorW(null_mut(), IDC_IBEAM);
    }
}

pub fn platform_run() {
    let mut window = create_window("FileX", "FileX").unwrap();

    run_message_loop(&mut window);
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

unsafe extern "system" fn win32_wnd_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg {
        // the wm_setcursor message is called automatically by windows on mouse move so however
        // the application updates the internal application state, I can set the appropriate
        // cursor here.
        WM_SETCURSOR => {
            match &crate::APPLICATION_STATE.cursor {
                Cursor::Hand => { SetCursor(CURSOR_HAND); 1 },
                Cursor::Arrow => { SetCursor(CURSOR_ARROW); 1 },
                Cursor::IBeam => { SetCursor(CURSOR_IBEAM); 1 },
                Cursor::NotSet => DefWindowProcW(h_wnd, msg, w_param, l_param)
            }
        },
        WM_MOUSEMOVE => handle_wm_mouse_move(h_wnd, msg, w_param, l_param),
        WM_LBUTTONDOWN => handle_wm_button_click(h_wnd, msg, w_param, l_param,),
        WM_LBUTTONUP => handle_wm_button_click(h_wnd, msg, w_param, l_param),
        WM_KEYDOWN => handle_wm_keydown(h_wnd, msg, w_param, l_param),
        WM_KEYUP => 0,
        WM_CHAR => handle_wm_char(h_wnd, msg, w_param, l_param),
        WM_CREATE => 0,
        WM_DESTROY => { PostQuitMessage(0); 0 },
        WM_PAINT => handle_wm_paint(h_wnd),
        WM_SIZE => handle_wm_size(h_wnd),
        _ => DefWindowProcW(h_wnd, msg, w_param, l_param)
    }
}

unsafe fn update_window(h_wnd: HWND) {
    crate::update_back_buffer(&mut GLOBAL_BACK_BUFFER.data);
    InvalidateRect(h_wnd, null_mut(), 0);
}


fn run_message_loop (window: &mut Window) {
    unsafe {
        loop {
            let mut msg = mem::MaybeUninit::<MSG>::zeroed().assume_init();
            if GetMessageW(&mut msg, window.handle, 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
                continue
            }
            else {
                break
            }
        }
    }
}

unsafe fn handle_wm_char(h_wnd: HWND, _msg: UINT, w_param: WPARAM, _l_param: LPARAM) -> LRESULT {
    let ctrl_down = (0 != GetAsyncKeyState(VK_CONTROL));
    if (!ctrl_down) {
        let c = std::char::decode_utf16([w_param as u16].iter().cloned()).nth(0).unwrap().unwrap();
        if !c.is_control() {
            handle_keyboard_keydown(KeyboardInputType::Char(c));
            update_window(h_wnd);
        }
    }
    0
}

unsafe fn handle_wm_keydown(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match w_param as i32 {
        0x41 => { // A
            if 0 != GetAsyncKeyState(VK_CONTROL) {
                handle_keyboard_keydown(KeyboardInputType::Ctrl_A);
                update_window(h_wnd);
            }
        }, 
        0x43 => { // C
            if 0 != GetAsyncKeyState(VK_CONTROL) {
                handle_keyboard_keydown(KeyboardInputType::Ctrl_C);
            }
        },
        0x56 => { // V
            if 0 != GetAsyncKeyState(VK_CONTROL) {
                let cp_text = get_text_from_clipboard(h_wnd);
                handle_keyboard_keydown(KeyboardInputType::Ctrl_V(cp_text));
                update_window(h_wnd);
            }
        },
        0x58 => { // X
            if 0 != GetAsyncKeyState(VK_CONTROL) {
                handle_keyboard_keydown(KeyboardInputType::Ctrl_X);
                update_window(h_wnd);
            }
        },
        0x59 => { // Y
            if 0 != GetAsyncKeyState(VK_CONTROL) {
                handle_keyboard_keydown(KeyboardInputType::Ctrl_Y);
                update_window(h_wnd);
            }
        },
        0x5A => { // Z
            if 0 != GetAsyncKeyState(VK_CONTROL) {
                handle_keyboard_keydown(KeyboardInputType::Ctrl_Z);
                update_window(h_wnd);
            }
        },
        VK_ESCAPE => handle_keyboard_keydown(KeyboardInputType::Escape),
        
        VK_BACK => { handle_keyboard_keydown(KeyboardInputType::Back); update_window(h_wnd); },
        VK_DELETE => { handle_keyboard_keydown(KeyboardInputType::Delete); update_window(h_wnd); },
        VK_SHIFT => handle_keyboard_keydown(KeyboardInputType::Shift),
        VK_CONTROL => handle_keyboard_keydown(KeyboardInputType::Ctrl),
        VK_MENU => handle_keyboard_keydown(KeyboardInputType::Alt),
        VK_CAPITAL => handle_keyboard_keydown(KeyboardInputType::CapsLock),
        
        VK_LEFT => { 
            let modifiers = get_keyboard_input_modifiers();
            handle_keyboard_keydown(KeyboardInputType::ArrowLeft(modifiers)); 
            update_window(h_wnd); },
        VK_UP => {
            let modifiers = get_keyboard_input_modifiers();
            handle_keyboard_keydown(KeyboardInputType::ArrowUp(modifiers)); },
        VK_RIGHT => { 
            let modifiers = get_keyboard_input_modifiers();
            handle_keyboard_keydown(KeyboardInputType::ArrowRight(modifiers)); 
            update_window(h_wnd); },
        VK_DOWN => {
            let modifiers = get_keyboard_input_modifiers();
            handle_keyboard_keydown(KeyboardInputType::ArrowDown(modifiers)); },
        _ => { DefWindowProcW(h_wnd, msg, w_param, l_param); }
    }
    0
}

unsafe fn handle_wm_mouse_move(h_wnd: HWND, _msg: UINT, _w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let mouse_x = GET_X_LPARAM(l_param);
    let mouse_y = GET_Y_LPARAM(l_param);

    let mut client_rect = mem::MaybeUninit::<RECT>::zeroed().assume_init();
    GetClientRect(h_wnd, &mut client_rect);

    // checking 2 pixels in here to make sure I don't overwrite the cursor
    // handling for the window edges. windows will display the proper resize
    // cursors by itself so I will set our cursor to NotSet. WM_SETCURSOR
    // knows how to handle it to use the default wnd proc
    let is_point_in_client_rect = is_point_in_rect_a(mouse_x, mouse_y, 
        client_rect.left + 2, 
        client_rect.top + 2, 
        client_rect.right - 2, 
        client_rect.bottom - 2);

    let (cursor, should_update_window) = handle_mouse_move(mouse_x, mouse_y);

    if is_point_in_client_rect {
        crate::APPLICATION_STATE.cursor = cursor;
    }
    else {
        crate::APPLICATION_STATE.cursor = Cursor::NotSet;
    }

    if should_update_window {
        update_window(h_wnd);
    }

    return 0;
}

unsafe fn handle_wm_button_click(h_wnd: HWND, msg: UINT, _w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let mouse_x = GET_X_LPARAM(l_param);
    let mouse_y = GET_Y_LPARAM(l_param);
    match msg {
        WM_LBUTTONDOWN => handle_mouse_button_down(mouse_x, mouse_y),
        WM_LBUTTONUP => handle_mouse_button_up(mouse_x, mouse_y),
        _ => { }
    }
    update_window(h_wnd);
    return 0;
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
    // NOTE(wayne) Every time I get the wm_size message I need to reallocate
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
    buffer.data.pixels = vec![crate::gui::Pixel::default(); pixel_size];
    buffer.data.width = width;
    buffer.data.height = height;
}

unsafe fn get_text_from_clipboard(h_wnd: HWND) -> Option<String> {
    let mut result: Option<String> = None;
    if TRUE == IsClipboardFormatAvailable(CF_UNICODETEXT) {
        if TRUE == OpenClipboard(h_wnd) {
            let hglb = GetClipboardData(CF_UNICODETEXT);
            if NULL != hglb {
                let lptstr = GlobalLock(hglb);
                result = convert_from_lpvoid_null_term_to_string(lptstr, 4096);
                GlobalUnlock(hglb);
            }
            CloseClipboard();
        }
    }
    return result;
}

pub fn set_text_into_clipboard(text: &str) { //(h_wnd: HWND, s: &str) {
    unsafe {
        //if TRUE == OpenClipboard(h_wnd) {
        if TRUE == OpenClipboard(NULL as HWND) {
            EmptyClipboard();
            let mut utf16: Vec::<u16> = text.encode_utf16().collect();
            utf16.push(0);
            let ptr_to_utf16 = utf16.as_ptr() as *const u8;
            let hglb = GlobalAlloc(GMEM_MOVEABLE, utf16.len() * 2);
            if NULL != hglb {
                let lptstr = GlobalLock(hglb) as *mut u8;
                for data_offset in 0..(utf16.len() * 2) as isize {
                    let src = ptr_to_utf16.offset(data_offset);
                    let dst = lptstr.offset(data_offset);
                    std::ptr::write(dst, *src);
                }
                GlobalUnlock(hglb);
                SetClipboardData(CF_UNICODETEXT, hglb);
            }
            CloseClipboard();
        }
    }
}

unsafe fn convert_from_lpvoid_null_term_to_string(ptr: LPVOID, max_length: usize) -> Option<String> {
    let mut result: Option<String> = None;
    if NULL != ptr {
        let mut data = ptr as *const u16;
        let mut text = String::new();
        let mut count = 0;
        while *data != 0 && count < max_length {
            text.push(std::char::from_u32(*data as u32).unwrap());
            data = data.offset(1);
            count += 1;
        }
        result = Some(text);
    }
    return result;
}

unsafe fn get_keyboard_input_modifiers() -> KeyboardInputModifiers {
    KeyboardInputModifiers {
        ctrl: (0 != GetAsyncKeyState(VK_CONTROL)),
        alt: (0 != GetAsyncKeyState(VK_MENU)),
        shift: (0 != GetAsyncKeyState(VK_SHIFT))
    }
}
#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::sync::Arc;
use std::sync::atomic::{AtomicBool,Ordering};
use std::thread;

mod win32;
mod gui;
use self::gui::Cursor;
use crate::gui::color::Color;
use crate::gui::textbox::TextBox;
use crate::gui::button::Button;
use crate::gui::draw::fill_rect;
use crate::gui::draw::draw_button;
use crate::gui::draw::draw_textbox;
use crate::gui::Rect;
use crate::gui::Bounds;
use crate::gui::style::BoxStyle;
use crate::gui::style::BoxSize;
use crate::gui::style::HorizontalAlign;
use crate::gui::style::VerticalAlign;
use crate::gui::PixelBuffer;

use crate::win32::platform_run;
use crate::win32::set_text_into_clipboard;
use crate::win32::invalidate_window;
use crate::win32::send_cursor_timer_tick;

type SetClipBoardTextData = fn(&str) -> ();
type THEME = crate::gui::color::DarkTheme;

const FILE_PATH_BOX_STYLE: BoxStyle = BoxStyle {
    border_color: Color::RED,
    border_size: BoxSize::single(0),
    padding_size: BoxSize { left:4, right:4, top:0, bottom:0 },
    background_color: THEME::BACKGROUND_LIGHT,
    text_color: THEME::TEXT,
    highlight_color: THEME::HIGHLIGHT,
    text_highlight_color: THEME::TEXT,
    font_size: 20.0,
    vertical_align: VerticalAlign::Center,
    horizontal_align: HorizontalAlign::Left
};

struct ApplicationState {
    set_clipboard_text_data: Option<SetClipBoardTextData>,
    needs_redraw: bool,
    cursor: Cursor,
    fonts: Vec::<fontdue::Font>,
    buttons: Vec::<Button>,
    textboxes: Vec::<TextBox>
}

static mut APPLICATION_STATE : ApplicationState = ApplicationState {
    set_clipboard_text_data: None,
    needs_redraw: true,
    cursor: gui::Cursor::NotSet,
    fonts: vec![],
    buttons: vec![],
    textboxes: vec![]
};

static mut GLOBAL_BACK_BUFFER: PixelBuffer = PixelBuffer {
    height: 0,
    width: 0,
    pixels: vec![]
};

static CURSOR_TOGGLE: AtomicBool = AtomicBool::new(true);

fn button_on_click(button: &mut Button) {
    unsafe {
        button.click_count += 1;
        APPLICATION_STATE.textboxes[0].set_text(&format!("{} was clicked {} times", button.text, button.click_count));
    }
}

fn main() {
    unsafe {
        APPLICATION_STATE.set_clipboard_text_data = Some(set_text_into_clipboard);
    }

    //init_test_view();
    init_primary_view();

    std::thread::spawn(move||{
        let mut b = false;
        loop {
            thread::sleep(std::time::Duration::from_millis(250));
            b = CURSOR_TOGGLE.swap(!b, Ordering::Relaxed);
            send_cursor_timer_tick();
        }
    });

    platform_run();
}

fn cursor_timer_tick() {
    let textboxes = unsafe { &APPLICATION_STATE.textboxes };
    for textbox in textboxes {
        if textbox.active {
            let toggle = CURSOR_TOGGLE.load(Ordering::Relaxed);
            unsafe { crate::draw_textbox(&mut GLOBAL_BACK_BUFFER, textbox, &APPLICATION_STATE.fonts[0], toggle) };
            update_window();
        }
    }
}

fn update_window() {
    unsafe {
        APPLICATION_STATE.needs_redraw = true;
        invalidate_window();
    }
}

fn handle_window_resize(width: i32, height: i32) {
    unsafe {
        let pixel_size = (width * height) as usize;
        GLOBAL_BACK_BUFFER.pixels = vec![crate::gui::Pixel::default(); pixel_size];
        GLOBAL_BACK_BUFFER.width = width;
        GLOBAL_BACK_BUFFER.height = height;
        for textbox in &mut APPLICATION_STATE.textboxes {
            textbox.update_bounds_rect(width, height);
        }
    }
}

// fn init_test_view() {
//     unsafe {
//         let font = include_bytes!("../fonts/OpenSans-Regular.ttf") as &[u8];
//         let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
//         APPLICATION_STATE.fonts.push(font);

//         APPLICATION_STATE.buttons.push(Button {
//             text: "Click Me!",
//             bounds: gui::Rect { x: 300, y: 300, w: 150, h: 50 },
//             hot: false, active: false, click_count: 0,
//             on_click: Some(button_on_click), 
//             style: BoxStyle::button_default(),
//             style_hot: BoxStyle::button_default_hot(),
//             style_active: BoxStyle::button_default_active()
//         });

//         APPLICATION_STATE.buttons.push(Button {
//             text: "BUY NOW",
//             bounds: Rect { x: 500, y: 300, w: 150, h: 50 },
//             hot: false, active: false, click_count: 0,
//             on_click: Some(button_on_click),
//             style: BoxStyle::button_default(),
//             style_hot: BoxStyle::button_default_hot(),
//             style_active: BoxStyle::button_default_active()
//         });

//         APPLICATION_STATE.textboxes.push(TextBox {
//             text: Vec::new(),
//             placeholder: "Username",
//             bounds: Rect { x: 10, y: 10, w: 500, h: 100 },
//             hot: false, active: false, 
//             cursor_index: 0, scroll_offset_x: 0,
//             selection_index: usize::MAX,
//             style: BoxStyle::textbox_default()
//         });
//         //APPLICATION_STATE.textboxes[0].style.horizontal_align = crate::gui::style::HorizontalAlign::Center;
//     }
// }

// fn update_back_buffer(mut buffer: &mut gui::PixelBuffer) {
//     let width = buffer.width;
//     let height = buffer.height;
//     fill_rect(&mut buffer, 0, 0, width, height, Color::LIGHT_GRAY);
//     fill_rect(&mut buffer, 0, height / 2 - 2, width, 4, Color::DARK_GRAY);
//     fill_rect(&mut buffer, width / 2 - 2, 0, 4, height, Color::DARK_GRAY);
//     //fill_rect(&mut buffer, 0, 0, 50, 50, Color::DARK_GRAY);
//     fill_rect(&mut buffer, 0, height / 4 - 2, width, 4, Color::DARK_GRAY);
//     fill_rect(&mut buffer, 0, height / 4 * 3 - 2, width, 4, Color::DARK_GRAY);

//     let font = unsafe { &APPLICATION_STATE.fonts[0] };
//     let textboxes = unsafe { &APPLICATION_STATE.textboxes };
//     let buttons = unsafe { &APPLICATION_STATE.buttons };

//     for textbox in textboxes {
//         draw_textbox(buffer, &textbox, &font);
//     }

//     for button in buttons {
//         draw_button(buffer, &button, &font);
//     }
// }

fn init_primary_view() {
    unsafe {
        let font = include_bytes!("../fonts/OpenSans-Regular.ttf") as &[u8];
        let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
        APPLICATION_STATE.fonts.push(font);

        APPLICATION_STATE.textboxes.push(TextBox {
            text: Vec::new(),
            placeholder: "",
            bounds: Bounds::variable_horizontal(0.2, 0, 0.8, 30),
            bounds_rect: Rect::default(),
            hot: false, active: false, 
            cursor_index: 0, scroll_offset_x: 0,
            selection_index: usize::MAX,
            style: FILE_PATH_BOX_STYLE
        });
    }
}

fn update_back_buffer() {
    let buffer = unsafe { &mut GLOBAL_BACK_BUFFER };
    let draw_cursor = CURSOR_TOGGLE.load(Ordering::Relaxed);
    let width = buffer.width;
    let height = buffer.height;
    fill_rect(buffer, 0, 0, width / 5, height, THEME::BACKGROUND_LIGHT);
    fill_rect(buffer, width /5, 0, width * 4 / 5, height, THEME::BACKGROUND);
    fill_rect(buffer, width * 3 / 5, 0, 4, height, THEME::BACKGROUND_LIGHT);
    // fill_rect(&mut buffer, 0, 0, width, height, Color::LIGHT_GRAY);
    // fill_rect(&mut buffer, 0, height / 2 - 2, width, 4, Color::DARK_GRAY);
    // fill_rect(&mut buffer, width / 2 - 2, 0, 4, height, Color::DARK_GRAY);
    // //fill_rect(&mut buffer, 0, 0, 50, 50, Color::DARK_GRAY);
    // fill_rect(&mut buffer, 0, height / 4 - 2, width, 4, Color::DARK_GRAY);
    // fill_rect(&mut buffer, 0, height / 4 * 3 - 2, width, 4, Color::DARK_GRAY);

    let font = unsafe { &APPLICATION_STATE.fonts[0] };
    let textboxes = unsafe { &APPLICATION_STATE.textboxes };
    // let buttons = unsafe { &APPLICATION_STATE.buttons };

    for textbox in textboxes {
        draw_textbox(buffer, &textbox, &font, textbox.active && draw_cursor);
    }

    // for button in buttons {
    //     draw_button(buffer, &button, &font);
    // }
}




mod win32;


mod gui;
use self::gui::*;

trait Platform {
    fn bitblt_back_buffer_to_screen();
    fn create_timer(milliseconds: u32);
}

struct ApplicationState {
    cursor: Cursor,
    fonts: Vec::<fontdue::Font>,
    buttons: Vec::<gui::Button>,
    textboxes: Vec::<gui::TextBox>
}

static mut APPLICATION_STATE : ApplicationState = ApplicationState {
    cursor: gui::Cursor::NotSet,
    fonts: vec![],
    buttons: vec![],
    textboxes: vec![]
};

fn button_on_click(button: &mut gui::Button) {
    unsafe {
        button.click_count += 1;
        APPLICATION_STATE.textboxes[0].set_text(&format!("{} was clicked {} times", button.text, button.click_count));
    }
}

fn main() {
    let font = include_bytes!("../fonts/OpenSans-Regular.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
    unsafe {
        APPLICATION_STATE.fonts.push(font);

        APPLICATION_STATE.buttons.push(Button {
            text: "Click Me!",
            bounds: gui::Rect { x: 300, y: 300, w: 150, h: 50 },
            hot: false, active: false, click_count: 0,
            on_click: Some(button_on_click), 
            style: BoxStyle::button_default(),
            style_hot: BoxStyle::button_default_hot(),
            style_active: BoxStyle::button_default_active()
        });

        APPLICATION_STATE.buttons.push(Button {
            text: "BUY NOW",
            bounds: Rect { x: 500, y: 300, w: 150, h: 50 },
            hot: false, active: false, click_count: 0,
            on_click: Some(button_on_click),
            style: BoxStyle::button_default(),
            style_hot: BoxStyle::button_default_hot(),
            style_active: BoxStyle::button_default_active()
        });

        APPLICATION_STATE.textboxes.push(TextBox {
            text: Vec::new(),
            placeholder: "Username",
            bounds: Rect { x: 10, y: 10, w: 500, h: 100 },
            hot: false, active: false, 
            cursor_index: 0, selection_start_index: usize::MAX,
            style: BoxStyle::textbox_default()
        });
    }

    win32::platform_init();
    win32::platform_run();
}

fn update_back_buffer(mut buffer: &mut gui::PixelBuffer) {
    let width = buffer.width;
    let height = buffer.height;
    gui::fill_rect(&mut buffer, 0, 0, width, height, Color::LIGHT_GRAY);
    gui::fill_rect(&mut buffer, 0, height / 2 - 2, width, 4, Color::DARK_GRAY);
    gui::fill_rect(&mut buffer, width / 2 - 2, 0, 4, height, Color::DARK_GRAY);
    // fill_rect(&mut buffer, 0, 0, 50, 50, Color::DARK_GRAY);
    gui::fill_rect(&mut buffer, 0, height / 4 - 2, width, 4, Color::DARK_GRAY);
    gui::fill_rect(&mut buffer, 0, height / 4 * 3 - 2, width, 4, Color::DARK_GRAY);

    let font = unsafe { &APPLICATION_STATE.fonts[0] };
    let textboxes = unsafe { &APPLICATION_STATE.textboxes };
    let buttons = unsafe { &APPLICATION_STATE.buttons };

    for textbox in textboxes {
        gui::draw_textbox(buffer, &textbox, &font);
    }

    for button in buttons {
        gui::draw_button(buffer, &button, &font);
    }
}





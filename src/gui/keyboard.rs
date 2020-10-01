

pub struct KeyboardInputModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool
}

pub enum KeyboardInputType {
    Char(char),
    Escape,
    Back,
    Delete,
    Ctrl,
    Ctrl_A,
    Ctrl_C,
    Ctrl_V(Option<String>),
    Ctrl_X,
    Alt,
    Shift,
    CapsLock,
    ArrowLeft(KeyboardInputModifiers),
    ArrowUp(KeyboardInputModifiers),
    ArrowRight(KeyboardInputModifiers),
    ArrowDown(KeyboardInputModifiers)
}

pub fn handle_keyboard_keydown(keytype: KeyboardInputType) {
    let textboxes = unsafe { &mut crate::APPLICATION_STATE.textboxes };
    for textbox in textboxes {
        if textbox.active {
            match keytype {
                KeyboardInputType::Char(c) => textbox.insert_char_at_cursor(c),
                KeyboardInputType::Escape => { },
                KeyboardInputType::Back => textbox.delete_char_left_of_cursor(),
                KeyboardInputType::Delete => textbox.delete_char_at_cursor(),
                KeyboardInputType::Ctrl_V(s) => textbox.set_text_option_string(s),
                KeyboardInputType::ArrowLeft(modifiers) => textbox.handle_arrow_left_keydown(modifiers),
                KeyboardInputType::ArrowUp(_modifiers) => { },
                KeyboardInputType::ArrowRight(modifiers) => textbox.handle_arrow_right_keydown(modifiers),
                KeyboardInputType::ArrowDown(_modifiers) => { },
                _ => { }
            }
            break;
        }
    }

}


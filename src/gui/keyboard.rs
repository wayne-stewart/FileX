
#[derive(Debug, Copy, Clone)]
pub struct KeyboardModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool
}

pub enum KeyboardInput {
    Char(char),
    Escape,
    Back,
    Delete,
    Ctrl,
    Ctrl_A,
    Ctrl_C,
    Ctrl_V(Option<String>),
    Ctrl_X,
    Ctrl_Y,
    Ctrl_Z,
    Alt,
    Shift,
    CapsLock,
    ArrowLeft(KeyboardModifiers),
    ArrowUp(KeyboardModifiers),
    ArrowRight(KeyboardModifiers),
    ArrowDown(KeyboardModifiers),
    Home(KeyboardModifiers),
    End(KeyboardModifiers)
}

pub fn keyboard_keydown(keytype: KeyboardInput) {
    let textboxes = unsafe { &mut crate::APPLICATION_STATE.textboxes };
    crate::update_window();
    for textbox in textboxes {
        if textbox.active {
            match keytype {
                KeyboardInput::Char(c) => textbox.insert_char(c),
                KeyboardInput::Escape => { },
                KeyboardInput::Back => textbox.delete_back(),
                KeyboardInput::Delete => textbox.delete(),
                KeyboardInput::Ctrl_A => textbox.select_all(),
                KeyboardInput::Ctrl_C => textbox.copy_to_clipboard(),
                KeyboardInput::Ctrl_V(text) => textbox.insert_text(text),
                KeyboardInput::Ctrl_X => textbox.cut_to_clipboard(),
                KeyboardInput::ArrowLeft(modifiers) => textbox.arrow_left(modifiers),
                KeyboardInput::ArrowUp(_modifiers) => { },
                KeyboardInput::ArrowRight(modifiers) => textbox.arrow_right(modifiers),
                KeyboardInput::ArrowDown(_modifiers) => { },
                KeyboardInput::Home(modifiers) => textbox.home(modifiers),
                KeyboardInput::End(modifiers) => textbox.end(modifiers),
                _ => { }
            }
            break;
        }
    }
}


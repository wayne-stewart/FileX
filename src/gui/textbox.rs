
use crate::gui::is_point_in_rect;
use crate::gui::style::BoxStyle;
use crate::gui::Rect;
use crate::gui::control::Control;
use crate::gui::keyboard::KeyboardInputModifiers;

pub struct TextBox {
    pub text: Vec::<char>,
    pub placeholder: &'static str,
    pub bounds: Rect,
    pub hot: bool,
    pub active: bool,
    pub cursor_index: usize, // index of char
    pub selection_start_index: usize,
    pub style: BoxStyle
}

impl TextBox {
    pub fn set_text(&mut self, text: &str) {
        self.text.clear();
        for c in text.chars() {
            self.text.push(c);
        }
    }

    pub fn handle_mouse_button_down(&mut self, mouse_x: i32, mouse_y: i32) {
        let hit = is_point_in_rect(mouse_x, mouse_y, self.get_bounds());
        self.hot = hit;
        self.active = hit;
    }

    pub fn set_text_option_string(&mut self, text: Option<String>) {
        self.text.clear();
        match text {
            Some(s) => self.set_text(&s),
            _ => { }
        }
    }

    pub fn insert_char_at_cursor(&mut self, c: char) {
        if self.cursor_index > self.text.len() {
            self.cursor_index = self.text.len();
        }
        self.text.insert(self.cursor_index, c);
        self.increment_cursor_index();
    }

    pub fn delete_char_at_cursor(&mut self) {
        if !self.text.is_empty() && 
            self.cursor_index < self.text.len() {
            self.text.remove(self.cursor_index);
        }
    }

    pub fn delete_char_left_of_cursor(&mut self) {
        self.decrement_cursor_index();
        self.delete_char_at_cursor();
    }

    pub fn set_cursor_index(&mut self, i: usize) {
        // don't need to check < 0 because cursor_index is a usize
        self.cursor_index = i;
        if self.cursor_index > self.text.len() {
            self.cursor_index = self.text.len();
        }
    }

    pub fn increment_cursor_index(&mut self) {
        self.cursor_index += 1;
        // cursor is allowed to exceed text length by one
        // indicating that the cursor is at the end of the string
        if self.cursor_index >  self.text.len() {
            self.cursor_index = self.text.len();
        }
    }

    pub fn decrement_cursor_index(&mut self) {
        if self.cursor_index > 0 {
            self.cursor_index -= 1;
        }
    }

    pub fn handle_arrow_right_keydown(&mut self, modifiers: KeyboardInputModifiers) {
        if modifiers.shift && self.selection_start_index == usize::MAX {
            self.selection_start_index = self.cursor_index;
        }
        else if modifiers.shift == false {
            self.selection_start_index = usize::MAX;
        }
        if modifiers.ctrl {
            self.ctrl_jump_cursor(1);
            self.increment_cursor_index();
        }
        else {
            self.increment_cursor_index();
        }
    }

    pub fn handle_arrow_left_keydown(&mut self, modifiers: KeyboardInputModifiers) {
        if modifiers.shift && self.selection_start_index == usize::MAX {
            self.selection_start_index = self.cursor_index;
        }
        else if modifiers.shift == false {
            self.selection_start_index = usize::MAX;
        }
        if modifiers.ctrl {
            self.ctrl_jump_cursor(-1);
        }
        else {
            self.decrement_cursor_index();
        }
    }

    fn ctrl_jump_cursor(&mut self, by: i32) {
        let mut peek_i = self.cursor_index as i32 + by;
        let mut i = self.cursor_index as i32;
        loop {
            peek_i = peek_i as i32 + by;
            i = i as i32 + by;
            if i <= 0 { i = 0; break; }
            if peek_i >= self.text.len() as i32 { i = self.text.len() as i32; break; }
            if  self.text[i as usize].is_alphanumeric() && !self.text[peek_i as usize].is_alphanumeric() {
                break;
            }
        }
        self.cursor_index = i as usize;
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



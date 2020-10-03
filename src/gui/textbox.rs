
use crate::gui::is_point_in_rect;
use crate::gui::style::BoxStyle;
use crate::gui::Rect;
use crate::gui::control::Control;
use crate::gui::keyboard::KeyboardInputModifiers;
use std::iter::FromIterator;
use std::str::FromStr;

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
        self.cursor_index = 0;
        self._insert_text(text);
    }

    pub fn insert_text(&mut self, text: Option<String>) {
        let text = match text {
            Some(t) => t,
            None => String::from_str("").unwrap()
        };
        if self.selection_start_index == usize::MAX {
            self._insert_text(&text);
        }
        else {
            let start = std::cmp::min(self.cursor_index, self.selection_start_index);
            let end = std::cmp::max(self.cursor_index, self.selection_start_index);
            for _ in start..end {
                self.text.remove(start);
            }
            self.cursor_index = start;
            self._insert_text(&text);
        }
    }

    fn _insert_text(&mut self, text: &str) {
        for c in text.chars() {
            self.text.insert(self.cursor_index, c);
            self.cursor_index += 1;
        }
        self.selection_start_index = usize::MAX;
    }

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_index > self.text.len() {
            self.cursor_index = self.text.len();
        }
        self.text.insert(self.cursor_index, c);
        self.increment_cursor_index();
    }

    pub fn delete(&mut self) {
        if !self.text.is_empty() && 
            self.cursor_index < self.text.len() {
            self.text.remove(self.cursor_index);
        }
    }

    pub fn delete_back(&mut self) {
        if self.selection_start_index == usize::MAX {
            if self.decrement_cursor_index() {
                self.delete();
            }
        }
        else {
            self.delete();
        }
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

    // returns true if cursor decremented, false if not
    pub fn decrement_cursor_index(&mut self) -> bool {
        if self.cursor_index > 0 {
            self.cursor_index -= 1;
            return true;
        }
        return false;
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

    pub fn handle_mouse_button_down(&mut self, mouse_x: i32, mouse_y: i32) {
        let hit = is_point_in_rect(mouse_x, mouse_y, self.get_bounds());
        self.hot = hit;
        self.active = hit;
    }

    pub fn copy_to_clipboard(&self) {
        let method_option = unsafe { crate::APPLICATION_STATE.set_clipboard_text_data };
        match method_option {
            None => { },
            Some(method) => { 
                let mut copied_text: String;
                if self.selection_start_index == usize::MAX {
                    copied_text = String::from_iter(&self.text);
                }
                else {
                    let start = std::cmp::min(self.cursor_index, self.selection_start_index);
                    let end = std::cmp::max(self.cursor_index, self.selection_start_index);
                    copied_text = String::with_capacity(end - start);
                    for i in start..end {
                        copied_text.push(self.text[i]);
                    }
                }
                method(&copied_text);
            }
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




use crate::gui::is_point_in_rect;
use crate::gui::style::BoxStyle;
use crate::gui::Rect;
use crate::gui::control::Control;
use crate::gui::keyboard::KeyboardModifiers;
use std::iter::FromIterator;
use std::str::FromStr;

pub struct TextBox {
    pub text: Vec::<char>,
    pub placeholder: &'static str,
    pub bounds: Rect,
    pub hot: bool,
    pub active: bool,
    pub cursor_index: usize, // index of char
    pub selection_index: usize,
    pub style: BoxStyle
}

impl TextBox {
    pub fn get_text(&self) -> String { 
        let start: usize;
        let end: usize;

        if self.selection_index == usize::MAX {
            start = 0;
            end = self.text.len();
        }
        else {
            start = std::cmp::min(self.cursor_index, self.selection_index);
            end = std::cmp::max(self.cursor_index, self.selection_index);
        }

        String::from_iter(&self.text[start..end])
    }

    // replaces all text in the control
    pub fn set_text(&mut self, text: &str) {
        self.text.clear();
        self.set_cursor_index(0);
        self._insert_text(text);
    }

    // inserts text at cursor or replaces text in selection
    pub fn insert_text(&mut self, text: Option<String>) {
        let text = match text {
            Some(t) => t,
            None => String::from_str("").unwrap()
        };
        if self.selection_index == usize::MAX {
            self._insert_text(&text);
        }
        else {
            self.delete();
            self._insert_text(&text);
        }
    }

    fn _insert_text(&mut self, text: &str) {
        for c in text.chars() {
            self.text.insert(self.cursor_index, c);
            self.increment_cursor_index();
        }
        self.selection_index = usize::MAX;
    }

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_index > self.text.len() {
            self.cursor_index = self.text.len();
        }
        self.text.insert(self.cursor_index, c);
        self.increment_cursor_index();
    }

    pub fn delete(&mut self) {
        if self.selection_index == usize::MAX {
            if self.cursor_index < self.text.len() {
                self.text.remove(self.cursor_index);
            }
        }
        else {
            let start = std::cmp::min(self.cursor_index, self.selection_index);
            let end = std::cmp::max(self.cursor_index, self.selection_index);
            for _ in start..end {
                self.text.remove(start);
            }
            self.set_cursor_index(start);
            self.selection_index = usize::MAX;
        }
    }

    pub fn delete_back(&mut self) {
        if self.selection_index == usize::MAX {
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
        // calculate display offset to keep cursor in view
    }

    pub fn increment_cursor_index(&mut self) {
        self.set_cursor_index(self.cursor_index + 1);
    }

    // returns true if cursor decremented, false if not
    pub fn decrement_cursor_index(&mut self) -> bool {
        if self.cursor_index > 0 {
            self.set_cursor_index(self.cursor_index - 1);
            return true;
        }
        return false;
    }

    pub fn select_all(&mut self) {
        self.set_cursor_index(self.text.len());
        self.selection_index = 0;
    }

    pub fn arrow_right(&mut self, modifiers: KeyboardModifiers) {
        self.update_selection_index(modifiers);
        if modifiers.ctrl {
            self.ctrl_jump_cursor(1);
            self.increment_cursor_index();
        }
        else {
            self.increment_cursor_index();
        }
    }

    pub fn arrow_left(&mut self, modifiers: KeyboardModifiers) {
        self.update_selection_index(modifiers);
        if modifiers.ctrl {
            self.ctrl_jump_cursor(-1);
        }
        else {
            self.decrement_cursor_index();
        }
    }

    pub fn home(&mut self, modifiers: KeyboardModifiers) {
        self.update_selection_index(modifiers);
        self.set_cursor_index(0);
        crate::update_window();
    }

    pub fn end(&mut self, modifiers: KeyboardModifiers) {
        self.update_selection_index(modifiers);
        self.set_cursor_index(self.text.len());
        crate::update_window();
    }

    pub fn left_mouse_button_down(&mut self, mouse_x: i32, mouse_y: i32) {
        let hit = is_point_in_rect(mouse_x, mouse_y, self.get_bounds());
        self.hot = hit;
        self.active = hit;
    }

    pub fn copy_to_clipboard(&self) {
        let method_option = unsafe { crate::APPLICATION_STATE.set_clipboard_text_data };
        match method_option {
            None => { },
            Some(method) => method(&self.get_text())
        }
    }

    pub fn cut_to_clipboard(&mut self) {
        let method_option = unsafe { crate::APPLICATION_STATE.set_clipboard_text_data };
        match method_option {
            None => { },
            Some(method) => { 
                method(&self.get_text());
                self.delete();
            }
        }
    }

    fn update_selection_index(&mut self, modifiers: KeyboardModifiers) {
        if modifiers.shift && self.selection_index == usize::MAX {
            self.selection_index = self.cursor_index;
        }
        else if modifiers.shift == false {
            self.selection_index = usize::MAX;
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
        self.set_cursor_index(i as usize);
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

#[cfg(test)]
mod textbox_tests {
    use super::*;
    const TEXT: &str = "1234567890";

    fn create_textbox_for_test() -> TextBox {
        let mut x = TextBox {
            text: Vec::new(),
            placeholder: "placeholder",
            bounds: Rect { x: 10, y: 10, w: 500, h: 100 },
            hot: false, active: false, 
            cursor_index: 0, selection_index: usize::MAX,
            style: BoxStyle::textbox_default()
        };
        x.set_text(TEXT);
        x
    }

    #[test]
    fn test_delete() {
        let mut textbox = create_textbox_for_test();

        // delete single character at index 3 which should be the 4
        textbox.set_cursor_index(3);
        textbox.delete();
        assert_eq!(textbox.cursor_index, 3);
        assert_eq!(textbox.selection_index, usize::MAX);
        assert_eq!(textbox.get_text(), "123567890");

        // delete range of characters from index 3 to 6 
        textbox.selection_index = 6;
        textbox.delete();
        assert_eq!(textbox.cursor_index, 3);
        assert_eq!(textbox.selection_index, usize::MAX);
        assert_eq!(textbox.get_text(), "123890");
    }

    #[test]
    fn test_select_all() {
        let mut textbox = create_textbox_for_test();
        
        textbox.select_all();
        assert_eq!(textbox.cursor_index, 10);
        assert_eq!(textbox.selection_index, 0);
        assert_eq!(textbox.get_text(), TEXT);
    }

    #[test]
    fn test_get_test_selection() {
        let mut textbox = create_textbox_for_test();

        textbox.set_cursor_index(2);
        textbox.selection_index = 7;
        assert_eq!(textbox.get_text(), "34567");
        // makes ure get_text didn't modify the indexes
        assert_eq!(textbox.cursor_index, 2);
        assert_eq!(textbox.selection_index, 7);
        
        textbox.set_cursor_index(8);
        textbox.selection_index = 1;
        assert_eq!(textbox.get_text(), "2345678");
    }
}

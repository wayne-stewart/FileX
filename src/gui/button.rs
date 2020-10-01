
use crate::gui::is_point_in_rect;
use crate::gui::style::BoxStyle;
use crate::gui::Rect;
use crate::gui::control::Control;

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

type ButtonClick = fn(&mut Button) -> ();

impl Button {
    pub fn handle_mouse_button_down(&mut self, mouse_x: i32, mouse_y: i32) {
        let hit = is_point_in_rect(mouse_x, mouse_y, self.get_bounds());
        self.hot = hit;
        self.active = hit;
    }

    pub fn handle_button_mouse_up(&mut self, mouse_x: i32, mouse_y: i32) {
        let hit = is_point_in_rect(mouse_x, mouse_y, self.get_bounds());
        self.hot = hit;
        if self.active && hit {
            match self.on_click {
                Some(method) => method(self),
                None => { }
            }
        }
        self.active = false;
    }
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


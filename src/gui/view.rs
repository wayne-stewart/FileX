
use crate::gui::is_point_in_rect;
use crate::gui::Rect;
use crate::gui::Bounds;
use crate::gui::style::BoxStyle;
use crate::gui::control::Control;
use std::cell::RefCell;

pub enum ViewBehavior {
    None,
    Button
}

impl Default for ViewBehavior {
    fn default() -> ViewBehavior { ViewBehavior::None }
}

pub enum ViewContent {
    None,
    StaticText(&'static str),
    EditableText(Vec::<char>),
    VerticalStack(Vec::<View>),
}

impl<'a> Default for ViewContent {
    fn default() -> ViewContent { ViewContent::None }
}

#[derive(Default)]
pub struct View {
    pub content: ViewContent,
    pub behavior: ViewBehavior,
    pub bounds: Bounds,
    pub bounds_rect: Rect,
    pub hot: bool,
    pub active: bool,
    pub on_click: Option<ButtonClick>,
    pub click_count: i32,
    pub style: BoxStyle,
    pub style_hot: BoxStyle,
    pub style_active: BoxStyle
}

// all methods implemented here will need to be recursive
// to call into child views
impl View {
    pub fn update_bounds_rect(&mut self, width: i32, height: i32) {
        self.bounds_rect = self.bounds.get_rect(width, height);
    }

    pub fn left_mouse_button_down(&mut self, view: &View, mouse_x: i32, mouse_y: i32) {
        let hit = is_point_in_rect(mouse_x, mouse_y, view.bounds_rect);
        self.hot = hit;
        self.active = hit;
        crate::update_window();
    }

    pub fn left_mouse_button_up(&mut self, view: &View, mouse_x: i32, mouse_y: i32) {
        let hit = is_point_in_rect(mouse_x, mouse_y, view.bounds_rect);
        self.hot = hit;
        crate::update_window();
        if self.active && hit {
            match self.on_click {
                Some(method) => method(self),
                None => { }
            }
        }
        self.active = false;
    }

    pub fn mouse_move(&mut self, mouse_x: i32, mouse_y: i32) {
        let hit = is_point_in_rect(mouse_x, mouse_y, self.bounds_rect);
        if self.hot != hit {
            crate::update_window();
            self.hot = hit;
        }
    }

    pub fn get_style<'a>(&'a self) -> &'a BoxStyle {
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

type ButtonClick = fn(&mut View) -> ();

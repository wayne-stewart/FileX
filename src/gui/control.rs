
use crate::gui::is_point_in_rect;
use crate::gui::Rect;
use crate::gui::BoxStyle;

pub trait Control {
    fn get_bounds(&self) -> Rect;
    fn get_hot(&self) -> bool;
    fn set_hot(&mut self, hit: bool);

    fn hit_check(&mut self, mouse_x: i32, mouse_y: i32) -> (bool, bool) {
        let hit = is_point_in_rect(mouse_x, mouse_y, self.get_bounds());
        let mut hot_changed = false;
        if self.get_hot() != hit {
            hot_changed = true;
            self.set_hot(hit);
        }
        (hot_changed, hit)
    }

    fn get_style<'a>(&'a self) -> &'a BoxStyle;
}



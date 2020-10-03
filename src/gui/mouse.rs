
use crate::gui::Cursor;
use crate::gui::control::Control;

pub fn handle_left_mouse_button_down(mouse_x: i32, mouse_y: i32) {
    let buttons = unsafe { &mut crate::APPLICATION_STATE.buttons };
    let textboxes = unsafe { &mut crate::APPLICATION_STATE.textboxes };
    for button in buttons {
        button.left_mouse_button_down(mouse_x, mouse_y);
    }
    for textbox in textboxes {
        textbox.left_mouse_button_down(mouse_x, mouse_y);
    }
}

pub fn handle_left_mouse_button_up(mouse_x: i32, mouse_y: i32) {
    let buttons = unsafe { &mut crate::APPLICATION_STATE.buttons };
    //let textboxes = &mut crate::APPLICATION_STATE.textboxes;
    for button in buttons {
        button.left_mouse_button_up(mouse_x, mouse_y);
    }
    // for textbox in textboxes {
    //     let hit = is_point_in_rect(mouse_x, mouse_y, textbox.get_bounds());
    //     handle_textbox_mouse_up(textbox, hit);
    // }
}

pub fn handle_mouse_move(mouse_x: i32, mouse_y: i32) -> (Cursor, bool) {
    let mut is_button_hot = false;
    let mut is_textbox_hot = false;
    let mut should_update_window = false;

    let buttons = unsafe { &mut crate::APPLICATION_STATE.buttons };
    for button in buttons {
        let (hot_changed, is_hot) = button.hit_check(mouse_x, mouse_y);
        if hot_changed { should_update_window = true }
        if is_hot { is_button_hot = true }
    }

    let textboxes = unsafe { &mut crate::APPLICATION_STATE.textboxes };
    for textbox in textboxes {
        let (hot_changed, is_hot) = textbox.hit_check(mouse_x, mouse_y);
        if hot_changed { should_update_window = true }
        if is_hot { is_textbox_hot = true }
    }

    if is_button_hot { 
        return (Cursor::Hand, should_update_window); 
    }
    else if is_textbox_hot { 
        return (Cursor::IBeam, should_update_window); 
    }
    else { 
        return (Cursor::Arrow, should_update_window); 
    }
}


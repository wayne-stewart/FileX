
use crate::gui::Cursor;
use crate::gui::control::Control;
//use crate::gui::view::ViewBehavior;

pub fn handle_left_mouse_button_down(mouse_x: i32, mouse_y: i32) {
    let buttons = unsafe { &mut crate::APPLICATION_STATE.buttons };
    let textboxes = unsafe { &mut crate::APPLICATION_STATE.textboxes };
    for button in buttons {
        button.left_mouse_button_down(mouse_x, mouse_y);
    }
    for textbox in textboxes {
        textbox.left_mouse_button_down(mouse_x, mouse_y);
    }
    crate::update_window();
    crate::CURSOR_TOGGLE.store(true, std::sync::atomic::Ordering::Relaxed);
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

pub fn handle_mouse_move(mouse_x: i32, mouse_y: i32) -> Cursor {
    let mut is_button_hot = false;
    let mut is_textbox_hot = false;

    let buttons = unsafe { &mut crate::APPLICATION_STATE.buttons };
    for button in buttons {
        let (hot_changed, is_hot) = button.hit_check(mouse_x, mouse_y);
        if hot_changed { crate::update_window() }
        if is_hot { is_button_hot = true }
    }

    let textboxes = unsafe { &mut crate::APPLICATION_STATE.textboxes };
    for textbox in textboxes {
        let (hot_changed, is_hot) = textbox.hit_check(mouse_x, mouse_y);
        if hot_changed { crate::update_window() }
        if is_hot { is_textbox_hot = true }
    }

    // let views =  unsafe { &mut crate::views };
    // for view in views {
    //     match view.behavior {
    //         ViewBehavior::None => { },
    //         ViewBehavior::Button => { 
    //             view.mouse_move(mouse_x, mouse_y);
    //         }
    //     }
    // }


    if is_button_hot { 
        Cursor::Hand 
    }
    else if is_textbox_hot { 
        Cursor::IBeam
    }
    else { 
        Cursor::Arrow
    }
}


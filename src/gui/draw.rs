
use crate::TextBox;
use crate::Button;
use crate::gui::control::Control;
use crate::gui::style::VerticalAlign;
use crate::gui::style::HorizontalAlign;
use crate::gui::style::BoxStyle;
use crate::gui::style::BoxSize;
use crate::gui::is_point_in_rect_a;
use crate::gui::color::Color;
use crate::gui::PixelBuffer;
use crate::gui::Rect;

pub fn draw_textbox(mut buffer: &mut PixelBuffer, textbox: &TextBox, font: &fontdue::Font) {
    let left = textbox.bounds.x;
    let top = textbox.bounds.y;
    let width = textbox.bounds.w;
    let height = textbox.bounds.h;
    let style = textbox.get_style();
    draw_border_box(&mut buffer, &textbox.bounds, &style);
    fill_text(&mut buffer, 
        &textbox.text, 
        left + style.border_size.left + style.padding_size.left, 
        top + style.border_size.top + style.padding_size.top, 
        width - style.border_size.left - style.padding_size.left - style.border_size.right - style.padding_size.right, 
        height - style.border_size.top - style.padding_size.top - style.border_size.bottom - style.padding_size.bottom, 
        &font, style.font_size, 
        textbox.scroll_offset_x,
        style.text_color, style.highlight_color, style.text_highlight_color,
        style.horizontal_align,
        style.vertical_align,
        textbox.cursor_index,
        textbox.selection_index,
        textbox.active);
}

pub fn draw_button(mut buffer: &mut PixelBuffer, button: &Button, font: &fontdue::Font) {
    let left = button.bounds.x;
    let top = button.bounds.y;
    let width = button.bounds.w;
    let height = button.bounds.h;
    let style = button.get_style();
    draw_border_box(&mut buffer, &button.bounds, &style);
    fill_text(&mut buffer, 
        &button.text.chars().collect(),
        left + style.border_size.left + style.padding_size.left, 
        top + style.border_size.top + style.padding_size.top, 
        width - style.border_size.left - style.padding_size.left - style.border_size.right - style.padding_size.right, 
        height - style.border_size.top - style.padding_size.top - style.border_size.bottom - style.padding_size.bottom, 
        &font, style.font_size, 
        0, // scroll_offset_x
        style.text_color, style.highlight_color, style.text_highlight_color,
        style.horizontal_align,
        style.vertical_align,
        0,usize::MAX, false);
}

fn draw_border_box(mut buffer: &mut PixelBuffer, bounds: &Rect, style: &BoxStyle) {
    let left = bounds.x;
    let top = bounds.y;
    let width = bounds.w;
    let height = bounds.h;
    fill_rect(&mut buffer, 
        left + style.border_size.left, 
        top + style.border_size.top, 
        width - style.border_size.left - style.border_size.right, 
        height - style.border_size.top - style.border_size.bottom, 
        style.background_color);
    draw_rect(&mut buffer, left, top, width, height, style.border_size, style.border_color);
}

pub fn fill_rect(buffer: &mut PixelBuffer, left: i32, top: i32, width: i32, height: i32, color: Color) {
    let right = left + width;
    let bottom = top + height;
    let stride = buffer.width;
    for y in top..bottom {
        for x in left..right {
            if  y < buffer.height && x < buffer.width {
                let offset = ((y * stride) + x) as usize;
                let pixel = &mut buffer.pixels[offset];
                pixel.red = color.red;
                pixel.green = color.green;
                pixel.blue = color.blue;
            }
        }
    }
}

fn draw_rect(buffer: &mut PixelBuffer, left: i32, top: i32, width: i32, height: i32, line_width: BoxSize, color: Color) {
    fill_rect(buffer, left, top, width, line_width.top, color); // top
    fill_rect(buffer, left + width - line_width.right, top, line_width.right, height, color); // right
    fill_rect(buffer, left, top + height - line_width.bottom, width, line_width.bottom, color); // bottom
    fill_rect(buffer, left, top, line_width.left, height, color); // left
}

fn alpha_blend_u8(c1: u8, c2 : u8, alpha: u8) -> u8 {
    let c1 = c1 as i32;
    let c2 = c2 as i32;
    let alpha = alpha as i32;
    let inv_alpha = 255 - alpha;
    let result = ((c1 * alpha) + (c2 * inv_alpha));
    return (result / 255) as u8;
}

fn fill_text(buffer: &mut PixelBuffer, text: &Vec::<char>,
    left: i32, top: i32, width: i32, height: i32, 
    font: &fontdue::Font, font_size: f32, scroll_offset_x: i32,
    text_color: Color, highlight_color: Color, highlight_text_color: Color,
    horizontal_align: HorizontalAlign, vertical_align: VerticalAlign,
    cursor_index: usize, selection_index: usize, draw_cursor: bool) {

    let buffer_stride = buffer.width;
    let max_bottom = std::cmp::min(top + height, buffer.height);
    let max_right = std::cmp::min(left + width, buffer.width);
    let max_top = std::cmp::max(top, 0);
    let max_left = std::cmp::max(left, 0);
    let (font_height, _, _,_) = measure_string(&['W'], font, font_size);
    let (_, text_width, _,_char_widths) = measure_string(&text, font, font_size);
    let h_align_offset = calculate_h_align_offset(width, text_width, scroll_offset_x, horizontal_align);
    let v_align_offset = calculate_v_align_offset(height, font_height, vertical_align);
    let mut cursor_left = left + h_align_offset + scroll_offset_x;
    let cursor_top = top + v_align_offset;
    let mut text_char_index = 0;
    let mut cursor_pos = cursor_left;
    let selection_start = std::cmp::min(cursor_index, selection_index);
    let selection_end = std::cmp::max(cursor_index, selection_index);
    for c in text {
        let (font_metrics, font_bitmap) = font.rasterize(*c, font_size);
        let buffer_top = cursor_top + font_height - font_metrics.height as i32 - font_metrics.ymin;
        let buffer_bottom = buffer_top + font_metrics.height as i32;
        let buffer_left = cursor_left;
        let buffer_right = buffer_left + font_metrics.width as i32;
        let mut font_bitmap_index = 0;
        let mut color = text_color;
        if  selection_index != usize::MAX &&
            text_char_index >= selection_start &&
            text_char_index < selection_end {
            color = highlight_text_color;
            let sel_left = std::cmp::max(cursor_left, max_left);
            let sel_width = font_metrics.advance_width as i32;
            let sel_right = std::cmp::min(cursor_left + sel_width, max_right);
            let sel_width = sel_right - sel_left;
            if sel_width > 0 && sel_right > 0 {
                fill_rect(buffer, sel_left, cursor_top - 2, sel_width, font_height + 4, highlight_color);                
            }
        }
        for buffer_y in buffer_top..buffer_bottom {
            for buffer_x in buffer_left..buffer_right {
                if  is_point_in_rect_a(buffer_x, buffer_y, max_left, max_top, max_right, max_bottom) {
                    let buffer_index = (buffer_y * buffer_stride) + buffer_x;
                    let buffer_pixel = &mut buffer.pixels[buffer_index as usize];
                    let font_pixel = font_bitmap[font_bitmap_index];
                    if font_pixel > 0 {
                        buffer_pixel.red = alpha_blend_u8(color.red, buffer_pixel.red, font_pixel);
                        buffer_pixel.green = alpha_blend_u8(color.green, buffer_pixel.green, font_pixel);
                        buffer_pixel.blue = alpha_blend_u8(color.blue, buffer_pixel.blue, font_pixel);
                    }
                }
                font_bitmap_index += 1;
            }
        }
        cursor_left += font_metrics.advance_width as i32;
        if cursor_index > text_char_index {
            cursor_pos = cursor_left;
        }
        text_char_index += 1;
    }

    if draw_cursor {
        fill_rect(buffer, 
            cursor_pos,
            cursor_top - 2,
            2, // width
            font_height + 4, // height
            text_color);
    }
}

fn calculate_h_align_offset(container_width: i32, text_width: i32, scroll_offset_x: i32, align: HorizontalAlign) -> i32 {
    match align {
        HorizontalAlign::Left => 0,
        //HorizontalAlign::Right => left, // I don't need this one yet so I'll wait on the implementation
        HorizontalAlign::Center => {
            if text_width > container_width {
                0
            }
            else {
                (container_width / 2) - (text_width / 2)
            }
        }
    }
}

fn calculate_v_align_offset(container_height: i32, text_height: i32, align: VerticalAlign) -> i32 {
    match align {
        VerticalAlign::Center => {
            container_height / 2 - text_height / 2
        },
        VerticalAlign::Bottom =>  { 
            container_height - text_height
        },
        VerticalAlign::Top => {
            0
        }
    }
}

/*
    return a tuple of height, width, baseline
*/
pub fn measure_string(text: &[char], font: &fontdue::Font, font_size: f32) -> (i32, i32, i32, Vec<i32>) {
    let mut height: i32 = 0;
    let mut width: i32 = 0;
    let mut ymin: i32 = 0;
    let mut char_widths: Vec<i32> = Vec::<i32>::with_capacity(text.len());
    for c in text {
        let m = font.metrics(*c, font_size);
        if height < m.height as i32 {
            height = m.height as i32;
        }
        width += m.advance_width as i32;
        char_widths.push(m.advance_width as i32);
        if ymin > m.ymin {
            ymin = m.ymin;
        }
    }
    (height, width, ymin, char_widths)
}


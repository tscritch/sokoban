use raylib::prelude::*;

// sprites 8x8
pub fn get_rect_from_index(sheet: &Texture2D, index: i32) -> Rectangle {
    let sheet_width = sheet.width();
    let x = (index * 8 % sheet_width) as f32;
    let y = (index * 8 / sheet_width) as f32;
    let width = 8.0;
    let height = 8.0;
    Rectangle {
        x,
        y,
        width,
        height,
    }
}

// draws 8x8 sprites from an index on the sheet
pub fn draw_sprite(draw: &mut RaylibDrawHandle, sheet: &Texture2D, index: i32, position: &Vector2) {
    let rect: Rectangle = get_rect_from_index(sheet, index);
    draw.draw_texture_rec(&sheet, rect, position, Color::WHITE)
}

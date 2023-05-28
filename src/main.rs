use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("sokoban").build();
    rl.set_window_position(1200, 120);

    let font = rl
        .load_font(&thread, "resources/fonts/alpha_beta.png")
        .expect("couldn't load font");
    let font_spacing = 2.0;
    let position: Vector2 = Vector2 { x: 12.0, y: 12.0 };

    let sprite_sheet: Texture2D = rl
        .load_texture(&thread, "resources/tiles.png")
        .expect("couldn't load tiles");
    sprite_sheet.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);

    while !rl.window_should_close() {
        let wp = rl.get_window_position();

        // --- BEGIN DRAW ---
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        d.draw_text_ex(
            &font,
            &format!("Window\nx:{}\ny:{}", wp.x, wp.y),
            Vector2 { x: 12.0, y: 24.0 },
            12.0,
            font_spacing,
            Color::BLACK,
        );

        // d.draw_texture_ex(
        //     &sprite_sheet,
        //     Vector2 { x: 100.0, y: 100.0 },
        //     0.0,
        //     3.0,
        //     Color::WHITE,
        // );



        // --- END DRAW ---
    }

}

// draws 8x8 sprites from an index on the sheet
fn draw_sprite(draw: &mut RaylibDrawHandle, sheet: &Texture2D, index: u8, position: &Vector2) {
    let crop_rect: Rectangle = Rectangle { x: (), y: (), width: (), height: () }
    draw.draw_texture_rec(&sheet, source_rec, position, Color::WHITE)
}

// sprites 8x8
fn get_rect_from_index()
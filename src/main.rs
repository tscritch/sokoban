use raylib::prelude::*;

mod tiles;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("sokoban").build();
    rl.set_window_position(800, 120);

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

        tiles::draw_sprite(&mut d, &sprite_sheet, 1, &position);
        tiles::draw_sprite(&mut d, &sprite_sheet, 2, &Vector2 { x: 20.0, y: 12.0 });
        tiles::draw_sprite(&mut d, &sprite_sheet, 3, &Vector2 { x: 28.0, y: 12.0 });
        tiles::draw_sprite(&mut d, &sprite_sheet, 4, &Vector2 { x: 36.0, y: 12.0 });
        tiles::draw_sprite(&mut d, &sprite_sheet, 5, &Vector2 { x: 42.0, y: 12.0 });
        tiles::draw_sprite(&mut d, &sprite_sheet, 6, &Vector2 { x: 50.0, y: 12.0 });

        // --- END DRAW ---
    }
}

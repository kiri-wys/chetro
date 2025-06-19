mod app;

use app::board::sprites::{PieceMappings, SpritesMap};
use macroquad::prelude::*;
use tracing_subscriber::FmtSubscriber;

const TARGET_RESOLUTION: Vec2 = Vec2 {
    x: 1920.0,
    y: 1080.0,
};

#[macroquad::main("Chetro")]
async fn main() {
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let s = std::fs::read_to_string("assets/pieces.json").unwrap();
    let mappings: PieceMappings = serde_json::from_str(&s).unwrap();
    let white_sprites = SpritesMap {
        atlas: load_texture("assets/pieces.png").await.unwrap(),
        mappings,
    };
    let black_sprites = SpritesMap {
        atlas: load_texture("assets/pieces_black.png").await.unwrap(),
        mappings,
    };
    let mut game = app::Game::new(
        white_sprites,
        black_sprites,
        load_texture("assets/move_gizmo.png").await.unwrap(),
    );

    let render_target = render_target(TARGET_RESOLUTION.x as u32, TARGET_RESOLUTION.y as u32);
    render_target.texture.set_filter(FilterMode::Linear);

    let mut render_target_cam =
        Camera2D::from_display_rect(Rect::new(0., 0., TARGET_RESOLUTION.x, TARGET_RESOLUTION.y));
    render_target_cam.render_target = Some(render_target.clone());

    loop {
        clear_background(DARKGRAY);

        // Render to a virtual workspace
        set_camera(&render_target_cam);
        clear_background(GRAY);
        let scale: f32 = f32::max(
            screen_width() / TARGET_RESOLUTION.x,
            screen_height() / TARGET_RESOLUTION.y,
        );

        game.ctx.mouse_position = Vec2 {
            x: mouse_position().0 / scale,
            y: mouse_position().1 / scale,
        };
        game.draw();
        game.update();

        // Render directly to the screen
        set_default_camera();
        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    TARGET_RESOLUTION.x * scale,
                    TARGET_RESOLUTION.y * scale,
                )),
                flip_y: true,
                ..Default::default()
            },
        );
        draw_text(&get_fps().to_string(), 0.0, 16.0, 32.0, GREEN);

        next_frame().await
    }
}

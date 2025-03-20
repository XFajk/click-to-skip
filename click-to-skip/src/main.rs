mod enemies;
mod main_game;
mod player;
mod skip_button;

use std::collections::VecDeque;

use cane::{management::GameObject, scene::*};
use macroquad::prelude::*;
use main_game::MainGame;

pub const RENDER_TARGET_SIZE: Vec2 = vec2(400., 300.);

#[macroquad::main(window_conf)]
async fn main() {
    let mut scenes: SceneQueue = VecDeque::new();

    schedule_scene(MainGame::new().await);

    let render_target = render_target(400, 300);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut camera = Camera2D::from_display_rect(Rect::new(0., 300., 400., -300.));
    camera.render_target = Some(render_target.clone());

    loop {
        let dt = get_frame_time();

        set_camera(&camera);
        for GameObject { object: scene, .. } in scenes.iter_mut() {
            scene.update(dt);
            scene.render(dt);
        }

        set_default_camera();

        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        draw_text(
            &format!("FPS: {}", get_fps()),
            10.0,
            20.0,
            30.0,
            Color::new(0.0, 0.0, 0.0, 1.0),
        );

        scenes = scenes
            .into_iter()
            .filter(|GameObject { object: scene, .. }| !scene.is_scheduled_for_removal())
            .collect();

        apply_modifier_to_scenes(&mut scenes);

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Click to Skip".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        ..Default::default()
    }
}

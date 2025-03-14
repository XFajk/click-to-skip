mod player;

use std::collections::VecDeque;

use cane::scene::*;
use macroquad::prelude::*;
use player::Player;

#[macroquad::main("Click to skip")]
async fn main() {
    let mut scenes: SceneQueue = VecDeque::new();

    schedule_scene(Box::new(MainGame::new));

    loop {
        let dt = get_frame_time();

        for scene in scenes.iter_mut() {
            scene.update(dt);
            scene.render(dt);
        }

        scenes = scenes
            .into_iter()
            .filter(|scene| !scene.is_scheduled_for_removal())
            .collect();

        transfer_scheduled_scenes(&mut scenes);

        next_frame().await;
    }
}

struct MainGame {
    scheduled_for_removal: bool,
    player: Player,
}

impl MainGame {
    fn new() -> Box<dyn Scene> {
        Box::new(Self {
            scheduled_for_removal: false,
            player: Player::new(Vec2::new(40.0, 40.0)),
        })
    }
}

impl Scene for MainGame {
    fn update(&mut self, dt: f32) {
        self.player.update(dt);
    }

    fn render(&mut self, dt: f32) {
        clear_background(Color::new(1.0, 1.0, 1.0, 1.0));
        draw_text(
            &format!("FPS: {}", get_fps()),
            5.0,
            5.0,
            10.0,
            Color::new(0.0, 0.0, 0.0, 1.0),
        );

        self.player.render(dt);
    }

    fn schedule_for_removal(&mut self) {
        self.scheduled_for_removal = true
    }

    fn is_scheduled_for_removal(&self) -> bool {
        self.scheduled_for_removal
    }
}

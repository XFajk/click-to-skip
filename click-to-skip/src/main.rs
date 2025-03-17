mod player;
mod skip_button;
mod enemies;

use std::collections::VecDeque;

use cane::scene::*;
use enemies::{Enemy, ProjectileManager, ShampooEnemy};
use macroquad::prelude::*;
use player::Player;
use skip_button::SkipButton;

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
        for scene in scenes.iter_mut() {
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
            .filter(|scene| !scene.is_scheduled_for_removal())
            .collect();

        transfer_scheduled_scenes(&mut scenes);

        next_frame().await;
    }
}

struct MainGame {
    scheduled_for_removal: bool,
    projectile_manager: ProjectileManager,

    player: Player,
    skip_button: SkipButton,
    shampoo_enemy: ShampooEnemy,
}

impl MainGame {
    async fn new() -> Box<dyn Scene> {
        let player = Player::new(vec2(40.0, 40.0)).await;
        Box::new(Self {
            scheduled_for_removal: false,
            projectile_manager: ProjectileManager::new(),

            player,
            skip_button: SkipButton::new(vec2(100., 100.)).await,
            shampoo_enemy: ShampooEnemy::new(vec2(200., 200.), vec2(70., 0.)).await,
        })
    }
}

impl Scene for MainGame {
    fn update(&mut self, dt: f32) {
        self.player.update(dt);
        self.skip_button.update(dt, self.player.body.point());

        self.shampoo_enemy.update(dt, &mut self.projectile_manager);
    }

    fn render(&mut self, dt: f32) {
        clear_background(DARKGRAY);
        
        self.skip_button.render(self.player.body.point());
        self.player.render(dt);
        self.shampoo_enemy.render(dt);
    }

    fn schedule_for_removal(&mut self) {
        self.scheduled_for_removal = true
    }

    fn is_scheduled_for_removal(&self) -> bool {
        self.scheduled_for_removal
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

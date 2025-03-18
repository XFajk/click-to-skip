use crate::{player::{self, Player}, RENDER_TARGET_SIZE};
use cane::animation::{Animation, AnimationManager};
use macroquad::prelude::*;

pub struct EnemyManager {}


pub trait Enemy {
    fn update(&mut self, dt: f32, player: &mut Player);
    fn render(&self, dt: f32);

    fn schedule_for_removal(&mut self);
    fn is_scheduled_for_removal(&self) -> bool;
}

pub struct ShampooEnemy {
    body: Rect,
    sprite_sheet: Texture2D,
    animation_manager: AnimationManager<Rect>,
    velocity: Vec2,
}

impl ShampooEnemy {
    pub async fn new(position: Vec2, spawn_velocity: Vec2) -> Self {
        let body = Rect::new(position.x, position.y, 36., 47.);
        let mut animation_manager = AnimationManager::new();

        let mut animation = Animation::new_single_duration(
            vec![
                Rect::new(0., 0., body.w, body.h),
                Rect::new(body.w, 0., body.w, body.h),
                Rect::new(body.w * 2., 0., body.w, body.h),
                Rect::new(body.w * 3., 0., body.w, body.h),
                Rect::new(body.w * 4., 0., body.w, body.h),
            ],
            0.16,
        );
        animation.auto_reset = true;
        animation_manager.add_animation("idle", animation);

        let mut animation = Animation::new_single_duration(
            vec![
                Rect::new(0., body.h, body.w, body.h),
                Rect::new(body.w, body.h, body.w, body.h),
                Rect::new(body.w * 2., body.h, body.w, body.h),
                Rect::new(body.w * 3., body.h, body.w, body.h),
                Rect::new(body.w * 4., body.h, body.w, body.h),
            ],
            0.16,
        );
        animation.auto_reset = true;
        animation_manager.add_animation("blink", animation);

        animation_manager.play("idle");

        Self {
            body,
            sprite_sheet: load_texture("./assets/textures/shampoo_enemy.png")
                .await
                .unwrap(),
            animation_manager,
            velocity: spawn_velocity,
        }
    }
}

impl Enemy for ShampooEnemy {
    fn update(&mut self, dt: f32, player: &mut Player) {
        self.animation_manager.update(dt);

        if self.body.right() > RENDER_TARGET_SIZE.x || self.body.left() < 0. {
            self.velocity.x *= -1.;
        }

        if self.body.bottom() > RENDER_TARGET_SIZE.y || self.body.top() < 0. {
            self.velocity.y *= -1.;
        }

        self.body.x += self.velocity.x * dt;
        self.body.y += self.velocity.y * dt;
    }

    fn render(&self, dt: f32) {
        draw_texture_ex(
            &self.sprite_sheet,
            self.body.x.floor(),
            self.body.y.floor(),
            WHITE,
            DrawTextureParams {
                source: Some(self.animation_manager.get_frame().clone()),
                ..Default::default()
            },
        );
    }

    fn schedule_for_removal(&mut self) {}

    fn is_scheduled_for_removal(&self) -> bool {
        todo!("Implement this method")
    }
}

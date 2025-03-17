use macroquad::prelude::*;
use crate::{player::Player, RENDER_TARGET_SIZE};

pub struct ProjectileManager {
    projectiles: Vec<Box<dyn Projectile>>,
}

impl ProjectileManager {
    pub fn new() -> Self {
        Self {
            projectiles: Vec::new(),
        }
    }
}

pub trait Enemy {
    fn update(&mut self, dt: f32, projectile_manager: &mut ProjectileManager);
    fn render(&self, dt: f32);
   
    fn schedule_for_removal(&mut self);
    fn is_scheduled_for_removal(&self) -> bool;
}

pub trait Projectile {
    fn update(&mut self, dt: f32, player: Player);
    fn render(&self, dt: f32);
    
    fn schedule_for_removal(&mut self);
    fn is_scheduled_for_removal(&self) -> bool;
}

pub struct ShampooEnemy {
    body: Rect,
    texture: Texture2D,
    velocity: Vec2, 
}

impl ShampooEnemy {
    pub async fn new(position: Vec2, spawn_velocity: Vec2) -> Self {
        Self {
            body: Rect::new(position.x, position.y, 36., 47.),
            texture: load_texture("./assets/textures/shampoo_enemy.png").await.unwrap(),
            velocity: spawn_velocity,
        }
    }
}

impl Enemy for ShampooEnemy {
    fn update(&mut self, dt: f32, projectile_manager: &mut ProjectileManager) {
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
        draw_texture_ex(&self.texture, self.body.x.floor(), self.body.y.floor(), WHITE, DrawTextureParams {
            source: Some(Rect::new(0., 0., self.body.w, self.body.h)),
            ..Default::default()
        });
    }

    fn schedule_for_removal(&mut self) {

    }

    fn is_scheduled_for_removal(&self) -> bool {
        todo!("Implement this method")    
    }
}
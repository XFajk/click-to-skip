use macroquad::prelude::*;

const SPEED: f32 = 120.0;

pub struct Player {
    pub body: Rect,
    texture: Texture2D,
}

impl Player {
    pub async fn new(spawn_point: Vec2) -> Self {
        Self {
            body: Rect::new(spawn_point.x, spawn_point.y, 12., 18.0),
            texture: load_texture("./assets/textures/cursor.png").await.unwrap(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut direction: Vec2 = Vec2::ZERO;

        if is_key_down(KeyCode::W) {
            direction += vec2(0.0, -1.0);
        }
        if is_key_down(KeyCode::S) {
            direction += vec2(0.0, 1.0);
        }
        if is_key_down(KeyCode::D) {
            direction += vec2(1.0, 0.0);
        }
        if is_key_down(KeyCode::A) {
            direction += vec2(-1.0, 0.0);
        }

        direction = if direction.length() > 0.0 {
            direction.normalize()*SPEED*dt
        } else {
            direction
        };


        self.body.x += direction.x;
        self.body.y += direction.y;

    }

    pub fn render(&mut self, _dt: f32) {
        draw_texture(&self.texture, self.body.x.floor(), self.body.y.floor(), WHITE);
    }
}

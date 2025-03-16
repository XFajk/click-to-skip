use macroquad::prelude::*;

pub struct SkipButton {
    body: Rect,
}

impl SkipButton {
    pub async fn new(spawn_point: Vec2) -> Self {
        Self {
            body: Rect::new(spawn_point.x, spawn_point.y, 48., 16.),
        }
    }

    fn is_colliding_with(&self, rect: Vec2) -> bool {
        self.body.contains(rect)
    }

    pub fn update(&self, dt: f32, cursor_point: Vec2) {
        if self.is_colliding_with(cursor_point) && is_key_pressed(KeyCode::Space) {
            todo!()
        }
    }

    pub fn render(&self, cursor_point: Vec2) {
        if self.is_colliding_with(cursor_point) {
            draw_rectangle(self.body.x, self.body.y, self.body.w, self.body.h, Color::new(0.3, 0.3, 0.3, 0.5));
        } else {
            draw_rectangle(self.body.x, self.body.y, self.body.w, self.body.h, Color::new(0.1, 0.1, 0.1, 0.5));
        }
    }
}

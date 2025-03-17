use macroquad::prelude::*;

pub struct SkipButton {
    body: Rect,
    texture: Texture2D,
}

impl SkipButton {
    pub async fn new(spawn_point: Vec2) -> Self {
        Self {
            body: Rect::new(spawn_point.x, spawn_point.y, 48., 16.),
            texture: load_texture("./assets/textures/skip_button.png").await.unwrap(),
        }
    }

    fn is_colliding_with(&self, rect: Vec2) -> bool {
        self.body.contains(rect)
    }

    pub fn update(&self, _dt: f32, cursor_point: Vec2) {
        if self.is_colliding_with(cursor_point) && is_key_pressed(KeyCode::Space) {
            todo!()
        }
    }

    pub fn render(&self, cursor_point: Vec2) {
        if self.is_colliding_with(cursor_point) {
            draw_texture_ex(&self.texture, self.body.x, self.body.y, WHITE, DrawTextureParams {
                source: Some(Rect::new(48., 0., 48., 16.)),
                ..Default::default()
            });
        } else {
            draw_texture_ex(&self.texture, self.body.x, self.body.y, WHITE, DrawTextureParams {
                source: Some(Rect::new(0., 0., 48., 16.)),
                ..Default::default()
            });
        }
    }
}

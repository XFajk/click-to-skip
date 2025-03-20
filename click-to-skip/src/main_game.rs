use crate::enemies::EnemyManager; 
use crate::player::Player;
use crate::skip_button::SkipButton;
use cane::scene::*;
use macroquad::prelude::*;


pub struct MainGame {
    scheduled_for_removal: bool,
    enemy_manager: EnemyManager,

    player: Player,
    skip_button: SkipButton,
}

impl MainGame {
    pub async fn new() -> Box<dyn Scene> {
        let player = Player::new(vec2(40.0, 40.0)).await;
        Box::new(Self {
            scheduled_for_removal: false,
            enemy_manager: EnemyManager::new(),

            player,
            skip_button: SkipButton::new(vec2(100., 100.)).await,
        })
    }
}

impl Scene for MainGame {
    fn update(&mut self, dt: f32) {
        self.player.update(dt);
        self.skip_button.update(dt, self.player.body.point());
    }

    fn render(&mut self, dt: f32) {
        clear_background(DARKGRAY);
        
        self.skip_button.render(self.player.body.point());
        self.player.render(dt);
    }

    fn schedule_for_removal(&mut self) {
        self.scheduled_for_removal = true
    }

    fn is_scheduled_for_removal(&self) -> bool {
        self.scheduled_for_removal
    }
}



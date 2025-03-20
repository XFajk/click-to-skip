use crate::enemies::{EnemyManager, shampoo_enemy::ShampooEnemy};
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
        let mut enemy_manager = EnemyManager::new();
        enemy_manager.push_enemy(Box::new(
            ShampooEnemy::new(vec2(200., 200.), vec2(100., 0.)).await,
        ));

        Box::new(Self {
            scheduled_for_removal: false,
            enemy_manager,

            player,
            skip_button: SkipButton::new(vec2(100., 100.)).await,
        })
    }
}

impl Scene for MainGame {
    fn update(&mut self, dt: f32) {
        self.player.update(dt);
        self.skip_button.update(dt, self.player.body.point());
        
        self.enemy_manager.update(dt, &mut self.player);
    }

    fn render(&mut self, dt: f32) {
        clear_background(DARKGRAY);

        self.enemy_manager.render(dt);
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

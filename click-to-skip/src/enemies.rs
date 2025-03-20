pub mod shampoo_enemy;

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::player::Player;
use macroquad::prelude::*;
use uuid::Uuid;

pub trait Enemy {
    fn update(&mut self, dt: f32, player: &mut Player, modifier_queue: &mut VecDeque<Box<dyn EmenyQueueModifier>>);
    fn render(&self, dt: f32);

    fn schedule_for_removal(&mut self);
    fn is_scheduled_for_removal(&self) -> bool;
}

pub trait EmenyQueueModifier {
    fn modify(&mut self, enemy_queue: &mut Vec<EnemyHolder>);
}

pub struct EnemyHolder {
    id: Rc<Uuid>,
    name: Option<Rc<str>>,
    scene: Box<dyn Enemy>
}

pub struct EnemyManager {
    pub enemies: RefCell<Vec<EnemyHolder>>,
    enemy_modifier_queue: RefCell<VecDeque<Box<dyn EmenyQueueModifier>>>
}

impl EnemyManager {
    pub fn new() -> Self {
        Self {
            enemies: RefCell::new(Vec::new()),
            enemy_modifier_queue: RefCell::new(VecDeque::new()),
        }
    }
}


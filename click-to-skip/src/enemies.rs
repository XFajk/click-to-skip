pub mod shampoo_enemy;

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::player::Player;
use macroquad::prelude::*;
use uuid::Uuid;
use cane::management::GameObject;

pub trait Enemy {
    fn update(&mut self, dt: f32, player: &mut Player, modifier_queue: &mut VecDeque<Box<dyn EmenyQueueModifier>>);
    fn render(&self, dt: f32);

    fn schedule_for_removal(&mut self);
    fn is_scheduled_for_removal(&self) -> bool;
}

pub type EnemyHolder = GameObject<Box<dyn Enemy>>;

pub trait EmenyQueueModifier {
    fn modify(&mut self, enemy_queue: &mut Vec<EnemyHolder>);
}

pub struct PushEnemyMod(Option<EnemyHolder>);

impl EmenyQueueModifier for PushEnemyMod {
    fn modify(&mut self, enemy_queue: &mut Vec<EnemyHolder>) {
        enemy_queue.push(self.0.take().unwrap());
    }
}

pub struct EnemyManager {
    pub enemies: RefCell<VecDeque<EnemyHolder>>,
    enemy_modifier_queue: RefCell<VecDeque<Box<dyn EmenyQueueModifier>>>
}

impl EnemyManager {
    pub fn new() -> Self {
        Self {
            enemies: RefCell::new(VecDeque::new()),
            enemy_modifier_queue: RefCell::new(VecDeque::new()),
        }
    }

    pub fn update(&mut self, dt: f32, player: &mut Player) {
        for enemy in self.enemies.borrow_mut().iter_mut() {
            enemy.update(dt, player, &mut self.enemy_modifier_queue.borrow_mut());
        }
    }

    pub fn render(&self, dt: f32) {
        for enemy in self.enemies.borrow().iter() {
            enemy.render(dt);
        }
    }

    pub fn push_enemy(&mut self, enemy: Box<dyn Enemy>) {
        self.enemies.borrow_mut().push_front(EnemyHolder {
            id: Rc::new(Uuid::new_v4()),
            name: None,
            object: enemy
        });
    }

    pub fn push_named_enemy(&mut self, enemy: Box<dyn Enemy>, name: &str) {
        self.enemies.borrow_mut().push_front(EnemyHolder {
            id: Rc::new(Uuid::new_v4()),
            name: Some(name.into()),
            object: enemy
        });
    }

    pub fn schedule_enemy(&mut self, enemy: Box<dyn Enemy>) {
        self.enemy_modifier_queue.borrow_mut().push_front(Box::new(PushEnemyMod(Some(EnemyHolder {
            id: Rc::new(Uuid::new_v4()),
            name: None,
            object: enemy
        }))));
    }

    pub fn schedule_named_enemy(&mut self, enemy: Box<dyn Enemy>, name: &str) {
        self.enemy_modifier_queue.borrow_mut().push_front(Box::new(PushEnemyMod(Some(EnemyHolder {
            id: Rc::new(Uuid::new_v4()),
            name: Some(name.into()),
            object: enemy
        }))));
    }
}


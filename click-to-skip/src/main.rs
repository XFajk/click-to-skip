mod context;
mod scene;

use std::{cell::RefCell, rc::Rc};

use context::{Context, RefContext};
use macroquad::prelude::*;
use scene::Scene;

#[macroquad::main("Click to skip")]
async fn main() {
    let ctx = RefContext::new();    
    ctx.borrow_mut().scenes.insert("main_game", Box::new(MainGame::new));

    ctx.push_to_scene_stack("main_game");

    loop {
        ctx.update();

        for scene in ctx.borrow_mut().scene_stack.iter_mut() {
            scene.update();
            scene.render();
        }

        next_frame().await;
    } 
}

struct MainGame {
    ctx: Rc<RefCell<Context>>,
}

impl MainGame {
    fn new(ctx: Rc<RefCell<Context>>) -> Box<dyn Scene> {
        Box::new(Self{
            ctx,
        })
    }
}

impl Scene for MainGame {
    fn update(&mut self) {
        
    }

    fn render(&mut self) {
        
    }
}

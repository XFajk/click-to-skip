use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    ops::Deref,
    rc::Rc,
};

use macroquad::{math::U16Vec2, time::get_frame_time};

use crate::scene::Scene;

pub struct Context {
    pub scenes: HashMap<&'static str, Box<dyn FnMut(Rc<RefCell<Self>>) -> Box<dyn Scene>>>,
    pub scene_stack: VecDeque<Box<dyn Scene>>,

    pub delta_time: f32,

    pub window_size: U16Vec2,
    pub resolution_size: U16Vec2,
}

pub struct RefContext(Rc<RefCell<Context>>);

impl RefContext {
    pub fn new() -> Self {
        RefContext(Rc::new(RefCell::new(Context {
            scenes: HashMap::new(),
            scene_stack: VecDeque::new(),
            delta_time: 1.0,
            window_size: U16Vec2::ZERO,
            resolution_size: U16Vec2::ZERO,
        })))
    }

    pub fn update(&self) {
        self.borrow_mut().delta_time = get_frame_time();
    }

    pub fn push_to_scene_stack(&self, new_scene_name: &'static str) {
        let mut borrowed_ctx = self.borrow_mut();
        let scene_constructor = borrowed_ctx
            .scenes
            .get_mut(new_scene_name)
            .expect(&format!("A scene with a name {new_scene_name} was never created"));
        let scene = scene_constructor(Rc::clone(&self));

        borrowed_ctx.scene_stack.push_back(scene);
    }
}

impl Deref for RefContext {
    type Target = Rc<RefCell<Context>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

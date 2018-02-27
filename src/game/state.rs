use graphics::pixel::Pixel;
use game::scene::Scene;
use std::rc::Rc;
pub struct GameState {
    tick: u64,
    current_scene: Option<Rc<Box<Scene>>>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            tick: 0,
            current_scene: None
        }
    }
    pub fn set_current_scene(&mut self, scene: Rc<Box<Scene>>) {
        self.current_scene = Some(scene);
    }
    pub fn update(&mut self) {
        self.tick += 1;
    }
    //TODO: Add utility functions
}
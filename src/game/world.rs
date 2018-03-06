use game::scene::Scene;
use std::collections::HashMap;
use game::event::Event;
pub struct World {
    scene_creators: HashMap<String, fn()-> Scene>,
    current_scene: Option<Scene>,
    default_scene: String,
    state: WorldState
}

impl World {
    pub fn new(scenes: HashMap<String, fn() -> Scene>, default_scene_name: String, dimensions: (u32,u32)) -> World {
        World {
            scene_creators: scenes,
            current_scene: None,
            default_scene: default_scene_name,
            state: WorldState::new(dimensions)
        }
    }
    pub fn set_scene(&mut self, scene_name: String) {
        self.current_scene = Some(self.scene_creators.get(&scene_name).unwrap()());
        self.current_scene.as_mut().unwrap().run_init(&self.state)
    }

    pub fn run_events(&mut self, events: Vec<Event>) {

    }
}

pub struct WorldState {
    tick: u64,
    entity_count: u32,
    dimensions: (u32,u32)
}

impl WorldState {
    pub fn new(dimensions: (u32,u32)) -> WorldState {
        WorldState {
            tick: 0,
            entity_count: 0,
            dimensions
        }
    }
}

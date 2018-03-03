use game::scene::Scene;
use std::collections::HashMap;
pub struct World {
    scene_creators: HashMap<String, fn()-> Scene>
}

impl World {
    pub fn set_scene(&mut self, scene_name: String) {}
}

pub struct WorldState {}
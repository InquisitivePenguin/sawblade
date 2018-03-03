use game::gameobject::GameObject;
use graphics::texture::FinalTexture;
use graphics::pixel::Pixel;
use std::marker::Sized;
use std::rc::Weak;
use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;
use game::world::WorldState;
use game::event::Event;

pub enum SceneMsg {
    Continue,
    ExitTo(String),
    Pause
}

pub struct SceneBuilder {}

impl SceneBuilder {
    pub fn blank() -> Scene {
        Scene {
            entities: vec![],
            dispatch: Box::new(Scene::default_dispatch),
            name: "Blank".to_string()
        }
    }
}

pub struct Scene {
    entities: Vec<Box<GameObject>>,
    dispatch: Box<fn(&mut Scene, Vec<Event>, &WorldState)>,
    name: String
    //collision_manager: CollsionManager
}

impl Scene {
    pub fn tick(&mut self) {
        println!("Running tick function!");
    }
    fn default_dispatch(scene: &mut Scene, input: Vec<Event>, state: &WorldState) {
        scene.tick();
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
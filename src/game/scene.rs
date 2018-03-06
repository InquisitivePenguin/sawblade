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

pub struct SceneBuilder {
    init: Option<Box<fn(&mut Scene, &WorldState)>>,
    name: String
}

impl SceneBuilder {
    pub fn blank() -> Scene {
        Scene {
            entities: vec![],
            init: Box::new(Scene::default_init),
            name: "Blank".to_string()
        }
    }

    pub fn new(name: String) -> SceneBuilder {
        SceneBuilder {
            init: None,
            name
        }
    }

    pub fn override_init(mut self, init_fn: fn(&mut Scene, &WorldState)) -> SceneBuilder {
        self.init = Some(Box::new(init_fn));
        self
    }

    pub fn build(mut self) -> Scene {
        Scene {
            entities: vec![],
            name: self.name,
            init: match self.init {
                Some(func) => func,
                None => Box::new(Scene::default_init)
            }
        }
    }
}

pub struct Scene {
    entities: Vec<Box<GameObject>>,
    init: Box<fn(&mut Scene, &WorldState)>,
    name: String
}

impl Scene {
    pub fn tick(&mut self) {
        println!("Running tick function!");
    }
    fn default_init(scene: &mut Scene, state: &WorldState) {}
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn run_init(&mut self, state: &WorldState) {
        self.init.deref()(self, state);
    }
}
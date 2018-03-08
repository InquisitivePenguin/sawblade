use game::gameobject::GameObject;
use graphics::texture::FinalTexture;
use game::world::WorldState;
use game::event::Event;
use std::ops::Deref;

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
            name: "Blank".to_string(),
            next_id: 0
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

    pub fn build(self) -> Scene {
        Scene {
            entities: vec![],
            name: self.name,
            init: match self.init {
                Some(func) => func,
                None => Box::new(Scene::default_init)
            },
            next_id: 0
        }
    }
}

pub struct Scene {
    pub entities: Vec<Box<GameObject>>,
    init: Box<fn(&mut Scene, &WorldState)>,
    name: String,
    next_id: u64
}

impl Scene {
    // TODO: Use events & world
    pub fn tick(&mut self, events: Vec<Event>, world: &WorldState) -> Vec<FinalTexture> {
        let self_ptr = self as *mut Scene;
        for entity in &mut self.entities {
            entity.as_mut().tick(self_ptr);
        }
        let collected_textures = {
            let mut texture_collector = vec![];
            for entity in &mut self.entities {
                texture_collector.push(entity.as_mut().render().expect("Whoops"));
            }
            texture_collector
        };
        collected_textures
    }
    fn default_init(_scene: &mut Scene, _state: &WorldState) {

    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn run_init(&mut self, state: &WorldState) {
        self.init.deref()(self, state);
    }

    pub fn spawn<T : GameObject + 'static>(&mut self, coordinates: (u32,u32)) {
        self.entities.push(Box::new(T::spawn(coordinates, self.next_id)));
        self.next_id += 1;
    }

    pub fn get_entity_by_id(&mut self, id: u64) -> Option<&mut GameObject> {
        for mut entity in &mut self.entities {
            if entity.get_id() == id {
                return Some(entity.as_mut())
            }
        }
        None
    }
}
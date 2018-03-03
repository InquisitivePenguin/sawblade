use game::graphicalcontext::GraphicalContext;
use game::scene::Scene;
use game::gameobject::GameObject;
extern crate rand;
use self::rand::StdRng;
use self::rand::Rng;
use self::GameLoopState::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;
use std::ops::Deref;
use graphics::pixel::Pixel;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use game::world::World;
use game::scene::SceneBuilder;

enum GameLoopState {
    Continue,
    Exit,
    Pause
}

pub struct GameBuilder {
    g_context_settings: Option<((u32,u32), String)>,
    def_scene_name: Option<String>,
    scene_funcs: HashMap<String, fn() -> Scene>
}

impl GameBuilder {
    // Modifier functions go here

    pub fn with_blank_scene(mut self) -> GameBuilder {
        self.with_scene(SceneBuilder::blank)
    }

    pub fn with_scene(mut self, scene: fn() -> Scene) -> GameBuilder {
        self.scene_funcs.insert(scene().get_name(), scene);
        self
    }

    pub fn default_scene(mut self, name: &str) -> GameBuilder {
        self.def_scene_name = Some(name.to_string());
        self
    }
}

pub struct Game {
    world: World,
    gcontext: GraphicalContext,
    default_scene_name: Option<String>
}

impl Game {
    pub fn new(title:String, res: (u32,u32)) -> GameBuilder {
        GameBuilder {
            g_context_settings: None,
            def_scene_name: None,
            scene_funcs: HashMap::new(),
        }
    }

    pub fn start(mut self) {
        {
            let name = (&self).default_scene_name.clone().expect("No default scene was provided");
            self.world.set_scene(name);
        }
        loop {
            (&mut self).game_cycle();
        }
    }



    fn game_cycle(&mut self) -> GameLoopState {
        //let mut scene = self.state.deref().borrow_mut().get_current_scene().borrow_mut();
        //scene.on_tick(Input::new());
        self.gcontext.wind.draw(&vec![]);
        Continue
    }
    /*
        Testing functions go here
    */
    #[cfg(test)]
    pub fn test_window_open(&self) -> bool {
        self.gcontext.wind.is_open()
    }
}
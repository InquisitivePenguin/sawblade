use game::graphicalcontext::GraphicalContext;
use game::scene::Scene;
use game::gameobject::GameObject;
extern crate rand;
extern crate sdl2;
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
use game::event::Event;
use game::event::Event::*;
use self::sdl2::Sdl;
use self::sdl2::EventPump;

enum GameLoopState {
    Continue,
    Exit,
    Pause
}

pub struct GameBuilder {
    g_context_settings: Option<((u32,u32), String)>,
    def_scene_name: Option<String>,
    scene_funcs: HashMap<String, fn() -> Scene>,
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

    pub fn build(mut self) -> Game {
        let context = sdl2::init().unwrap();
        let graphicalcontext = GraphicalContext::new(&context, "Sawblade".to_string(), (1280,1080));
        let event_pump = (&context).event_pump().unwrap();
        Game {
            sdl_context: context,
            world: World {
                scene_creators: self.scene_funcs
            },
            gcontext: graphicalcontext,
            default_scene_name: None,
            event_pump: event_pump,
        }
    }
}

pub struct Game {
    sdl_context: Sdl,
    world: World,
    gcontext: GraphicalContext,
    default_scene_name: Option<String>,
    event_pump: EventPump
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

    fn collect_events(&mut self) -> Vec<Event> {
        let mut collector = vec![];
        // Always add tick
        collector.push(Tick);
        collector
    }
    /*
        Testing functions go here
    */
    #[cfg(test)]
    pub fn test_window_open(&self) -> bool {
        self.gcontext.wind.is_open()
    }
}
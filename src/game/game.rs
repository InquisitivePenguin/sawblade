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
use game::input::KeyboardKey::*;
use self::sdl2::event::Event::*;


#[derive(PartialEq)]
enum GameLoopState {
    Continue,
    Exit,
    Pause
}

pub struct GameBuilder {
    g_context_settings: Option<((u32,u32), String)>,
    def_scene_name: Option<String>,
    scene_funcs: HashMap<String, fn() -> Scene>,
    window_settings: (String, (u32,u32))
}

impl GameBuilder {
    // Modifier functions go here

    pub fn with_blank_scene(mut self) -> GameBuilder {
        self.def_scene_name = Some(SceneBuilder::blank().get_name());
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
        let graphicalcontext = GraphicalContext::new(&context, self.window_settings.0, self.window_settings.1);
        let event_pump = (&context).event_pump().unwrap();
        let def_scene_name = self.def_scene_name.clone();
        Game {
            sdl_context: context,
            world: World::new(self.scene_funcs, self.def_scene_name.expect("No default scene was provided"), (500,500)), // TODO: Replace with customizable dimensions
            gcontext: graphicalcontext,
            default_scene_name: def_scene_name,
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
            window_settings: (title,res),
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
            let state = (&mut self).game_cycle();
            if state == Exit {
                break
            }
        }
    }

    fn game_cycle(&mut self) -> GameLoopState {
        let collected_events = self.collect_events();
        let collected_events_duplicate = collected_events.clone();
        for event in collected_events {
            if event == Key(Escape) || event == Close {
                return Exit;
            }
        }
        let textures = self.world.run_events(collected_events_duplicate);
        self.gcontext.draw_textures(textures);
        Continue
    }

    fn collect_events(&mut self) -> Vec<Event> {
        let mut collector = vec![];
        collector.append(&mut self.event_pump_retrieve());
        collector
    }
    fn event_pump_retrieve(&mut self) -> Vec<Event> {
        let mut collector = vec![];
        for event in self.event_pump.poll_iter() {
            collector.push(match event {
                Quit { .. } => Close,
                _ => Tick
            });
        }
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
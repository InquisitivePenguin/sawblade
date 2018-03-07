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

/// The helper class for generating a `Game`. Usually created by a call to `Game::new`.
///
/// # Examples
/// ```
/// use sawblade::game::game::Game;
///
/// // Creates a GameBuilder instance
/// let game_builder = Game::new("GameBuilder test".to_string(), (100,100));
/// ```
pub struct GameBuilder {
    g_context_settings: Option<((u32,u32), String)>,
    def_scene_name: Option<String>,
    scene_funcs: HashMap<String, fn() -> Scene>,
    window_settings: (String, (u32,u32))
}

impl GameBuilder {
    // Modifier functions go here
    /// This is a modifier function that makes the resulting `Game` use a blank scene as a default scene.
    /// To avoid unexpected problems, do not call this function twice on the same `GameBuilder` and do not
    /// define any other scenes for this `GameBuilder`.
    ///
    /// # Examples
    /// ```
    /// use sawblade::game::game::Game;
    ///
    /// let game_builder = Game::new("GameBuilder test".to_string(), (100,100)).with_blank_scene();
    /// ```
    pub fn with_blank_scene(mut self) -> GameBuilder {
        self.def_scene_name = Some(SceneBuilder::blank().get_name());
        self.with_scene(SceneBuilder::blank)
    }

    /// This is a modifier function that defines a scene-creation function for the resulting `Game`.
    /// To avoid problems, don't call this multiple times with the same input.
    /// # Examples
    /// ```
    /// use sawblade::game::game::Game;
    /// use sawblade::game::scene::Scene;
    /// use sawblade::game::scene::SceneBuilder;
    ///
    /// fn my_custom_scene_function() -> Scene {
    ///   SceneBuilder::new("My Scene".to_string()).build()
    /// }
    ///
    /// let game_builder = Game::new("GameBuilder test".to_string(), (100,100)).with_scene(my_custom_scene_function);
    /// ```
    pub fn with_scene(mut self, scene: fn() -> Scene) -> GameBuilder {
        self.scene_funcs.insert(scene().get_name(), scene);
        self
    }

    /// This is a modifier function that sets the default scene for the resulting `Game`. This _must_ be used when building
    /// a game, or a runtime error will be thrown saying that no default scene was specified.
    ///
    /// The name of the scene is what you pass in to this function. If you don't know what the scene name is,
    /// it's the string you initialize your SceneBuilder with in your scene generation function.
    ///
    /// Make sure that you also pass the scene definition in via `with_scene()`.
    /// # Examples
    /// ```
    /// use sawblade::game::game::Game;
    /// use sawblade::game::scene::Scene;
    /// use sawblade::game::scene::SceneBuilder;
    ///
    /// fn my_custom_scene_function() -> Scene {
    ///   SceneBuilder::new("My Scene".to_string()).build()
    /// }
    ///
    /// let game_builder = Game::new("GameBuilder test".to_string(), (100,100)).with_scene(my_custom_scene_function)
    /// .default_scene("My Scene".to_string());
    /// ```
    pub fn default_scene(mut self, name: String) -> GameBuilder {
        self.def_scene_name = Some(name);
        self
    }

    /// This function generates a `Game` from a `GameBuilder`. Keep in mind that this will also initialize the window along with it's canvas.
    /// Therefore it's recommended to call `Game::start()` directly after creating it.
    ///
    /// # Examples
    /// ```
    /// use sawblade::game::game::Game;
    /// use sawblade::game::scene::Scene;
    /// use sawblade::game::scene::SceneBuilder;
    ///
    /// fn my_custom_scene_function() -> Scene {
    ///   SceneBuilder::new("My Scene".to_string()).build()
    /// }
    ///
    /// let game = Game::new("GameBuilder test".to_string(), (100,100)).with_scene(my_custom_scene_function)
    /// .default_scene("My Scene".to_string()).build();
    /// ```

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

/// Central entry point for any Sawblade game.
/// This stores all the components nessecary to run the game, including the graphical layer, the world state, and the
/// event handler.
///
/// This also abstracts many internal components of the system. The event handling, central game loop,
/// FPS regulation, and other boilerplate things you would probably write as a part of a game engine
/// are abstracted away from you. Though not currently implemented, it will become possible to override these
/// things for more customization.

pub struct Game {
    sdl_context: Sdl,
    world: World,
    gcontext: GraphicalContext,
    default_scene_name: Option<String>,
    event_pump: EventPump
}

impl Game {
    /// This function generates a `GameBuilder` given some settings for the windows.
    /// Note that this does not actually create a `Game`, but a GameBuilder object.
    /// There isn't a direct constructor for `Game`.
    ///
    /// # Examples
    /// ```
    /// use sawblade::game::game::Game;
    /// let game_builder = Game::new("My game".to_string(), (1000,1000));
    /// let game = game_builder.with_blank_scene().build();
    /// ```
    pub fn new(title:String, res: (u32,u32)) -> GameBuilder {
        GameBuilder {
            window_settings: (title,res),
            g_context_settings: None,
            def_scene_name: None,
            scene_funcs: HashMap::new(),
        }
    }
    /// This function starts the game. It should probably be called directly after building the `Game` object.

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
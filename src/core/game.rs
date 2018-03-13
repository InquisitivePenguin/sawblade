use graphics::graphicalcontext::GraphicalContext;
extern crate sdl2;
use self::GameLoopState::*;
use core::world::World;
use core::event::Event;
use core::event::Event::*;
use self::sdl2::Sdl;
use self::sdl2::EventPump;
use core::input::KeyboardKey::*;
use self::sdl2::event::Event::*;
use std::collections::HashMap;
use core::fps::FPSRegulator;


#[derive(PartialEq)]
enum GameLoopState {
    Continue,
    Exit
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
    window_settings: (String, (u32,u32)),
    world_fn: Option<fn() -> Box<World>>
}

impl GameBuilder {
    // Modifier functions go here

    /// This function sets the world

    pub fn with_world(mut self, world_fn: fn() -> Box<World>) -> GameBuilder {
        self.world_fn = Some(world_fn);
        self
    }

    /// This function generates a `Game` from a `GameBuilder`. Keep in mind that this will also initialize the window along with it's canvas.
    /// Therefore it's recommended to call `Game::start()` directly after creating it.
    ///
    /// # Examples
    /// ```
    /// use sawblade::game::game::Game;
    /// use sawblade::game::world::World;
    /// use sawblade::game::event::Event;
    ///
    /// struct MyWorld {}
    ///
    /// impl World for MyWorld {
    ///   fn event_loop(events: Vec<Event>) -> Vec<FinalTexture> {
    ///     vec![]
    ///   }
    /// }
    ///
    /// fn build_world() -> Box<World> {
    ///   Box::new(MyWorld {})
    /// }
    ///
    /// let game = Game::new("GameBuilder test".to_string(), (100,100)).with_world(build_world)
    /// .build();
    /// ```

    pub fn build(self) -> Game {
        let context = sdl2::init().unwrap();
        let graphicalcontext = GraphicalContext::new(&context, self.window_settings.0, self.window_settings.1);
        let event_pump = (&context).event_pump().unwrap();
        Game {
            sdl_context: context,
            world: self.world_fn.expect("No world generation function was passed to the engine")(),
            gcontext: graphicalcontext,
            event_pump,
            fps_reg: FPSRegulator::new(60)
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
    world: Box<World>,
    gcontext: GraphicalContext,
    event_pump: EventPump,
    fps_reg: FPSRegulator
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
            world_fn: None
        }
    }
    /// This function starts the game. It should probably be called directly after building the `Game` object.

    pub fn start(mut self) {
        self.world.init();
        loop {
            let state = (&mut self).game_cycle();
            if state == Exit {
                break
            }
        }
    }

    fn game_cycle(&mut self) -> GameLoopState {
        self.fps_reg.start();
        let collected_events = self.collect_events();
        let collected_events_duplicate = collected_events.clone();
        for event in collected_events {
            if event == Key(Escape) || event == Close {
                return Exit;
            }
        }
        let textures = self.world.as_mut().event_loop(collected_events_duplicate);
        self.gcontext.draw_textures(textures);
        self.fps_reg.wait();
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

// Macros go here
#[macro_export]
macro_rules! sawblade_run_world {
    ($world: ident, $title: expr, $res: expr) => {
        {
            use sawblade::core::game::Game;
            Game::new($title.to_string(), $res).with_world($world).build().start();
        }
    };
}
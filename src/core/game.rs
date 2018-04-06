use graphics::graphicalcontext::GraphicalContext;
extern crate sdl2;
use self::sdl2::Sdl;
use self::sdl2::EventPump;
use core::input::KeyboardKey::*;
use self::sdl2::event::Event::*;
use core::fps::FPSRegulator;
use core::input::KeyboardKey;
use core::application::Application;
use core::input::Input;


#[derive(PartialEq)]
pub enum GameStatus {
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
    app_fn: Option<fn() -> Box<Application>>
}

impl GameBuilder {
    // Modifier functions go here

    /// This function sets the application

    pub fn with_app(mut self, app_fn: fn() -> Box<Application>) -> GameBuilder {
        self.app_fn = Some(app_fn);
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
            app_layer: self.app_fn.expect("No world generation function was passed to the engine")(),
            gcontext: graphicalcontext,
            event_pump,
            fps_reg: FPSRegulator::new(60),
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
    app_layer: Box<Application>,
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
            app_fn: None
        }
    }
    /// This function starts the game. It should probably be called directly after building the `Game` object.

    pub fn start(mut self) {
        self.app_layer.init();
        loop {
            self.fps_reg.start();
            let state = self.app_layer.game_loop(&mut self.gcontext, Input {});
            if state == GameStatus::Exit {
                break
            }
            self.fps_reg.wait();
        }
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
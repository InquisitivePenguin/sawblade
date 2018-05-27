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

pub struct WindowSettings {
    pub resolution: (u32, u32),
    pub fullscreen: bool,
    pub title: String
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
    app_fn: Option<fn() -> Box<Application>>,
    window_settings: Option<WindowSettings>,
    has_window: bool
}

impl GameBuilder {
    // Modifier functions go here

    /// This function sets the application for the Game. The function supplied will be activated on the start of the game
    /// to generate the global Application.

    pub fn with_app(mut self, app_fn: fn() -> Box<Application>) -> GameBuilder {
        self.app_fn = Some(app_fn);
        self
    }

    /// This function sets the settings for the window (for the game).

    pub fn with_window_settings(mut self, window_settings: WindowSettings) -> GameBuilder {
        self.window_settings = Some(window_settings);
        self
    }

    /// This function sets this application as having no graphical context. This is useful for game servers or applications
    /// that have no window.

    pub fn with_no_window(mut self) -> GameBuilder {
        self.has_window = false;
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
        let graphicalcontext = GraphicalContext::new(&context, self.window_settings, self.has_window);
        let event_pump = (&context).event_pump().unwrap();
        Game {
            sdl_context: context,
            app_layer: self.app_fn.expect("No application generation function was passed to the engine")(),
            gcontext: graphicalcontext,
            event_pump,
            fps_reg: FPSRegulator::new(60),
        }
    }
}

/// Primary entry point for all Sawblade-powered games.
/// The main purpose of the Game class is to abstract a lot of the FPS regulation,
/// graphics management, and other stuff nobody wants to write, leaving you to write
/// your code!

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
    /// ```
    pub fn new() -> GameBuilder {
        GameBuilder {
            window_settings: None,
            app_fn: None,
            has_window: true
        }
    }
    /// This function initiates the Application and begins the game loop.

    pub fn start(mut self) {
        self.app_layer.init();
        loop {
            self.fps_reg.start();
            let state = self.app_layer.game_loop(&mut self.gcontext, self.event_pump.poll_iter().into());
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
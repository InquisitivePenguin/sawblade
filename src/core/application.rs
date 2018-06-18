use core::game::GameStatus;
use graphics::graphicalcontext::GraphicalContext;
use core::input::Input;

/// The Application is the main launching point for the game. Unlike what other Application classes typically represent
/// in other game engines, this handles considerably less: FPS regulation, resource management, and other boilerplate is
/// handled internally by the Game object. This was done to seperate the game logic from the utility logic.
///
/// Primarily, the Application in a Sawblade game is the dictator of what occurs each frame. This is the level at which rendering is done,
/// though it is recommended to pass the GraphicalContext reference to dedicated Renderer instances; one advantage of this is that
/// default Renderers that Sawblade uses, such as the UIRenderer, allow for easy linking with default Sawblade traits defined on the
/// State.
///
/// The Application, idiomatically, should _not_ handle game logic, unless the game is small enough that it does not warrant subsystems.
/// The Application should be in charge of sending the State to the subsystems it controls and have them perform their logical operations on
/// the State.
pub trait Application {
    fn init(&mut self);
    fn game_loop(&mut self, graphical_context: &mut GraphicalContext, input: Input) -> GameStatus;
}
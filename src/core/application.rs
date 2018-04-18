use core::game::GameStatus;
use graphics::graphicalcontext::GraphicalContext;
use core::input::Input;

/// This is where you will probably do most of your low-level coding, and also where you call your Systems
/// and activate Renderers.
pub trait Application {
    fn init(&mut self);
    fn game_loop(&mut self, graphical_context: &mut GraphicalContext, input: Input) -> GameStatus;
}
use core::game::GameStatus;
use graphics::graphicalcontext::GraphicalContext;
use core::input::Input;

pub trait Application {
    fn init(&mut self);
    fn game_loop(&mut self, graphical_context: &mut GraphicalContext, input: Input) -> GameStatus;
}
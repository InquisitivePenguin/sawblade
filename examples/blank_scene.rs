extern crate sawblade;
use self::sawblade::core::game::*;
use self::sawblade::core::application::Application;
use self::sawblade::graphics::graphicalcontext::GraphicalContext;
use self::sawblade::core::input::Input;

struct GameApp;

impl Application for GameApp {
    fn init(&mut self) {}
    fn game_loop(&mut self, gc: &mut GraphicalContext, input: Input) -> GameStatus {
        gc.refresh();
        if input.should_quit() {
            GameStatus::Exit
        } else {
            GameStatus::Continue
        }
    }
}

fn make_app() -> Box<Application> {
    Box::new(GameApp {})
}

fn main() {
    let game = Game::new("Blank Scene".to_string(), (500,500)).with_app(make_app).build();
    game.start();
}
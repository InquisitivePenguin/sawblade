extern crate sawblade;
use self::sawblade::core::game::*;
use self::sawblade::core::application::Application;
use self::sawblade::graphics::graphicalcontext::GraphicalContext;
use self::sawblade::core::input::Input;

struct GameApp;

impl Application for GameApp {
    fn init(&mut self) {}
    fn game_loop(&mut self, gc: &mut GraphicalContext, input: Input) -> GameStatus {
        gc.update();
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
    let game = Game::new().with_app(make_app).with_window_settings(WindowSettings {
        resolution: (1000, 1000),
        title: "Blank Scene".to_string(),
        fullscreen: false
    }).build();
    game.start();
}
extern crate sawblade;
use self::sawblade::core::game::Game;

fn main() {
    let game = Game::new("Blank Scene".to_string(), (500,500)).with_blank_world().build();
    game.start();
}
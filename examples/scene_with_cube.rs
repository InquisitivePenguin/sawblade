extern crate sawblade;
use self::sawblade::game::game::Game;
use self::sawblade::game::scene::*;
use self::sawblade::game::world::WorldState;

fn custom_game_init(scene: &mut Scene, world: &WorldState) {
    println!("Hello!");
}

fn game_scene() -> Scene {
    SceneBuilder::new("Default Scene".to_string()).override_init(custom_game_init).build()
}

fn main() {
    let game = Game::new("Blank Scene".to_string(), (500,500))
        .with_scene(game_scene)
        .default_scene("Default Scene")
        .build();
    game.start();
}
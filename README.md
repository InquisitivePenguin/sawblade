# Sawblade

Sawblade is a game engine written in Rust, with a focus on speed, safety, and modularity. This library exposes both high-level and
low-level functionality, allowing you to customize what gets rendered down to the pixel, while also providing many useful
helper functions to reduce boilerplate code.

## Examples
Here is a simple platformer game:

`main.rs`:
```rust
extern crate sawblade;
use self::sawblade::Game;
use self::sawblade::GameState;
mod jumper;
mod block;
mod setup;

fn main() {
  let s_game = Game::new().
  with_render_window("My platformer game", 600, 800)
  .with_custom_event_loop( |state: &GameState| {
    let input = Game::collect_input();
    state.trigger_events(input,GameState::EventTriggerProcess:Hierarchial);
  } ).with_custom_init(setup::init())
  .with_components(!vec[block::Block,jumper::Jumper]);
  s_game.load();
  s_game.start();
}
```

## Credits
Author: Jackson Lewis

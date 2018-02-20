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
use self::sawblade::GraphicalSettings;
use self::sawblade::Scene;
use self::sawblade::GameObject;
mod jumper;
mod block;
mod setup;
use self::block::Block;

struct MyScene {
    jumper: self::jumper::JumperCharacter,
    blocks: Vec<Block>,
}

impl Scene for MyScene {
    fn new() -> MyScene {
        MyScene {
            jumper: self::jumper::JumperCharacter::new(),
            blocks: vec![]
        }
    }
    fn on_init(&mut self) {
        // Load jumper character
        self.jumper.set_pos(0,0);
        // Place a few blocks to jump on
        self.blocks.push(Block::spawn((50,50)));
        self.blocks.push(Block::spawn((100,100)));
        self.blocks.push(Block::spawn((150,150)));                
    }
    fn get_scene_entities(&mut self) -> Vec<&mut GameObject> {
        entities = vec![];
        entities.push(&mut self.jumper);
        for block in &mut self.blocks {
            entities.push(block);
        }
        entities
    }
    fn get_scene_name(&self) -> &str {
        "MyScene"
    }
}

fn main() {
  let window_settings = GraphicalSettings::new()
  .fullscreen(true);
  let game_settings = GameSettings::new()
  .default_scene("MyScene");
  let s_game = Game::new()
  .has_window_with_settings("My platformer game", 600, 800)
  .with_scene(MyScene::new())
  .with_window_settings(window_settings)
  .with_game_settings(game_settings);
  s_game.load_required_resources();
  s_game.start();
}
```

## Installing
Clone this repository, then run `cargo build` to build the library.

## Running tests
To verify everything is working correctly, run `cargo test -- --test-threads=1`. It's
important that you run this with the option `--test-threads=1`, as some tests involve creating
SDL windows, and only one context can exist at once, which is problematic
since Rust runs tests concurrently by default.

## Versions
Sawblade will initially have a handful of core features initially, and
will increase in size over time.

Here is what we hope to implement on each release:

`0.1`: windows, pixel buffers, textures, sprites, event triggers, customizable basic loops

`0.2`: more customizable looping, basic physics, more advanced external sprite-based manipulation, user input event handling templates

`0.3`: graphical shaders, basic AI abstraction, higher-level manipulation functions

`0.4`: extensive window manipulation, particle effects

`0.5`: Multi-threading support

`0.6`: Box2D integration

More to be announced.

Versioning is as follows:

`<game engine version>.<major release>.<minor release>.<beta release>.<bleeding-edge release>`

The major release and minor releases are for official changes, and are updated in the `Cargo.toml` file. Updates
to these releases are made in the master branch.

Beta releases and bleeding-edge releases are in their own branches. The bleeding-edge release
number increments each commit. Bleeding-edge releases are not guarenteed to pass tests,
or even work. The bleeding-edge branch is merged into the beta branch once a new feature has been
implemented and tested. Testers should use the beta branch for tests.
## Credits
Author: Jackson Lewis

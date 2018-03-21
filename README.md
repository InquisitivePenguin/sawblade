# Sawblade

Sawblade is a game engine written in Rust, with a focus on speed, safety, and modularity. It allows you to build your game up from the
lowest level, but still abstract and simplify what would otherwise be boilerplate with a plethora of helper functions and classes. It allows this by supplying many macros for ease of use and abstraction, as well as many helper classes.

Here's a demo of it in action:
```rust
#[macro_use]
extern crate sawblade;
use sawblade::core::{Game, World, Entity, Event, KeyboardKey};
use sawblade::controllers::physics::{VelocityController, VelocityScheduler};
use sawblade::graphics::{FinalTexture};
use sawblade::utils::collision::{is_point_overlapping_with_rect};
sawblade_entity_struct!(Cube {
  controllers {
    velocity_controller: VelocityController
  }
  fields {
    color: (u32, u32, u32)
  }
  world = GameWorld
});
  
sawblade_world_struct!(GameWorld {
  coordinate_system has bounds (500,500)
});
impl Entity for Cube {
  fn event(&mut self, world: GameWorld, event: Event) {
    pass_event!(self.velocity_controller, event);
    match event {
      Tick => {
        
      }
    }
  }
}
impl GameWorld {
  sawblade_world_handle_event!(
    Tick => {
      
    }
  );
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

`0.1`: windows, textures, file loading, entities, basic controllers, worlds, coordinate systems, basic macros

`0.2`: basic physics, more macros, schedulers, better graphical manipulation

`0.3`: graphical shaders, basic AI controllers, sound handling, tile-maps, basic UI helpers

`0.4`: extensive macro collections, particle effects, better UI support

`0.5`: Multi-threading support

`0.6`: Box2D integration with Physics components

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

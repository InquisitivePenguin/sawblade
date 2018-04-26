# Sawblade

Sawblade is a game engine written in Rust, with a focus on speed, safety, and modularity. It operates on a seperated systems principle, which means that ideally, changing any code in a class or system doesn't affect other systems unless you want it to.

Sawblade has a hierarchal structure of `Application -> System`, where the `Application` is used to render things to the screen,
play sounds, and modify inputs, and the `System` is used to modify a specific part of the state each tick. The game's state is
the primary 'store' of the current game data.

Sawblade does not restrict you to any design method. If you want, you can directly return textures to the screen! But it provides built-in support for Lua scripting, ECS design, shaders, powerful AI control, and networking.

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

`0.1`: Core components (Game, Application, System), ECS support, shape rendering, primitive texture rendering, window event handling

`0.2`: Tile-maps, sounds, UI helper classes/framework

`0.3`: Graphical shaders, AI controllers

`0.4`: Better UI support, more macros

`0.5`: Networking, multi-threading

`0.6`: Box2D physics (maybe?)

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

# Sawblade

Sawblade is a game engine written in Rust, with a focus on speed, safety, and modularity. This library exposes both high-level and
low-level functionality, allowing you to customize what gets rendered down to the pixel, while also providing many useful
helper functions to reduce boilerplate code.

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

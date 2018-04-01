Sawblade revolves around an ECS (entity-component-system) model and a immutable
world state. On each 'tick' of the engine, Sawblade captures all input and then
relays those inputs to the World class it controls. The World is a 
container for everything that happens inside the game. This is _not_ the world
state. A World typically contains a script engine, custom properties,
and, of course, the world state.

The WorldState is a custom-defined class that implements the ToLua trait
so that it can be turned into a Lua object. It is passed to the scripting engine
on each tick.

The scripting engine is the 'core' of Sawblade. It manages all of the scripts,
systems, and objects in the scene.

Entities are custom classes that have a script attached to them. That
script executes each tick and the entity then pulls the data from that script
to update itself. The script has certain global variables set,
like the world state and the events/messages.

Systems are unique, global objects that can operate on multiple entities before
or after the entities update. Usually they act on entities that carry
certain components. Entities define their components via the `get_components`
function.

Systems can also send 'messages'. A message is a user-defined event
that has some significance to an entity. Messages are helpful when
communicating with objects in an abstract way.

Systems can also recieve messages! Entities send messages by setting
the variable `_messages` to a list of string pairs. They will be given
to the System on the next tick.


Let's see an example of this.

In a basic game, a rectangle with changing colors is rendered to 500x500 screen. Let's take a look at the process that the engine uses.

First, lets consider an entity named HelloText:
```rust
pub struct SomeRect {
  color: u32
}
impl SomeRect {
  pub fn new() -> HelloText {
    HelloText { color: 0 }
  }
}
impl ScriptableEntity for SomeRect {
  fn get_identifier(&self) -> String {
    "HelloText".to_string()
  }
  fn inject(&self) -> Vec<(String, String)> {
    vec![
      ("color", String::from(self.color))
    ]
  }
  fn get_entity_script(&self) -> Script {
    Script::from_file("hellotext.lua").unwrap()
  }
  fn apply_state(&mut self, state: ScriptState) {
    self.color = state.get::<u32>("color").unwrap();
  }
}
impl Entity for SomeRect {
  fn get_id(&self) -> u64 { 0 }
  fn render(&mut self) -> Vec<FinalTexture> {
    vec![
      FinalTexture::from_rect((50,50), (50,50), Color::from(self.color))
    ]
  }
}
```

With a world named GameWorld:
```rust
pub struct GameWorld {
  engine: ScriptEngine
}

impl World for GameWorld {
  fn init(&mut self) {
    engine.load_single(SomeRect::new())
  }
  fn event_loop(&mut self, events: Vec<Event>) -> Vec<FinalTexture> {
    self.engine.run(Message::from_events(events));
    self.engine.render()
  }
}
```

With a Lua script named `somerect.lua`:
```lua
color += 1
```

This code is what would compose this game.
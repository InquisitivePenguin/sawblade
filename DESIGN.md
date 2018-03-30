In Sawblade, there is exactly one World, which contains all of what happens inside the game. The World is populated with Entities, which are objects
with logic and event handling added to them. On each iteration, Sawblade handles input fetching and then pipes those inputs to the World. The World creates custom events and
triggers based on the input, and then activates the scripting engine. This runs the scripts for each Entity current in the scene, and also runs
for each of there respective components. It then applies the output to each of the Entities. Then the world collects the textures to be rendered to the screen,
and then passes that back to Sawblade, which then renders everything.

Components are entirely Lua-based.

Inside Lua, each entity class has an event handling script and also
data linked with the Rust entity. The components of this entity are stored
in here as well.

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
  fn get_entity_script(&self) -> Script {
    Script::from_file("hellotext.lua").unwrap()
  }
  fn apply_state(&mut self, state: EntityState) {
    self.color = state.extract::<u32>("color").unwrap();
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
  engine: ScriptEngine,
  message_handler: MessageHandler
}

impl World for GameWorld {
  fn init(&mut self) {
    engine.load_single(SomeRect::new())
  }
  fn event_loop(&mut self, events: Vec<Event>) -> Vec<FinalTexture> {
    self.message_handler.increment_tick();
    let messages = EventManager::generate_from_events(events).and_custom_script(
    |events: Vec<Event>| {
      message_handler.this_tick()
    }
    );
    self.engine.run(messages, &mut self.message_handler);
    self.engine.render()
  }
}
```

With a Lua script named `somerect.lua`:
```lua
SomeRect = {}

function SomeRect:new()
  local t = setmetatable({}, { __index: SomeRect})
  t.color = 0
  return t
end

function SomeRect:on_msg(message)
  if message == Message("Tick")
    self.color = self.color + 1
end
```

This code is what would compose this game.
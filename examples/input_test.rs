#[macro_use]
extern crate sawblade;
use sawblade::core::world::World;
use sawblade::core::gameobject::Entity;
use sawblade::core::input::KeyboardKey;
use sawblade::core::event::Event;
use sawblade::graphics::texture::FinalTexture;
use sawblade::core::utils::Message;
use sawblade::graphics::utils::render;

const KEY_BOUNDS: (u32,u32) = (50,50);

struct GameWorld {
    keys: Vec<Key>
}

#[derive(Clone, Debug)]
struct Key {
    coordinates: (u32,u32),
    key: KeyboardKey,
    pressed: bool
}

const KEY_TESTS: [KeyboardKey; 5] = [
    KeyboardKey::Up,
    KeyboardKey::Down,
    KeyboardKey::Left,
    KeyboardKey::Right,
    KeyboardKey::Q
];

impl Key {
    pub fn new(coordinates: (u32,u32), key: KeyboardKey) -> Key {
        Key {
            coordinates,
            key,
            pressed: false
        }
    }
}

impl Entity for Key {
    fn get_id(&self) -> u64 {
        0
    }
    fn handle_msg(&mut self, msg: Message) {
        match msg {
            Message::Input(Event::Tick) => {
                self.pressed = false;
            },
            Message::Input(Event::Key(some_key)) => {
                if some_key == self.key.clone() {
                    self.pressed = true;
                }
            },
            _ => ()
        };
    }
    fn render(&mut self) -> Vec<FinalTexture> {
        if self.pressed {
            vec! [
              FinalTexture::make_rect((50,50), self.coordinates)
            ]
        }
        else {
            vec![]
        }
    }
}

impl GameWorld {
    pub fn new() -> GameWorld {
        GameWorld {
            keys: vec![]
        }
    }
}

impl World for GameWorld {
    fn init(&mut self) {
        println!("init");
        // Create keys
        let mut loc_x = 0;
        let mut loc_y = 0;
        for key in KEY_TESTS.iter() {
            let new_key = Key::new((loc_x, loc_y), key.clone());
            loc_x += KEY_BOUNDS.0;
            loc_y += KEY_BOUNDS.1;
            self.keys.push(new_key);
        }
    }
    fn event_loop(&mut self, events: Vec<Event>) -> Vec<FinalTexture> {
        for mut key_entity in &mut self.keys {
            for event in events.clone() {
                key_entity.handle_msg(Message::from(event));
            }
        }
        render(&mut self.keys)
    }
}

fn make_world() -> Box<World> {
    Box::new(GameWorld::new())
}

fn main() {
    sawblade_run_world!(make_world, "Input Test", (1000,1000))
}
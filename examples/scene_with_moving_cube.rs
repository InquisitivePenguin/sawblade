#[macro_use]
extern crate sawblade;
use self::sawblade::core::game::Game;
use self::sawblade::core::event::Event;
use self::sawblade::graphics::texture::FinalTexture;
use self::sawblade::core::world::World;
use self::sawblade::core::gameobject::Entity;
use self::sawblade::core::coordinate_system::CoordinateSystem;
use self::sawblade::core::input::KeyboardKey;
use self::sawblade::core::utils::Message;
use self::sawblade::graphics::utils::render;

fn build_world() -> Box<World> {
    Box::new(
        GameWorld::new()
    )
}

const velocity_decay: f32 = 0.95;

fn decay_velocity(x: f32, y: f32) -> (f32, f32) {
    let new_x = x * velocity_decay;
    let new_y = y * velocity_decay;
    (new_x, new_y)
}

struct GameWorld {
    cubes: Vec<Cube>,
    pub coordinate_system: CoordinateSystem
}

impl GameWorld {
    pub fn new() -> GameWorld {
        GameWorld {
            cubes: vec![],
            coordinate_system: CoordinateSystem::new(500, 500, 1)
        }
    }
}

impl World for GameWorld {
    fn init(&mut self) {
        self.cubes.push(Cube::new((100,100)));
    }
    fn event_loop(&mut self, events: Vec<Event>) -> Vec<FinalTexture> {
        for mut cube in &mut self.cubes {
            for event in &events {
                cube.handle_msg(Message::from(event.clone()));
            }
        }
        render(self.cubes.as_mut())
    }
}

struct Cube {
    coordinates: (u32,u32),
    movement_amount_x: f32,
    movement_amount_y: f32
}

impl Cube {
    fn new(coordinates: (u32,u32)) -> Cube {
        Cube {
            coordinates,
            movement_amount_x: 1.0,
            movement_amount_y: 1.0,
        }
    }
}

impl Entity for Cube {
    fn get_id(&self) -> u64 {
        0
    }
    fn handle_msg(&mut self, msg: Message) {
        match msg {
            Message::Input(Event::Tick) => {
                let new_velocities = decay_velocity(self.movement_amount_x, self.movement_amount_y);
                self.movement_amount_x = new_velocities.0;
                self.movement_amount_y = new_velocities.1;
                let move_x = self.movement_amount_x as i32;
                let move_y = self.movement_amount_y as i32;
                self.coordinates = CoordinateSystem::move_to(self.coordinates, move_x, move_y);
            }
            Message::Input(Event::Key(KeyboardKey::Left)) | Message::Input(Event::Key(KeyboardKey::A)) => {
                self.movement_amount_x -= 1.0;
            }
            Message::Input(Event::Key(KeyboardKey::Right))| Message::Input(Event::Key(KeyboardKey::D)) => {
                self.movement_amount_x += 1.0;
            }
            Message::Input(Event::Key(KeyboardKey::Up)) | Message::Input(Event::Key(KeyboardKey::W)) => {
                self.movement_amount_y -= 1.0;
            },
            Message::Input(Event::Key(KeyboardKey::Down)) | Message::Input(Event::Key(KeyboardKey::S)) => {
                self.movement_amount_y += 1.0;
            }
            _ => {}
        }
    }
    fn render(&mut self) -> Vec<FinalTexture> {
            vec![
                FinalTexture::make_rect((50,50), self.coordinates)
            ]
    }
}

fn main() {
    sawblade_run_world!(build_world, "Scene with Cube", (500,500));
}
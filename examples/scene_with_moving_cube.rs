#[macro_use]
extern crate sawblade;
use self::sawblade::core::game::Game;
use self::sawblade::core::event::Event;
use self::sawblade::graphics::texture::FinalTexture;
use self::sawblade::core::world::World;
use self::sawblade::core::entity::Entity;
use self::sawblade::components::*;
use self::sawblade::core::coordinate_system::CoordinateSystem;
use self::sawblade::core::input::KeyboardKey;

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
        for i in vec!(0, 100, 200) {
            for j in vec!(50, 150, 250) {
                self.cubes.push(Cube::spawn((i,j), 1));
            }
        }
    }
    fn event_loop(&mut self, events: Vec<Event>) -> Vec<FinalTexture> {
        unsafe {
            let ptr = self as *mut GameWorld;
            for cube in &mut (*ptr).cubes {
                for event in &events {
                    cube.event(&mut (*ptr), event.clone());
                }
            }
            render(self.cubes.as_mut())
        }
    }
}

struct Cube {
    coordinates: (u32,u32),
    id: u64,
    movement_amount_x: f32,
    movement_amount_y: f32
}

impl Entity for Cube {
    type World = GameWorld;
    fn spawn(coordinates: (u32,u32), id: u64 ) -> Cube {
        Cube {
            coordinates,
            id,
            movement_amount_x: 1.0,
            movement_amount_y: 1.0,
        }
    }
    fn get_id(&self) -> u64 {
        self.id
    }
    fn event(&mut self, world: &mut GameWorld, event: Event) {
        match event {
            Event::Tick => {
                let new_velocities = decay_velocity(self.movement_amount_x, self.movement_amount_y);
                self.movement_amount_x = new_velocities.0;
                self.movement_amount_y = new_velocities.1;
                let move_x = self.movement_amount_x as i32;
                let move_y = self.movement_amount_y as i32;
                self.coordinates = world.coordinate_system.move_to(self.coordinates, move_x, move_y);
            }
            Event::Key(KeyboardKey::Left) => {
                self.movement_amount_x -= 1.0;
            }
            Event::Key(KeyboardKey::Right) => {
                self.movement_amount_x += 1.0;
            }
            Event::Key(KeyboardKey::Up) => {
                self.movement_amount_y -= 1.0;
            },
            Event::Key(KeyboardKey::Down) => {
                self.movement_amount_y += 1.0;
            }
            _ => {}
        }
    }
}

impl Renderable for Cube {
    fn render(&mut self) -> Option<Vec<FinalTexture>> {
        Some(
            vec![
              FinalTexture::make_rect((50,50), self.coordinates)
            ]
        )
    }
}

fn main() {
    sawblade_run_world!(build_world, "Scene with Cube", (500,500));
}
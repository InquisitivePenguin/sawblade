#[macro_use]
extern crate sawblade;
use self::sawblade::core::game::Game;
use self::sawblade::core::event::Event;
use self::sawblade::graphics::texture::FinalTexture;
use self::sawblade::core::world::World;
use self::sawblade::core::entity::Entity;
use self::sawblade::controllers::*;
use self::sawblade::core::coordinate_system::CoordinateSystem;
use self::sawblade::core::input::KeyboardKey;

fn build_world() -> Box<World> {
    Box::new(
        GameWorld::new()
    )
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
    moving_left: bool,
    movement_amount: i32
}

impl Entity for Cube {
    type World = GameWorld;
    fn spawn(coordinates: (u32,u32), id: u64 ) -> Cube {
        Cube {
            coordinates,
            id,
            movement_amount: 3,
            moving_left: false
        }
    }
    fn get_id(&self) -> u64 {
        self.id
    }
    fn event(&mut self, world: &mut GameWorld, event: Event) {
        match event {
            Event::Tick => {
                if self.moving_left {
                    self.coordinates = world.coordinate_system.move_to(self.coordinates, -self.movement_amount, 0);
                }
                else {
                    self.coordinates = world.coordinate_system.move_to(self.coordinates, self.movement_amount, 0);
                }
            }
            Event::Key(KeyboardKey::Left) => {
                self.moving_left = true;
            }
            Event::Key(KeyboardKey::Right) => {
                self.moving_left = false;
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
#[macro_use]
extern crate sawblade;
use self::sawblade::core::game::Game;
use self::sawblade::core::event::Event;
use self::sawblade::graphics::texture::FinalTexture;
use self::sawblade::core::world::World;
use self::sawblade::core::entity::Entity;
use self::sawblade::controllers::Controller;
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
            let mut collected = vec![];
            for cube in &mut (*ptr).cubes {
                match cube.render() {
                    Some(texture) => collected.push(texture),
                    None => ()
                }
            }
            collected
        }
    }
}

struct MoveController {
    obj_id: u64,
    pub movement_amount: i32,
    between_counter: u32
}

impl Controller for MoveController {
    type World = GameWorld;
    fn bind(id: u64) -> MoveController {
        MoveController {
            obj_id: id,
            movement_amount: 3,
            between_counter: 0
        }
    }
    fn tick(&mut self, world: *mut GameWorld) {
        /*
        unsafe {
            (*scene).get_entity_by_id(self.obj_id).unwrap().recv(scene, "move".to_string());
        }
        */
    }
}

struct Cube {
    coordinates: (u32,u32),
    id: u64,
    movement_controller: MoveController,
    moving_left: bool
}

impl Entity for Cube {
    type World = GameWorld;
    fn spawn(coordinates: (u32,u32), id: u64 ) -> Cube {
        Cube {
            coordinates,
            id,
            movement_controller: MoveController::bind(id),
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
                    self.coordinates = world.coordinate_system.move_to(self.coordinates, -self.movement_controller.movement_amount, 0);
                }
                else {
                    self.coordinates = world.coordinate_system.move_to(self.coordinates, self.movement_controller.movement_amount, 0);
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
    fn render(&mut self) -> Option<FinalTexture> {
        Some(FinalTexture::make_rect((50,50), self.coordinates))
    }
}

fn main() {
    sawblade_run_world!(build_world, "Scene with Cube", (500,500));
}
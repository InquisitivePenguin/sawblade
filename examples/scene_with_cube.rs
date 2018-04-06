extern crate sawblade;
use self::sawblade::core::gameobject::Entity;
use self::sawblade::controllers::*;
use self::sawblade::graphics::texture::FinalTexture;
use self::sawblade::core::world::World;
use self::sawblade::core::event::Event;
use self::sawblade::core::game::Game;

struct BasicWorld {
    cube: Vec<Cube>
}

impl BasicWorld {
    pub fn new() -> BasicWorld {
        BasicWorld {
            cube: vec![]
        }
    }
}

impl World for BasicWorld {
    fn init(&mut self) {
        self.cube = vec![Cube::spawn((200,200), 1)];
    }
    fn event_loop(&mut self, events: Vec<Event>) -> Vec<FinalTexture> {
        render(&mut self.cube)
    }
}

fn world_create() -> Box<World> {
    Box::new(BasicWorld::new())
}

struct Cube {
    coordinates: (u32,u32),
    id: u64
}

impl Entity for Cube {
    type World = BasicWorld;
    fn spawn(coordinates: (u32,u32), id: u64 ) -> Cube {
        Cube {
            coordinates,
            id
        }
    }
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl Renderable for Cube {
    fn render(&mut self) -> Option<Vec<FinalTexture>> {
        Some(vec![FinalTexture::make_rect((100,100), self.coordinates)])
    }
}

fn main() {
    let game = Game::new("Scene with Cube".to_string(), (500,500))
        .with_world(world_create)
        .build();
    game.start();
}
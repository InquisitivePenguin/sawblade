extern crate sawblade;
use self::sawblade::game::game::Game;
use self::sawblade::game::scene::*;
use self::sawblade::game::world::WorldState;
use self::sawblade::game::gameobject::GameObject;
use self::sawblade::game::gamecontroller::GameController;
use self::sawblade::game::msg::Msg;
use self::sawblade::game::msg::Msg::*;
use self::sawblade::graphics::texture::FinalTexture;


fn custom_game_init(scene: &mut Scene, world: &WorldState) {
    scene.spawn::<Cube>((50,50));
    scene.spawn::<Cube>((400,400));
    scene.spawn::<Cube>((200,200));
}

fn game_scene() -> Scene {
    SceneBuilder::new("Default Scene".to_string()).override_init(custom_game_init).build()
}

struct RotationController {
    current_rotation: u32,
    rotation_inc: u32,
    id: u64
}

impl GameController for RotationController {
    fn bind(id: u64) -> RotationController {
        RotationController {
            current_rotation: 0,
            rotation_inc: 0,
            id
        }
    }

    fn recv(&mut self, scene: *mut Scene, message: Msg) {
        match message {
            Custom("inc_rotate", params) => {
                let rotate_amount: u32 = params[0].parse().unwrap();
                self.rotation_inc += rotate_amount;
            }
            _ => ()
        }
    }
}

struct MoveController {
    obj_id: u64,
    pub movement_amount: i32,
    between_counter: u32
}

impl GameController for MoveController {
    fn bind(id: u64) -> MoveController {
        MoveController {
            obj_id: id,
            movement_amount: 1,
            between_counter: 0
        }
    }
    fn tick(&mut self, scene: *mut Scene) {
        unsafe {
            (*scene).get_entity_by_id(self.obj_id).unwrap().recv(scene, "move".to_string());
        }
    }
}

struct Cube {
    coordinates: (u32,u32),
    id: u64,
    rotation_controller: RotationController,
    movement_controller: MoveController
}

impl GameObject for Cube {
    fn spawn(coordinates: (u32,u32), id: u64 ) -> Cube {
        Cube {
            coordinates,
            id,
            rotation_controller: RotationController::bind(id),
            movement_controller: MoveController::bind(id)
        }
    }
    fn get_id(&self) -> u64 {
        self.id
    }
    fn recv(&mut self, scene: *mut Scene, trigger: String) {
        match trigger.as_str() {
            "move" => {
                self.coordinates.0 += self.movement_controller.movement_amount as u32;
            },
            _ => {}
        }
    }
    fn render(&mut self) -> Option<FinalTexture> {
        Some(FinalTexture::make_rect((50,50), self.coordinates))
    }
    fn tick(&mut self, scene: *mut Scene) {
        self.movement_controller.tick(scene);
    }
}

fn main() {
    let game = Game::new("Scene with Cube".to_string(), (500,500))
        .with_scene(game_scene)
        .default_scene("Default Scene".to_string())
        .build();
    game.start();
}
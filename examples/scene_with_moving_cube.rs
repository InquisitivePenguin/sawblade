extern crate sawblade;
use self::sawblade::core::game::Game;
use self::sawblade::core::application::Application;
use self::sawblade::core::input::KeyboardKey;
use self::sawblade::core::input::Input;
use self::sawblade::graphics::graphicalcontext::GraphicalContext;
use self::sawblade::core::game::GameStatus;
use self::sawblade::core::system::System;
use self::sawblade::graphics::texture::*;

fn make_app() -> Box<Application> {
    Box::new(
        GameApplication::new()
    )
}

const velocity_decay: f32 = 0.95;

fn decay_velocity(x: f32, y: f32) -> (f32, f32) {
    let new_x = x * velocity_decay;
    let new_y = y * velocity_decay;
    (new_x, new_y)
}

struct GameState {
    cube: Cube
}

struct GameApplication {
    state: GameState,
    cube_system: CubeMovementSystem,
}

struct CubeMovementSystem;

impl GameApplication {
    pub fn new() -> GameApplication {
        GameApplication {
            state: GameState {
                cube: Cube::new((100,100))
            },
            cube_system: CubeMovementSystem {}
        }
    }
}

impl Application for GameApplication {
    fn init(&mut self) {
    }
    fn game_loop(&mut self, graphical_context: &mut GraphicalContext, input: Input) -> GameStatus {
        graphical_context.clear();
        self.cube_system.update(&input, &mut self.state);
        // Render
        graphical_context.draw_textures(vec![
            Texture {
                components: vec![
                    TextureComponent::BasicShape(
                        Shape::Rectangle(
                            (50,50).into()
                        ),
                        (255,255,255)
                    )
                ],
                relative_origin: self.state.cube.coordinates.into(),
            }
        ]);
        graphical_context.update();
        if input.should_quit() {
            GameStatus::Exit
        } else {
            GameStatus::Continue
        }
    }
}

impl System for CubeMovementSystem {
    type GameState = GameState;
    fn update(&mut self, input: &Input, state: &mut GameState) {
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

fn main() {
    Game::new("".to_string(), (1000,1000)).with_app(make_app).build().start();
}
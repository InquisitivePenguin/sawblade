extern crate sawblade;
use self::sawblade::core::game::*;
use self::sawblade::core::application::Application;
use self::sawblade::core::input::KeyboardKey;
use self::sawblade::core::input::Input;
use self::sawblade::graphics::graphicalcontext::GraphicalContext;
use self::sawblade::core::game::GameStatus;
use self::sawblade::core::system::System;
use self::sawblade::graphics::texture::*;
use self::sawblade::core::math::Vector;

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
                            Vector::from_generic((50 as u32,50 as u32))
                        ),
                        (255,255,255)
                    )
                ],
                relative_origin: Vector::from_generic(self.state.cube.coordinates),
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
        if input.key_is_down(KeyboardKey::Up) {
            state.cube.movement_amount_y += -1 as f32;
        }
        if input.key_is_down(KeyboardKey::Down) {
            state.cube.movement_amount_y += 1 as f32;
        }
        if input.key_is_down(KeyboardKey::Left) {
            state.cube.movement_amount_x += -1 as f32;
        }
        if input.key_is_down(KeyboardKey::Right) {
            state.cube.movement_amount_x += 1 as f32;
        }

        state.cube.movement_amount_x *= 0.99;
        state.cube.movement_amount_y *= 0.99;

        if state.cube.movement_amount_x > 0.0 {
            state.cube.coordinates.0 += state.cube.movement_amount_x as u32;
        }
        if state.cube.movement_amount_x < 0.0 && state.cube.coordinates.0 as f32 + state.cube.movement_amount_x  > 0.0 {
            state.cube.coordinates.0 -= -state.cube.movement_amount_x as u32;
        }
        if state.cube.movement_amount_y > 0.0 {
            state.cube.coordinates.1 += state.cube.movement_amount_y as u32;
        }
        if state.cube.movement_amount_y < 0.0  && state.cube.coordinates.1 as f32 + state.cube.movement_amount_y  > 0.0 {
            state.cube.coordinates.1 -= -state.cube.movement_amount_y as u32;
        }
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
    Game::new().with_app(make_app).with_window_settings(WindowSettings {
        resolution: (1000, 1000),
        fullscreen: false,
        title: "Moving Cube Example".to_string()
    }).build().start();
}
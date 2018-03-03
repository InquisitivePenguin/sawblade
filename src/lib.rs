// Top level file for Sawblade Library
// This should only include tests and linked modules
pub mod game;
pub mod graphics;
pub mod ui;

#[cfg(test)]
mod test {
    use self::super::*;
    use graphics::texture::FinalTexture;
    use graphics::pixel::Pixel;
    use game::input::Input;
    use std::cell::Ref;
    use game::game::Game;
    struct TestScene {

    }
    /*
    impl game::scene::Scene for TestScene {

    }
    */
    struct TestEntity {
        coordinates: (u32,u32),
        id: u64
    }
    impl game::gameobject::GameObject for TestEntity {
        fn spawn(&mut self, coords: (u32,u32), id: u64) -> bool {
            self.id = id;
            self.coordinates = coords;
            true
        }
        fn on_tick(&mut self, state: &GameState, input: &Input) -> game::gameobject::GameObjectMsg {
            game::gameobject::GameObjectMsg::NoMsg
        }
        fn get_coordinates(&self) -> (u32,u32) {
            self.coordinates
        }
        fn get_bounding_box(&self) -> (u32,u32) {(0,0)}
        fn get_id(&self) -> u64 {self.id}
        fn render(&mut self) -> Option<FinalTexture> {None}
    }
    // game module
        // Game object
        #[test]
        fn game_creates_window_successfully() {
            let g = game::game::Game::new("Testing".to_string(),(50,50));
            assert!(g.test_window_open())
        }
        #[test]
        fn game_can_load_scene() {

        }
        #[test]
        fn game_can_load_game_object() {

        }
        #[test]
        // This function runs one tick of the game and checks if the character in the designated scene is on the right coordinate
        fn can_run_frame_successfully() {

        }
        #[test]
        fn game_can_run() {
            let g = Game::new("Sawblade Test".to_string(), (600,800)).with_blank_scene();
            g.start();
        }

        // GameState object
        #[test]
        fn can_do_all_configuration_functions() {

        }

    // graphics module
        // Layer object
        #[test]
        fn can_fill_layers() {

        }

        // Texture object
        #[test]
        fn can_load_texture_from_file() {

        }
        #[test]
        fn can_successfully_render_texture() {

        }

        // Pixel object
        // Nothing yet...

        // Window object
        #[test]
        fn can_output_to_screen() {

        }

    // ui module
        // Nothing yet...
}
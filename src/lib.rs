// Top level file for Sawblade Library
// This should only include tests and linked modules
pub mod game;
pub mod graphics;
pub mod ui;

#[cfg(test)]
mod test {
    use self::super::*;
    struct TestScene {

    }
    impl game::scene::Scene for TestScene {

    }
    // game module
        // Game object
        #[test]
        fn game_creates_window_successfully() {
            let g = game::game::Game::new("Testing".to_string(),(50,50));
            assert!(g.test_window_open())
        }
        #[test]
        fn game_successfully_populates_buffer() {
            let mut g = game::game::Game::new("Testing".to_string(),(50,50));
            g.test_rand_pixel_populate((50,50));
            assert_ne!(g.get_state().get_raw_screen_buffer()[49][49].rbg.0, 0)
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
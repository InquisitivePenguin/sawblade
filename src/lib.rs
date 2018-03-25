#![feature(duration_from_micros)]
// Top level file for Sawblade Library
// This should only include tests and linked modules
pub mod core;
pub mod graphics;
pub mod components;

#[cfg(test)]
mod test {
    use self::super::*;
    use graphics::texture::FinalTexture;
    use graphics::pixel::Pixel;
    use std::cell::Ref;
    use game::game::Game;
    // game module
        // Game object
        #[test]
        fn game_creates_window_successfully() {
            let g = game::game::Game::new("Testing".to_string(),(50,50)).with_blank_scene().build();
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
            let g = Game::new("Sawblade Test".to_string(), (600,800)).with_blank_scene().build();
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
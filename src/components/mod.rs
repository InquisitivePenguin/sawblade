use std::marker::Sized;
use core::world;
use graphics::texture::FinalTexture;
/// This is a very simple component that allows textures and basic shapes to be rendered to the screen
pub trait Renderable {
    fn render(&mut self) -> Option<Vec<FinalTexture>>;
}

// Renderable related functions

pub fn render<T: Renderable>(entities: &mut Vec<T>) -> Vec<FinalTexture> {
    let mut textures = vec![];
    for entity in entities {
        match entity.render() {
            Some(mut final_textures) => textures.append(&mut final_textures),
            None => ()
        };
    };
    textures
}
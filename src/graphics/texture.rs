extern crate sdl2;
use self::sdl2::render::Texture;
use self::sdl2::render::TextureCreator;
use std::cell::RefCell;
// FinalTexture is a single image that represents a raw texture to be rendered to the screen
pub struct FinalTexture {
    scene_coords: (u64,u64),
    texture: Texture
}

impl FinalTexture {
    pub fn new(texture: Texture, coords: (u64,u64)) -> FinalTexture {
        FinalTexture {
            scene_coords: coords,
            texture: texture
        }
    }
    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }
}
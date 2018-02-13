extern crate sdl2;
use self::sdl2::render::Texture;
use self::sdl2::render::TextureCreator;
use std::cell::RefCell;
// FinalTexture is a single image that represents a raw texture to be rendered to the screen
pub struct FinalTexture<'a> {
    scene_coords: (u64,u64),
    texture: Texture<'a>
}

impl<'a> FinalTexture<'a> {
    pub fn new(tc: &TextureCreator<Texture<'a>>, coords: (u64,u64)) {

    }
}

struct GameObjectTexture<'a>{
    textures: Vec<Texture<'a>>,
    texture_files: Vec<String>
}
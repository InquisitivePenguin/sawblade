use graphics::pixel::Pixel;
extern crate sdl2;
use self::sdl2::render::TextureCreator;
struct Layer {
    layer_num: u32,
    raw_pixels: Vec<Vec<Pixel>>
}

impl Layer {
    pub fn new(num: u32) -> Layer {
        Layer {
            layer_num: num,
            raw_pixels: vec![]
        }
    }

    //pub fn from_list(Vec<>)
}
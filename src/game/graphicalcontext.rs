extern crate sdl2;
use self::sdl2::render::TextureCreator;
use self::sdl2::surface::Surface;
use self::sdl2::render::Texture;
use graphics::window::Window;

pub struct GraphicalContext {
    wind: Window,
    texture_generator: TextureCreator<self::sdl2::video::WindowContext>,
    texture_storage: Vec<Texture>
}

impl GraphicalContext {
    pub fn new(window_title: String, resolution: (u32,u32)) -> GraphicalContext {
        let mut n_wind = Window::new(resolution,window_title);
        let mut n_texture_pool = n_wind.get_canvas().texture_creator();
        GraphicalContext {
            wind: n_wind,
            texture_generator: n_texture_pool,
            texture_storage: vec![]
        }
    }

    pub fn append_texture(&mut self, mem_texture: Surface) {
        let n_texture = self.texture_generator.create_texture_from_surface(mem_texture).unwrap();
        self.texture_storage.push(n_texture);
    }
}


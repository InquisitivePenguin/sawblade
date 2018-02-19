extern crate sdl2;

use self::sdl2::render::TextureCreator;
use self::sdl2::surface::Surface;
use self::sdl2::render::Texture;
use graphics::window::Window;
use std::collections::HashMap;

pub struct GraphicalContext {
    wind: Window,
    texture_generator: TextureCreator<self::sdl2::video::WindowContext>,
    texture_storage: HashMap<String, Texture>
}

impl GraphicalContext {
    pub fn new(window_title: String, resolution: (u32, u32)) -> GraphicalContext {
        let mut n_wind = Window::new(resolution, window_title);
        let mut n_texture_pool = n_wind.get_canvas().texture_creator();
        GraphicalContext {
            wind: n_wind,
            texture_generator: n_texture_pool,
            texture_storage: HashMap::new()
        }
    }

    pub fn load_texture_from_surface(&mut self, texture_name: String, mem_texture: Surface) {
        let n_texture = self.texture_generator.create_texture_from_surface(mem_texture).unwrap();
        self.texture_storage.insert(texture_name, n_texture);
    }

    pub fn borrow_texture(&mut self, texture_name: String) -> Option<&Texture> {
        if self.texture_storage.contains_key(texture_name.as_str()) {
            Some(&self.texture_storage[texture_name.as_str()])
        } else {
            None
        }
    }

    pub fn release(mut self) {
        unsafe {
            for texture in self.texture_storage {
                texture.1.destroy();
            }
        }
        self.wind.close();
    }
}
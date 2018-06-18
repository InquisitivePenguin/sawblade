extern crate sdl2;
use self::sdl2::render::TextureCreator;
use self::sdl2::surface::Surface;
use self::sdl2::render;
use self::sdl2::Sdl;
use graphics::window::Window;
use std::collections::HashMap;
use graphics::texture::*;
use core::game::WindowSettings;
use std::borrow::Borrow;

pub struct GraphicalContext {
    window: Option<Window>,
    texture_generator: Option<TextureCreator<self::sdl2::video::WindowContext> >,
    texture_storage: HashMap<String, render::Texture>
}

impl GraphicalContext {
    pub fn new(context: &Sdl, settings: Option<WindowSettings>, should_have_window: bool) -> GraphicalContext {
        if !should_have_window {
            return GraphicalContext {
                window: None,
                texture_generator: None,
                texture_storage: HashMap::new()
            };
        }
        let settings = settings.unwrap();
        let mut n_wind = Window::new(context, settings.resolution, settings.title);
        let n_texture_pool = n_wind.get_canvas().texture_creator();
        GraphicalContext {
            window: Some(n_wind),
            texture_generator: Some(n_texture_pool),
            texture_storage: HashMap::new()
        }
    }

    pub fn load_texture_from_surface(&mut self, texture_name: String, mem_texture: Surface) {
        if let None = self.texture_generator {
            return;
        }
        let n_texture = self.texture_generator.as_ref().unwrap().create_texture_from_surface(mem_texture).unwrap();
        self.texture_storage.insert(texture_name, n_texture);
    }

    pub fn borrow_texture(&mut self, texture_name: String) -> Option<&render::Texture> {
        if self.texture_storage.contains_key(texture_name.as_str()) {
            Some(&self.texture_storage[texture_name.as_str()])
        } else {
            None
        }
    }

    pub fn release(mut self) {
        if let None = self.window {
            return;
        }
        unsafe {
            for texture in self.texture_storage {
                texture.1.destroy();
            }
        }
        self.window.unwrap().close();
    }

    pub fn draw_textures(&mut self, textures: Vec<Texture>) {
        if let None = self.window {
            return;
        }
        for texture in textures {
            self.window.as_mut().unwrap().draw_texture(texture);
        }
    }
    pub fn update(&mut self) {
        if let None = self.texture_generator {
            return;
        }
        self.window.as_mut().unwrap().update();
    }
    pub fn clear(&mut self) {
        if let None = self.texture_generator {
            return;
        }
        self.window.as_mut().unwrap().fill_blank();
    }
}
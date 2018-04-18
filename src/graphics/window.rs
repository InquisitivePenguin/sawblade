extern crate sdl2;
use self::sdl2::rect::Point;
use self::sdl2::pixels::Color;
use self::sdl2::Sdl;
use self::sdl2::video::FullscreenType;
use graphics::texture::*;
use self::sdl2::rect::Rect;

pub struct Window {
    title: String,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Window {
    pub fn new(context: &Sdl, res: (u32,u32), title: String) -> Window {
        let vid = (context).video().unwrap();
        let canvas = vid.window(title.as_str(),
            res.0,
            res.1
        ).position_centered().build().unwrap()
            .into_canvas().build().unwrap();
        Window {
            title,
            canvas
        }

    }

    pub fn resize(&mut self, size: (u32,u32)) {
        let mut window = self.canvas.window_mut();
        window.set_size(size.0, size.1).unwrap();
    }

    pub fn fullscreen(&mut self, fullscreen_on: bool) {
        let mut window = self.canvas.window_mut();
        window.set_fullscreen(if fullscreen_on {
            FullscreenType::True
        } else {
            FullscreenType::Desktop
        }).unwrap();
    }

    pub fn is_open(&self) -> bool {
        true
    }

    pub fn draw_texture(&mut self, texture: Texture) {
        //let texture_creator = self.canvas.texture_creator();
        if texture.components.len() == 0 {
            return;
        }
        match texture.components[0] {
            TextureComponent::BasicShape(Shape::Rectangle(bounds),color) => {
                self.canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
                let rect = Rect::new(texture.relative_origin.x_i32(),
                                     texture.relative_origin.y_i32(),
                                     bounds.x_u32(),
                                     bounds.y_u32());
                self.canvas.fill_rect(rect).expect("Could not draw rectangle to SDL2 canvas");
            },
            /*
            SawbladeTexture::Circle(radius) => {

            }
            SawbladeTexture::FromFile(file_name) => {

            }
            */
            _ => ()
        };
        self.canvas.set_draw_color(Color::RGB(255,255,255));
    }

    pub fn update(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0,0,0));
        self.canvas.present();
    }

    pub fn fill_blank(&mut self) {
        self.canvas.clear();
    }

    pub fn close(&mut self) {
        self.canvas.clear();
    }

    pub fn get_canvas(&mut self) -> &mut sdl2::render::Canvas<sdl2::video::Window> {
        &mut self.canvas
    }
}

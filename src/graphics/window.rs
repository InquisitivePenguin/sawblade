extern crate sdl2;
use self::sdl2::rect::Point;
use self::sdl2::pixels::Color;
use self::sdl2::Sdl;
use graphics::pixel::Pixel;

pub struct Window {
    title: String,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    context: Sdl
}

impl Window {
    pub fn new(res: (u32,u32), title: String) -> Window {
        let context = sdl2::init().unwrap();
        let vid = (&context).video().unwrap();
        let canvas = vid.window(title.as_str(),
            res.0,
            res.1
        ).position_centered().build().unwrap()
            .into_canvas().build().unwrap();
        Window {
            title: title,
            canvas: canvas,
            context: context
        }
    }
    pub fn is_open(&self) -> bool {
        true
    }

    pub fn draw(&mut self, pixels: &Vec<Vec<Pixel>>) {
        self.fill_blank();
        for i in pixels {
            for q in i {
                self.canvas.set_draw_color(Color::RGB(q.rbg.0,q.rbg.1,q.rbg.2));
                self.canvas.draw_point::<Point>(q.to_sdl_point());
            }
        }
        self.canvas.present();
    }

    fn fill_blank(&mut self) {
        self.canvas.clear();
    }

    pub fn close(&mut self) {
        self.context.sdldrop();
    }

    pub fn get_canvas(&mut self) -> &mut sdl2::render::Canvas<sdl2::video::Window> {
        &mut self.canvas
    }
}

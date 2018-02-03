extern crate sdl2;
use self::sdl2::rect::Point;

#[derive(Clone,Copy,Debug)]
pub struct Pixel {
    pub rbg: (u8,u8,u8),
    pos: (u32,u32)
}

impl Pixel {
    // Function 'Pixel::new'
    // What it does: creates a new Pixel object (with a default color of black and positioning at (0,0))
    pub fn new() -> Pixel {
        Pixel {
            rbg: (0,0,0),
            pos: (0,0)
        }
    }

    pub fn with_position(mut self, pos: (u32,u32)) -> Pixel {
        self.set_pos(pos.0,pos.1);
        self
    }

    pub fn with_color(mut self, color: (u8,u8,u8)) -> Pixel {
        self.set_color(color.0,color.1,color.2);
        self
    }
    // Function 'Pixel::set_pos'
    // What it does: sets the position of a Pixel object
    pub fn set_pos(&mut self, x: u32, y: u32) {
        self.pos = (x,y);
    }
    // Function 'Pixel::set_color'
    // What it does: sets the color values of a Pixel object
    pub fn set_color(&mut self, r: u8,b: u8, g: u8) {
        self.rbg = (r,b,g)
    }

    pub fn to_sdl_point(&self) -> Point {
        Point::new(self.pos.0 as i32,self.pos.1 as i32)
    }
}
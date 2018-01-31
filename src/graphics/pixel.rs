pub struct Pixel {
    rbg: (u8,u8,u8),
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
}
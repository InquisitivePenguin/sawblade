use game::state::GameState;
use graphics::window::Window;
use graphics::pixel::Pixel;
extern crate rand;
use self::rand::StdRng;
use self::rand::Rng;

pub struct Game {
    state: GameState,
    out: Window
}

impl Game {
    pub fn new(title:String, res: (u32,u32)) -> Game {
        Game {
            out: Window::new(res,title),
            state: GameState::new()
        }
    }
    /*
        Testing functions go here
    */
    // What this does: setup a random pixel array for rendering in testing
    pub fn test_rand_pixel_populate (&mut self, res: (u32,u32)) {
        let mut pixels = vec![];
        let mut rng = StdRng::new().unwrap();
        for i in 0..res.0 {
            let mut tmp :Vec<Pixel> = vec![];
            for q in 0..res.1 {
                let mut randarr = [0,0,0];
                rng.fill_bytes(&mut randarr);
                tmp.push(Pixel::new().with_position((i,q)).with_color((randarr[0],randarr[1],randarr[2])));
            }
            pixels.push(tmp);
        }
        self.state.set_screen_buffer(pixels);
    }

    pub fn test_render (&mut self) {
        self.out.draw(self.state.get_screen_buffer());
    }
}
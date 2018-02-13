use game::state::GameState;
use graphics::window::Window;
use graphics::pixel::Pixel;
use game::scene::Scene;
extern crate rand;
use self::rand::StdRng;
use self::rand::Rng;
extern crate sdl2;
use self::sdl2::render::TextureCreator;
use self::sdl2::render::Texture;

pub struct Game {
    state: GameState,
    out: Window,
    scenes: Vec<Box<Scene>>
}

impl Game {
    pub fn new(title:String, res: (u32,u32)) -> Game {
        Game {
            out: Window::new(res,title),
            state: GameState::new(),
            scenes: vec![]
        }
    }

    pub fn get_state(& mut self) -> &mut GameState {
        & mut (self.state)
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
        self.state.set_raw_screen_buffer(pixels);
    }

    pub fn test_render (&mut self) {
        self.out.draw(self.state.get_raw_screen_buffer());
    }

    pub fn test_window_open (&self) -> bool {
        self.out.is_open()
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        println!("Hello");
        self.out.close();
    }
}
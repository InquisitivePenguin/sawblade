use game::state::GameState;
use game::graphicalcontext::GraphicalContext;
use game::scene::Scene;
use graphics::pixel::Pixel;
extern crate rand;
use self::rand::StdRng;
use self::rand::Rng;
use self::GameLoopState::*;

enum GameLoopState {
    Inactive,
    CollectingResources,
    ProcessingSceneTriggers,
    Rendering,
    Pausing
}
pub struct Game {
    state: GameState,
    gcontext: GraphicalContext,
    //scenes: Vec<Box<Scene>>,
    game_loop_state: GameLoopState,
    // gameresourcemanager: GameResourceManager
}

impl Game {
    pub fn new(title:String, res: (u32,u32)) -> Game {
        Game {
            gcontext: GraphicalContext::new(title,res),
            state: GameState::new(),
            //scenes: vec![],
            game_loop_state: Inactive
        }
    }

    pub fn get_state(& mut self) -> &mut GameState {
        & mut (self.state)
    }

    /*
        Testing functions go here
    */
    #[cfg(test)]
    // What this does: setup a random pixel array for rendering in testing
    pub fn test_rand_pixel_populate(&mut self, res: (u32, u32)) {
        let mut pixels = vec![];
        let mut rng = StdRng::new().unwrap();
        for i in 0..res.0 {
            let mut tmp: Vec<Pixel> = vec![];
            for q in 0..res.1 {
                let mut randarr = [0, 0, 0];
                rng.fill_bytes(&mut randarr);
                tmp.push(Pixel::new().with_position((i, q)).with_color((randarr[0], randarr[1], randarr[2])));
            }
            pixels.push(tmp);
        }
        self.state.set_raw_screen_buffer(pixels);
    }
    #[cfg(test)]
    pub fn test_render(&mut self) {
        self.out.draw(self.state.get_raw_screen_buffer());
    }
    #[cfg(test)]
    pub fn test_window_open(&self) -> bool {
        self.out.is_open()
    }
}
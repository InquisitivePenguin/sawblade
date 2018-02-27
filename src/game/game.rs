use game::state::GameState;
use game::graphicalcontext::GraphicalContext;
use game::scene::Scene;
use game::input::Input;
use game::gameobject::GameObject;
extern crate rand;
use self::rand::StdRng;
use self::rand::Rng;
use self::GameLoopState::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;
use std::ops::Deref;

enum GameLoopState {
    Continue,
    Exit,
    Pause
}
pub struct Game {
    state: Option<Rc<GameState> >,
    gcontext: GraphicalContext,
    scenes: HashMap<String, Rc<Box<Scene>>>,
    default_scene_name: Option<String>
    // gameresourcemanager: GameResourceManager
}

struct BlankScene {
    state: Weak<GameState>
}

impl Scene for BlankScene {
    fn new(state: Weak<GameState>) -> BlankScene {
        let mut new = BlankScene {
            state
        };
        new
    }
    fn get_scene_entities(&mut self) -> Option<Vec<Box<GameObject>>> {
        None
    }
    fn get_scene_name(&self) -> &str {
        "Blank Scene"
    }
    fn get_game_state(&self)  -> Rc<GameState> {
        self.state.upgrade().unwrap()
    }
}

impl Game {
    pub fn new(title:String, res: (u32,u32)) -> Game {
        Game {
            gcontext: GraphicalContext::new(title,res),
            state: None,
            scenes: HashMap::new(),
            default_scene_name: None
        }
    }

    // Modifier functions go here

    pub fn with_blank_scene(mut self) -> Game {
        let state_ref = self.state.clone();
        let mut blank_scene = BlankScene::new(Rc::downgrade(&state_ref.unwrap()));
        let blank_scene_name = blank_scene.get_scene_name().to_string();
        self = self.with_scene(Box::new(blank_scene));
        self.default_scene_name = Some(blank_scene_name);
        self
    }

    pub fn with_scene(mut self, scene: Box<Scene>) -> Game {
        self.scenes.insert(scene.get_scene_name().to_string(), Rc::new(scene));
        self
    }

    pub fn default_scene(mut self, name: &str) -> Game {
        self.default_scene_name = Some(name.to_string());
        self
    }

    pub fn start(mut self) {
        // Load default scene
        let def_scene = self.scenes.get(
            &self.default_scene_name.expect("No default scene was provided")
        ).unwrap();
        let mut new_state = GameState::new();
        new_state.set_current_scene(def_scene.clone());
        self.state = Some(Rc::new(new_state));
    }

    pub fn get_state(&mut self) -> Rc<GameState> {
        self.state.clone().unwrap().clone()
    }


    fn game_cycle(&mut self) -> GameLoopState {
        Continue
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
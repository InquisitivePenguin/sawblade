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
use graphics::pixel::Pixel;
use std::cell::RefCell;

enum GameLoopState {
    Continue,
    Exit,
    Pause
}
pub struct Game {
    state: Rc<RefCell<GameState>>,
    gcontext: GraphicalContext,
    scenes: HashMap<String, Rc<Box<Scene>>>,
    default_scene_name: Option<String>
    // gameresourcemanager: GameResourceManager
}

struct BlankScene {
    state: Rc<RefCell<GameState>>
}

impl Scene for BlankScene {
    fn new(state: Rc<RefCell<GameState>>) -> BlankScene {
        let new = BlankScene {
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
    fn get_game_state(&self)  -> Rc<RefCell<GameState>> {
        self.state.clone()
    }
}

impl Game {
    pub fn new(title:String, res: (u32,u32)) -> Game {
        Game {
            gcontext: GraphicalContext::new(title,res),
            state: Rc::new(RefCell::new(GameState::new())),
            scenes: HashMap::new(),
            default_scene_name: None
        }
    }

    // Modifier functions go here

    pub fn with_blank_scene(mut self) -> Game {
        let mut blank_scene = BlankScene::new(Rc::clone(&self.state));
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
        self.state.deref().borrow_mut().set_current_scene(Rc::clone(def_scene));
    }

    pub fn get_state(&mut self) -> Rc<RefCell<GameState>> {
        Rc::clone(&self.state)
    }


    fn game_cycle(&mut self) -> GameLoopState {
        Continue
    }
    /*
        Testing functions go here
    */
    #[cfg(test)]
    pub fn test_window_open(&self) -> bool {
        self.gcontext.wind.is_open()
    }
}
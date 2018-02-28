use game::gameobject::GameObject;
use graphics::texture::FinalTexture;
use graphics::pixel::Pixel;
use game::state::GameState;
use game::input::Input;
use std::marker::Sized;
use std::rc::Weak;
use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;

pub enum SceneMsg {
    Continue,
    ExitTo(String),
    Pause
}
pub trait Scene {
    fn new(state: Rc<RefCell<GameState>>) -> Self where Self : Sized;

    fn get_final_textures(&mut self) -> Option<Vec<FinalTexture>> {None}

    fn get_raw_pixels(&mut self) -> Option<Vec<Vec<Option<Pixel>>>> {None}

    fn get_scene_entities(&mut self) -> Option<Vec<Box<GameObject>>>;

    fn get_scene_name(&self) -> &str;

    fn on_init(&mut self) {}

    fn on_tick(&mut self, input: Input) -> SceneMsg {SceneMsg::Continue}

    fn on_tick_internal(&mut self, input: Input) {
        let gs_ref = self.get_game_state();
        for mut entity in self.get_scene_entities().unwrap() {
            let gs_ref = gs_ref.deref().borrow();
            let msg = entity.on_tick(gs_ref.deref(), &input);
        }
    }

    fn on_exit(&mut self) {}

    fn get_game_state(&self) -> Rc<RefCell<GameState>>;
}
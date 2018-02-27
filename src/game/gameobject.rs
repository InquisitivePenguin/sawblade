use game::state::GameState;
use game::input::Input;
use graphics::texture::FinalTexture;

pub enum GameObjectMsg {
    NoMsg,
    Msg(String)
}
pub trait GameObject {
    /*
    spawn should be called when creating a new GameObject-based entity. Don't spawn if function returns false
    */
    fn spawn(&mut self, coords: (u32,u32), id: u64) -> bool;
    /*
    on_tick should be called on the object for each game 'tick' in the scene
    */
    fn on_tick(&mut self, state: &GameState, input: &Input) -> GameObjectMsg;
    /*
    These are getter/setter functions for the vars that should be in the derived struct
    */
    fn get_coordinates(&self) -> (u32,u32);
    fn get_bounding_box(&self) -> (u32,u32);
    fn get_id(&self) -> u64;
    fn render(&mut self) -> FinalTexture;
}
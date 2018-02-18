use std::iter::Map;
pub trait GameObject {
    type SceneDelegate;

    /*
    spawn should be called when creating a new GameObject-based entity
    */
    fn spawn(coords: (u32,u32)) -> Self;
    /*
    on_tick should be called on the object for each game 'tick'
    */
    fn on_tick(&mut self) -> Self::SceneDelegate;
    /*
    on_directional_input is called after on_tick if directional input was identified by the Scene code
    */
    fn on_directional_input(&mut self) -> Self::SceneDelegate;
    /*
    on_trigger is called after on_directional_input to trigger any code caused by an Event
    */
    fn on_trigger(&mut self, method : String) -> Self::SceneDelegate;

    /*
    These are getter/setter functions for the vars that should be in the derived struct
    */
    fn get_coords(&self) -> (u32,u32);
    fn set_directional_input(&mut self, input: (u32,u32));
}
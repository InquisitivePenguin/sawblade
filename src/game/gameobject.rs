use std::iter::Map;
pub trait GameObject {
    type SceneDelegate;
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
}
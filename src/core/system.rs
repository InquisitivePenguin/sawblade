use core::input::Input;
pub trait System {
    type GameState;
    fn update(&mut self, input: &Input,state: &mut Self::GameState);
}

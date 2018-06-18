use core::input::Input;
/// A subsystem is where all of the exciting logic happens. Each system should (idiomatically) be modulated to handle a specific part of the
/// game, updating the state deterministically. Systems can also have a parent-child relationship with that System owning its own subsystems. Sawblade provides several systems for
/// use by default.
/// 
/// Note: This is _not_ the same thing as an ECS System object. That would be covered by the ECSSystem and the ECSDispatcher
/// in the `ecs` module.
pub trait System {
    type GameState;
    fn update(&mut self, input: &Input,state: &mut Self::GameState);
}

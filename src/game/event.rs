use game::input::KeyboardKey;
pub enum Event {
    Tick,
    Key(KeyboardKey),
    Custom(String),
    MouseMovement((i32,i32))
}
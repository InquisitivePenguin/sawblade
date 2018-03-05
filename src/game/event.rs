use game::input::KeyboardKey;
#[derive(PartialEq)]
pub enum Event {
    Tick,
    Key(KeyboardKey),
    Custom(String),
    MouseMovement((i32,i32)),
    Close
}
use game::input::KeyboardKey;
pub enum Event {
    Tick,
    Key(KeyboardKey)
}
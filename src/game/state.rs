use graphics::pixel::Pixel;
pub struct GameState {
    //cycle_state: CycleState, //TODO: Implement cycle state
    ticks: u64,
    is_outputting: bool,
    screen_buf: Vec<Vec<Pixel>>
}

impl GameState {
    // Function 'GameState::new'
    // What it does: creates a blank, fresh GameState
    pub fn new() -> GameState {
        GameState {
            ticks: 0,
            is_outputting: false,
            screen_buf : vec!()
        }
    }

    pub fn get_buf(&self) -> &Vec<Vec<Pixel>> { // Fetches buffer
        &self.screen_buf
    }
    //TODO: Add configuration input function
    //TODO: Add more updating functions
}
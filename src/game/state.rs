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

    pub fn get_screen_buffer(&mut self) -> &mut Vec<Vec<Pixel>> { // Fetches buffer
        &mut self.screen_buf
    }
    pub fn set_screen_buffer(&mut self, new_buffer: Vec<Vec<Pixel>>) {
        self.screen_buf = new_buffer;
    }
    //TODO: Add configuration input function
    //TODO: Add more updating functions
}
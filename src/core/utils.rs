/// Used externally for handling functions which may or may not succeed
#[must_use]
pub type GameResult<T> = Result<T, GameError>;

/// This is unfinished
pub enum GameError {
    Unexpected
    //TODO: Add more kinds of errors
}
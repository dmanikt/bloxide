use crate::player;

/// Determines the time step length in between advancements of each player.
pub const PLAYER_SPEED: f64 = 0.1;

/// A Game struct holds information related to the size of the game board,
/// the two players, and the status of the game.
pub struct Game {
    width: u32,  // Measured in "blocks"
    height: u32, // Measured in "blocks"

    player_one: player::Player,
    player_two: player::Player,

    is_game_over: bool,
    time_waited: f64, // Used when moving players forward
}

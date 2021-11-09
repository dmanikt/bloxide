use crate::player;
use piston_window::{Context, G2d, Key};

/// Determines the time step length in between advancements of each player.
pub const PLAYER_SPEED: f64 = 0.1;

/// A Game struct holds information related to the size of the game board,
/// the two players, and the status of the game.
///
/// Note on time_waited: This variable represents the amount of time waited, in seconds,
/// since the last time that both players were moved forward by one block.  This will
/// be incremented by a fixed amount each time the window updates, and then once it
/// passes a certain threshold, we will march each player forward one block and then
/// reset time_waited to 0.
pub struct Game {
    width: u32,  // Measured in "blocks"
    height: u32, // Measured in "blocks"

    player_one: player::Player,
    player_two: player::Player,

    is_game_over: bool,
    time_waited: f64, // Used when moving players forward
}

impl Game {
    /// Creates a new Game with the given width and height.  Initialize Players 1 and 2 for the
    /// game by calling the player_1 and player_2 methods in player.rs.
    pub fn new(width: u32, height: u32) -> Self {
        todo!()
    }

    /// Draws the game by first drawing both players, and then drawing a black border on the outer
    /// edge of the game window.
    pub fn draw(&self, con: Context, g: &mut G2d) {
        todo!()
    }

    /// Given an amount of time elapsed (this will be provided by the game window itself),
    /// updates the game to account for this passed time.  The game's time_waited field
    /// should be incremented by the amount of time elapsed.  If this causes time_waited to
    /// meet (or exceed) the constant `PLAYER_SPEED`, then both players should be moved
    /// forward and time_waited should be reset to 0.
    pub fn update(&mut self, time_elapsed: f64) {
        todo!()
    }

    /// Updates the game based on a key pressed by the user.  The WASD keys will change the
    /// direction of player 1 (do this by calling the update_direction method for player 1).
    /// The directional arrow keys will update the direction of player 2 (do this by calling the
    /// update_direction method for player 2).  The enter key will restart the game, but only
    /// if the game is currently over.
    pub fn key_pressed(&mut self, key: Key) {
        todo!()
    }

    /// Resets the state of the game to represent a brand new game by creating new
    /// players and resetting is_game_over and time_waited.
    pub fn restart(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_new() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_update() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_key_pressed() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_restart() {
        todo!()
    }
}

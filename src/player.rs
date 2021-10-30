use crate::graphics::Block;
use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

/// A simple enumerated type representing the four directions a player can move.
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns the direction opposite the current one.
    pub fn opposite_direction(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

/// A Player has a current moving direction and a "trail" which is a
/// LinkedList of the Blocks that it has moved over.
pub struct Player {
    moving_direction: Direction, /* the direction in which the player is currently moving */
    has_moved_in_direction: bool, /* true if the player has advanced forward in this direction
                                 since switching to it */
    trail: LinkedList<Block>, /* head of LL is the front of the player's trail */
    color: Color,
}

impl Player {
    /// Returns a Player object representing Player 1 at the start of the game.  Player 1
    /// should begin with its head at location (4,4) (i.e., 4 units right and down from the
    /// top-left corner).  It should be oriented (initially moving) to the right, with 2
    /// additional blocks trailing behind (to the left of ) its head.  Player 1 should
    /// be colored blue (pick a nice shade of blue)
    pub fn player_1() -> Player {
        todo!()
    }

    /// Returns a Player object representing Player 2 at the start of the game.  Player 2
    /// should begin with its head positioned 4 units left and up from the bottom-right corner.
    /// It should be oriented (initially moving) upward, with 2 additional blocks trailing
    /// below (to the left of ) its head.  Player 2 should be colored red (pick a nice shade of red)
    pub fn player_2() -> Player {
        todo!()
    }

    /// Draws the player given a graphics Context and G2d.  A player is drawn by drawing all
    /// of the blocks in its trail.  Note that the type Block has a draw() function, which can
    /// be used here when iterating over the player's trail.  Color the head block slightly,
    /// but noticeably, brighter than the rest of the trail.
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        todo!()
    }

    /// Moves a player forward by one block.  This should simply extend their trail in their
    /// moving direction.
    pub fn move_forward(&mut self) {
        todo!()
    }

    /// Based on the current "head" position of the player and its current moving direction,
    /// returns the position that the head would be in if this player moved forward by one block.
    pub fn next_head_position(&self) -> Block {
        todo!()
    }

    /// Updates the player's moving direction to the parameter, unless the parameter is None or it
    /// is the same as the current direction or the opposite of the current direction.  (e.g., if
    /// a player is currently moving Right, then this method will only update the player's direction
    /// if `Some(Direction::Up)` or `Some(Direction::Down)` are passed to the method.
    pub fn update_direction(&mut self, dir: Option<Direction>) {
        todo!()
    }

    /// Checks if the specified location is covered by the player's trail.
    pub fn trail_covers_location(&self, location: Block) -> bool {
        todo!()
    }

    /// Checks if the player would run into its own trail if it were to move forward
    /// by one block in its current moving direction.
    pub fn imminent_self_collision(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_opposite_direction() { todo!() }

    fn test_player_1() { todo!() }

    fn test_player_2() {
        todo!()
    }

    fn test_move_forward() {
        todo!()
    }

    fn test_next_head_position() {
        todo!()
    }

    fn test_update_direction() {
        todo!()
    }

    fn test_trail_covers_location() {
        todo!()
    }

    fn test_imminent_self_collision() {
        todo!()
    }
}

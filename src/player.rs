use crate::graphics::Block;
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
}

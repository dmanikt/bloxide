use crate::graphics::Block;
use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

/// A simple enumerated type representing the four directions a player can move.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
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
    /// should begin with its head at location (4,3) (i.e., 5 units right and 4 down from the
    /// top-left corner).  It should be oriented (initially moving) to the right, with 2
    /// additional blocks trailing behind (to the left of) its head.  Player 1 should
    /// be colored red
    pub fn player_1() -> Player {
        let mut trail = LinkedList::new();
        // The trail begins as 3 horizontal Blocks with the "head" 5 blocks from the
        // left wall and 4 blocks below the top.  The other two blocks are directly
        // to the left of the head.
        for x in 2..=4 {
            trail.push_front(Block {
                x,
                y: 3,
            });
        }
        Player {
            moving_direction: Direction::Right,
            has_moved_in_direction: false,
            trail,
            color: piston_window::color::hex("ff0000"), // red
        }
    }

    /// Returns a Player object representing Player 2 at the start of the game.  Player 2
    /// should begin with its head positioned 4 units left and 5 up from the bottom-right corner.
    /// It should be oriented (initially moving) upward, with 2 additional blocks trailing
    /// behind (below) its head.  Player 2 should be colored blue
    pub fn player_2(game_width: u32, game_height: u32) -> Player {
        let mut trail = LinkedList::new();
        // The trail begins as 3 vertical Blocks with the "head" 5 Blocks up from the
        // bottom and 4 Blocks from the right wall.  The other two blocks are directly
        // below the head.
        for y in (game_height - 5)..=(game_height - 3) {
            trail.push_back(Block {
                x: game_width - 4,
                y,
            });
        }
        Player {
            moving_direction: Direction::Up,
            has_moved_in_direction: false,
            trail,
            color: piston_window::color::hex("0000ff"), // blue
        }
    }

    /// Draws the player given a graphics Context and G2d.  A player is drawn by drawing all
    /// of the blocks in its trail.  Note that the type Block has a draw() function, which can
    /// be used here when iterating over the player's trail.
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.trail {
            block.draw(self.color, con, g);
        }
    }

    /// Moves a player forward by one block.  This should simply extend their trail in their
    /// moving direction.
    pub fn move_forward(&mut self) {
        self.has_moved_in_direction = true;
        self.trail.push_front(self.next_head_position());
    }

    /// Based on the current "head" position of the player and its current moving direction,
    /// returns the position that the head would be in if this player moved forward by one block.
    pub fn next_head_position(&self) -> Block {
        let &Block { x: head_x, y: head_y } = self.trail.front().unwrap();
        match self.moving_direction {
            Direction::Up => Block { x: head_x, y: head_y - 1 },
            Direction::Down => Block { x: head_x, y: head_y + 1 },
            Direction::Left => Block { x: head_x - 1, y: head_y },
            Direction::Right => Block { x: head_x + 1, y: head_y },
        }
    }

    /// Updates the player's moving direction to the parameter, unless the parameter is None or it
    /// is the same as the current direction or the opposite of the current direction, or the player
    /// has not yet moved in that direction.  (e.g., if a player is currently moving Right, then
    /// this method will only update the player's direction if `Some(Direction::Up)` or
    /// `Some(Direction::Down)` are passed to the method.)
    pub fn update_direction(&mut self, dir: Option<Direction>) {
        if self.has_moved_in_direction {
            if let Some(direction) = dir {
                if direction != self.moving_direction && direction != self.moving_direction.opposite_direction() {
                    self.moving_direction = direction;
                    self.has_moved_in_direction = false;
                }
            }
        }
    }

    /// Checks if the specified location is covered by the player's trail.
    pub fn trail_covers_location(&self, location: Block) -> bool {
        self.trail.contains(&location)
    }

    /// Checks if the player would run into its own trail if it were to move forward
    /// by one block in its current moving direction.
    pub fn imminent_self_collision(&self) -> bool {
        self.trail_covers_location(self.next_head_position())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opposite_direction() {
        assert_eq!(Direction::Up, Direction::Down.opposite_direction());
        assert_eq!(Direction::Down, Direction::Up.opposite_direction());
        assert_eq!(Direction::Left, Direction::Right.opposite_direction());
        assert_eq!(Direction::Right, Direction::Left.opposite_direction());
    }

    #[test]
    fn test_player_1() {
        let player_1 = Player::player_1();
        assert_eq!(false, player_1.has_moved_in_direction);
        assert_eq!(Direction::Right, player_1.moving_direction);
        assert_eq!(3, player_1.trail.len());
        assert_eq!(&Block { x: 4, y: 3 }, player_1.trail.front().unwrap());
    }

    #[test]
    fn test_player_2() {
        let game_width = 35_u32;
        let game_height = 25_u32;
        let player_2 = Player::player_2(game_width, game_height);
        assert_eq!(false, player_2.has_moved_in_direction);
        assert_eq!(Direction::Up, player_2.moving_direction);
        assert_eq!(3, player_2.trail.len());
        assert_eq!(&Block { x: 31, y: 20 }, player_2.trail.front().unwrap());
    }

    #[test]
    fn test_move_forward() {
        let mut player_1 = Player::player_1();
        player_1.move_forward();

        assert_eq!(4, player_1.trail.len());
        assert!(player_1.trail_covers_location(Block { x: 5, y: 3 }));
    }

    #[test]
    fn test_next_head_position() {
        let player_2 = Player::player_2(30, 30);
        assert_eq!(Block { x: 26, y: 24 }, player_2.next_head_position());
    }

    #[test]
    fn test_update_direction() {
        let mut player_1 = Player::player_1();
        player_1.move_forward();

        player_1.update_direction(Some(Direction::Up));
        assert_eq!(Direction::Up, player_1.moving_direction);

        player_1.update_direction(None);
        assert_eq!(Direction::Up, player_1.moving_direction);

        player_1.update_direction(Some(Direction::Down));
        assert_eq!(Direction::Up, player_1.moving_direction);
    }

    #[test]
    fn test_trail_covers_location() {
        let player_1 = Player::player_1();

        assert!(player_1.trail_covers_location(Block { x: 2, y: 3 }));
        assert!(!player_1.trail_covers_location(Block {x: 10, y: 4}));
    }

    #[test]
    fn test_imminent_self_collision() {
        let mut player_1 = Player::player_1();

        player_1.move_forward();
        player_1.update_direction(Some(Direction::Down));
        player_1.move_forward();
        player_1.update_direction(Some(Direction::Left));
        player_1.move_forward();
        player_1.update_direction(Some(Direction::Up));
        assert!(player_1.imminent_self_collision());
    }
}

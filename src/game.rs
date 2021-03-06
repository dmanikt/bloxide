use crate::graphics::{draw_rectangle, Block};
use crate::player;
use crate::player::{Direction, Player};
use piston_window::{Context, G2d, Key};

/// Determines the time step length in between advancements of each player.
pub const PLAYER_SPEED: f64 = 0.10;

/// A Game struct holds information related to the size of the game board,
/// the two players, and the status of the game.
pub struct Game {
    width: u32,  // Measured in "blocks"
    height: u32, // Measured in "blocks"

    ai: bool, // whether or not the AI is playing for player one

    player_one: player::Player,
    player_two: player::Player,

    winner: Option<bool>, // Some(true) for player 1, Some(false) for player 2, None if game isn't over.

    is_game_over: bool,
}

impl Game {
    /// Creates a new Game with the given width and height.  Initialize Players 1 and 2 for the
    /// game by calling the player_1 and player_2 methods in player.rs.
    pub fn new(width: u32, height: u32) -> Self {
        Game {
            width,
            height,

            ai: false,

            player_one: Player::player_1(),
            player_two: Player::player_2(width, height),

            winner: None,

            is_game_over: false,
        }
    }

    /// Draws the game by first drawing both players, and then drawing a black border on the outer
    /// edge of the game window.
    pub fn draw(&self, con: &Context, g: &mut G2d) -> (Option<bool>, bool) {
        self.player_one.draw(con, g, self.ai);
        self.player_two.draw(con, g, false);

        // draw the border of the game last so that it covers up anything on the border
        draw_rectangle([0., 0., 0., 1.0], 0, 0, self.width, 1, con, g);
        draw_rectangle([0., 0., 0., 1.0], 0, 0, 1, self.height, con, g);
        draw_rectangle([0., 0., 0., 1.0], 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle([0., 0., 0., 1.0], self.width - 1, 0, 1, self.height, con, g);

        (self.winner, self.ai)
    }

    /// Returns the height of the game (in blocks)
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Returns the width of the game (in blocks)
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Given an amount of time elapsed (this will be provided by the game window itself),
    /// updates both of the players to account for this passed time (both of them should have
    /// their wait_time methods called).  If this causes time_waited to meet or exceed the
    /// constant `PLAYER_SPEED` for either (or both) players, then the appropriate players should
    /// be moved forward ONLY after verifying that they are not going to lose if they do so.
    /// If they will lose, then the game is over and the proper winner should be indicated.
    pub fn update(&mut self, time_elapsed: f64) {
        self.player_one.wait_time(time_elapsed);
        self.player_two.wait_time(time_elapsed);

        if !self.is_game_over {
            if self.player_one.time_waited() >= PLAYER_SPEED && !self.ai {
                if self.player_one_collision(self.player_one.next_head_position()) {
                    self.is_game_over = true;
                    self.winner = Some(false);
                } else {
                    self.player_one.move_forward();
                    self.player_one.advance_direction_queue();
                    self.player_one.wait_time(-PLAYER_SPEED);
                }
            }
            // if AI enabled
            else if self.player_one.time_waited() >= PLAYER_SPEED {
                // if about to crash, turn
                if self.player_one_collision(self.player_one.next_head_position()) {
                    self.update_ai_direction();
                    if self.player_one_collision(self.player_one.next_head_position()) {
                        self.is_game_over = true;
                        self.winner = Some(false);
                    }
                } else {
                    self.player_one.move_forward();
                    self.player_one.advance_direction_queue();
                    self.player_one.wait_time(-PLAYER_SPEED);
                }
            }

            if self.player_two.time_waited() >= PLAYER_SPEED {
                if self.player_two_collision() {
                    self.is_game_over = true;
                    self.winner = Some(true);
                } else {
                    self.player_two.move_forward();
                    self.player_two.advance_direction_queue();
                    self.player_two.wait_time(-PLAYER_SPEED);
                }
            }
        }
    }

    /// Updates player 1 to turn away from a wall in a direction that will keep it alive.
    /// This forces the other player to make a risky move to beat the AI.
    pub fn update_ai_direction(&mut self) {
        if !self.player_one_collision(self.player_one.position_on_turn()) {
            self.player_one.turn(false)
        } else if !self.player_one_collision(self.player_one.position_on_cc()) {
            self.player_one.turn(true);
        }
    }

    /// Updates the game based on a key pressed by the user.  The WASD keys will change the
    /// direction of player 1 (do this by calling the update_direction method for player 1).
    /// However, they will do nothing if player one is currently being controlled by the AI.
    /// The directional arrow keys will update the direction of player 2 (do this by calling the
    /// update_direction method for player 2).  The enter key will restart the game, but only
    /// if the game is currently over.  The p key toggles the AI on and off for player 1.
    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::W => {
                if !self.ai {
                    self.player_one.update_direction(Some(Direction::Up))
                }
            }
            Key::A => {
                if !self.ai {
                    self.player_one.update_direction(Some(Direction::Left))
                }
            }
            Key::S => {
                if !self.ai {
                    self.player_one.update_direction(Some(Direction::Down))
                }
            }
            Key::D => {
                if !self.ai {
                    self.player_one.update_direction(Some(Direction::Right))
                }
            }
            Key::Up => self.player_two.update_direction(Some(Direction::Up)),
            Key::Down => self.player_two.update_direction(Some(Direction::Down)),
            Key::Left => self.player_two.update_direction(Some(Direction::Left)),
            Key::Right => self.player_two.update_direction(Some(Direction::Right)),
            Key::P => self.ai = !self.ai,
            Key::Return => {
                if self.is_game_over {
                    self.restart();
                }
            }
            _ => {}
        }
    }

    /// Resets the state of the game to represent a brand new game by creating new
    /// players and resetting is_game_over and time_waited.
    pub fn restart(&mut self) {
        self.player_one = Player::player_1();
        self.player_two = Player::player_2(self.width, self.height);

        self.winner = None;

        self.is_game_over = false;
        self.ai = false;
    }

    /// Checks if the given Block (i.e., a location) is out of the bounds of the gameboard.
    /// This will be used when determining if a snake has run out of bounds (i.e., died)
    fn is_out_of_bounds(&self, block: Block) -> bool {
        block.x == 0 || block.x >= (self.width - 1) || block.y == 0 || block.y >= (self.height - 1)
    }

    /// Checks if player one will be facing a collision in its next movement by
    /// 1. if the player is out of bounds
    /// 2. if the player crashes into itself
    /// 3. if the player crashes into the other snake
    fn player_one_collision(&mut self, position: Block) -> bool {
        self.is_out_of_bounds(position)
            || self.player_one.imminent_self_collision()
            || self.player_two.trail_covers_location(position)
    }

    /// Checks if player two will be facing a collision in its next movement
    fn player_two_collision(&mut self) -> bool {
        self.is_out_of_bounds(self.player_two.next_head_position())
            || self.player_two.imminent_self_collision()
            || self
                .player_one
                .trail_covers_location(self.player_two.next_head_position())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::Block;

    #[test]
    fn test_new() {
        let game = Game::new(35, 25);
        assert_eq!(35, game.width);
        assert_eq!(25, game.height);
        assert_eq!(Block { x: 5, y: 3 }, game.player_one.next_head_position());
        assert_eq!(Block { x: 31, y: 19 }, game.player_two.next_head_position());
        assert_eq!(false, game.is_game_over);
    }

    #[test]
    fn test_update() {
        let mut game = Game::new(35, 25);
        game.update(0.08);
        assert_eq!(0.08, game.player_one.time_waited());

        game.update(0.08);
        assert_eq!(0.16 - PLAYER_SPEED, game.player_one.time_waited());
    }

    #[test]
    fn test_key_pressed() {
        let mut game = Game::new(35, 25);
        game.update(PLAYER_SPEED + 0.1);
        game.key_pressed(Key::S);
        game.key_pressed(Key::D);
        game.key_pressed(Key::Left);

        assert_eq!(Block { x: 5, y: 4 }, game.player_one.next_head_position());
        assert_eq!(Block { x: 30, y: 19 }, game.player_two.next_head_position())
    }

    #[test]
    fn test_is_out_of_bounds() {
        let game = Game::new(35, 25);
        assert!(game.is_out_of_bounds(Block { x: 0, y: 0 }));
        assert!(game.is_out_of_bounds(Block { x: 15, y: 24 }));
        assert!(!game.is_out_of_bounds(Block { x: 15, y: 15 }));
        assert!(!game.is_out_of_bounds(Block { x: 1, y: 1 }));
    }
}

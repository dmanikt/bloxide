//! This file contains useful functions and constants for drawing
//! the state of the game on the game window.

use piston_window::types::Color;
use piston_window::*;

/// Constants keeping track of the game's background color as well
/// as the number of "points" (pixels on most machines) that each
/// Block occupies in each dimension.
pub const BACK_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
pub const POINTS_PER_BLOCK: f64 = 25.0;

/// A Block simply contains an x and y coordinate representing its position
/// on the game board.  The top left corner is (0, 0), x-values increase to
/// the right all the way to the width - 1, and y-values increase going down
/// all the way to the height - 1.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Block {
    pub x: u32,
    pub y: u32,
}

impl Block {
    /// Draws the current Block in the given Color on the give Context and
    /// 2D Graphics object.
    pub fn draw(&self, color: Color, con: &Context, g: &mut G2d) {
        draw_rectangle(color, self.x, self.y, 1, 1, con, g);
    }
}

/// Utility drawing function.  Draws a rectangle with the given Color and location.  The
/// location should be understood as follows: the start x and y coordinates are Block
/// coordinates, not pixels.  The width and height are also given in number of blocks.
pub fn draw_rectangle(
    color: Color,
    start_x: u32,
    start_y: u32,
    width: u32,
    height: u32,
    con: &Context,
    g: &mut G2d,
) {
    let gui_start_x = (start_x as f64) * 25.0;
    let gui_start_y = (start_y as f64) * 25.0;

    rectangle(
        color,
        [
            gui_start_x,
            gui_start_y,
            25.0 * (width as f64),
            25.0 * (height as f64),
        ],
        con.transform,
        g,
    );
}

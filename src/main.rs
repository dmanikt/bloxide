extern crate piston_window;

mod game;
mod graphics;
mod player;

use piston_window::*;
use crate::game::Game;

fn main() {
    // The dimensions of the game board, in "block" units.
    let (width, height) = (35_u32, 25_u32);

    // Creating the frame within which the game is displayed
    let mut window: PistonWindow = WindowSettings::new(
        "Bloxide",
        [
            ((width as f64) * graphics::POINTS_PER_BLOCK) as u32,
            ((height as f64) * graphics::POINTS_PER_BLOCK) as u32,
        ],
    )
    .exit_on_esc(true)
    .resizable(false)
    .build()
    .unwrap();

    let mut game = Game::new(width, height);

    // main animation loop (will be updated later when more is implemented)
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // handle key events
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _dev| {
            clear(graphics::BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            // update game backend
            game.update(arg.dt);
        });
    }
}

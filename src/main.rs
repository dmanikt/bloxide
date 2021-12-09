extern crate piston_window;

mod game;
mod graphics;
mod player;

use crate::game::Game;
use gfx_device_gl::{CommandBuffer, Device, Factory, Resources};
use piston_window::glyph_cache::rusttype::GlyphCache;
use piston_window::*;

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

    // setup for font drawing
    let assets = find_folder::Search::ParentsThenKids(3, 2)
        .for_folder("assets")
        .unwrap();
    let font = &assets.join("AtariClassic-gry3.ttf");
    let texture_context = piston_window::TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let texture_settings = piston_window::TextureSettings::new();
    let mut glyphs = Glyphs::new(font, texture_context, texture_settings).unwrap();

    // main animation loop
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // handle key events
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, dev| {
            clear(graphics::BACK_COLOR, g);
            let (winner, ai) = game.draw(&c, g);

            // if the game is over, call a function that draws the appropriate
            // game-over message.
            if let Some(player) = winner {
                game_over_screen(&game, player, ai, &c, g, dev, &mut glyphs)
            }
        });

        event.update(|arg| {
            // update game backend
            game.update(arg.dt);
        });
    }
}

/// Draws the game-over screen displaying which player won.  True represents the red player
/// winning, false represents the blue player winning.
fn game_over_screen(
    game: &Game,
    winner: bool,
    ai: bool,
    con: &Context,
    g: &mut G2d,
    dev: &mut Device,
    glyphs: &mut GlyphCache<TextureContext<Factory, Resources, CommandBuffer>, Texture<Resources>>,
) {
    let main_color = if winner && ai {
        [0., 1.0, 0., 0.15]
    } else if winner {
        [1.0, 0., 0., 0.15]
    } else {
        [0., 0., 1.0, 0.15]
    };
    graphics::draw_rectangle(
        main_color,
        0,
        0,
        game.get_width(),
        game.get_height(),
        con,
        g,
    );

    let game_over_msg = if winner && ai {
        "Green Player Wins!"
    } else if winner {
        "Red Player Wins!"
    } else {
        "Blue Player Wins!"
    };

    let (trans_x, trans_y) = if winner && ai {
        (140.0, 300.0)
    } else if winner {
        (170.0, 300.0)
    } else {
        (155.0, 300.0)
    };

    //drawing the text on the game over screen
    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
        .draw(
            game_over_msg,
            glyphs,
            &con.draw_state.clone(),
            con.transform.trans(trans_x, trans_y).scale(0.8, 0.8),
            g,
        )
        .unwrap();

    glyphs.factory.encoder.flush(dev);
}

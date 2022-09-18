extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;
use crate::draw::{to_coords, to_coords_u32};
use crate::game::Game;

const BACK_COLOR: Color = [0.2, 0.2, 0.2, 1.0];

fn main() {
    let (width, height) = (30, 30);
    let mut window: PistonWindow = WindowSettings::new("Snake", [to_coords_u32(width), to_coords_u32(height)])
        .exit_on_esc(true).build().unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });

        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key)
        }
    }
}

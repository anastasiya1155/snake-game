use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::rectangle;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coords(game_coords:  i32) -> f64 {
    (game_coords as f64) * BLOCK_SIZE
}

pub fn to_coords_u32(game_coords: i32) -> u32 {
    to_coords(game_coords) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coords(x);
    let gui_y = to_coords(y);

    rectangle(color, [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coords(x);
    let gui_y = to_coords(y);

    rectangle(color, [gui_x, gui_y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)], con.transform, g);
}

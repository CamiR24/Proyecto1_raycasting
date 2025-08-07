use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
}

pub fn find_player_position(maze: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == 'p' {
                return Some((col_index, row_index));
            }
        }
    }
    None
}
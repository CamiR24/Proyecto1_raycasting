// player.rs

use raylib::prelude::*;
use std::f32::consts::PI;
use crate::maze::Maze;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
}

impl Player {
    pub fn is_position_free(&self, maze: &Maze, new_pos: Vector2, block_size: f32) -> bool {
        let player_radius = block_size * 0.3;
        
        let positions_to_check = [
            Vector2::new(new_pos.x - player_radius, new_pos.y - player_radius), 
            Vector2::new(new_pos.x + player_radius, new_pos.y - player_radius),
            Vector2::new(new_pos.x - player_radius, new_pos.y + player_radius), 
            Vector2::new(new_pos.x + player_radius, new_pos.y + player_radius), 
            new_pos, 
        ];
        
        for pos in &positions_to_check {
            let grid_x = (pos.x / block_size) as usize;
            let grid_y = (pos.y / block_size) as usize;

            if grid_y >= maze.len() || grid_x >= maze[0].len() {
                return false; //fuera del maze
            }
            
            //verificar si hay pared 
            let cell = maze[grid_y][grid_x];
            if cell != ' ' && cell != 'g' {
                return false; //si hay
            }
        }
        
        true //posiciones libres
    }
    
    pub fn has_reached_goal(&self, maze: &Maze, block_size: f32) -> bool {
        let grid_x = (self.pos.x / block_size) as usize;
        let grid_y = (self.pos.y / block_size) as usize;
        
        if grid_y < maze.len() && grid_x < maze[0].len() {
            maze[grid_y][grid_x] == 'g'
        } else {
            false
        }
    }
}

pub fn process_events(player: &mut Player, rl: &RaylibHandle, maze: &Maze, block_size: f32) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 10.0;
    const MOUSE_SENSITIVITY: f32 = 0.003;

    let mouse_delta = rl.get_mouse_delta(); 
    player.a += -mouse_delta.x * MOUSE_SENSITIVITY; 

    if player.a < 0.0 {
        player.a += 2.0 * PI;
    } else if player.a > 2.0 * PI {
        player.a -= 2.0 * PI;
    }

    if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
        player.a += ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
        player.a -= ROTATION_SPEED;
    }

    let mut new_pos = player.pos;

    if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
        new_pos.x += MOVE_SPEED * player.a.cos();
        new_pos.y += MOVE_SPEED * player.a.sin();
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
        new_pos.x -= MOVE_SPEED * player.a.cos();
        new_pos.y -= MOVE_SPEED * player.a.sin();
    }

    if player.is_position_free(maze, new_pos, block_size) {
        player.pos = new_pos;
    } else {
        let new_pos_x = Vector2::new(new_pos.x, player.pos.y);
        if player.is_position_free(maze, new_pos_x, block_size) {
            player.pos = new_pos_x;
        } else {
            let new_pos_y = Vector2::new(player.pos.x, new_pos.y);
            if player.is_position_free(maze, new_pos_y, block_size) {
                player.pos = new_pos_y;
            }
        }
    }
}
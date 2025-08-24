// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod caster;
mod player;

use line::line;
use maze::{Maze,load_maze};
use caster::{cast_ray, Intersect};
use framebuffer::Framebuffer;
use player::{Player, process_events};

use raylib::prelude::*;
use std::thread;
use std::time::{Duration, Instant};
use std::f32::consts::PI;

fn cell_to_color(cell: char) -> Color {
  match cell {
    'A' => {
      return Color::new(255, 215, 0, 255);
    },
    'R' => {
      return Color::new(220, 20, 60, 255);
    },
    'V' => {
      return Color::new(50, 205, 50, 255);
    },
    'M' => {
      return Color::new(138, 43, 226, 255);
    },
    'B' => {
      return Color::new(30, 144, 255, 255);
    },
    'T' => {
      return Color::new(64, 224, 208, 255);
    },
    'P' => {
      return Color::new(255, 105, 180, 255);
    },
    'N' => {
      return Color::new(255, 140, 0, 255);
    },
    'g' => {
      return Color::GRAY;
    },
    _ => {
      return Color::WHITE;
    },
  }
}

fn draw_cell(
  framebuffer: &mut Framebuffer,
  xo: usize,
  yo: usize,
  block_size: usize,
  cell: char,
) {
  if cell == ' ' {
    return;
  }
  let color = cell_to_color(cell);
  framebuffer.set_current_color(color);

  for x in xo..xo + block_size {
    for y in yo..yo + block_size {
      framebuffer.set_pixel(x as u32, y as u32);
    }
  }
}

pub fn render_maze(
  framebuffer: &mut Framebuffer,
  maze: &Maze,
  block_size: usize,
  player: &Player,
) {
  for (row_index, row) in maze.iter().enumerate() {
    for (col_index, &cell) in row.iter().enumerate() {
      let xo = col_index * block_size;
      let yo = row_index * block_size;
      draw_cell(framebuffer, xo, yo, block_size, cell);
    }
  }

  framebuffer.set_current_color(Color::WHITESMOKE);

  // draw what the player sees
  let num_rays = 5;
  for i in 0..num_rays {
    let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
    let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    cast_ray(framebuffer, &maze, &player, a, block_size, true);
  }
}

fn render_minimap(
  framebuffer: &mut Framebuffer,
  maze: &Maze,
  player: &Player,
  block_size: usize,
) {
  let margin = 20;
  
  // Calcular el tamaño real del maze
  let maze_rows = maze.len();
  let maze_cols = if maze_rows > 0 { maze[0].len() } else { 0 };
  
  // Calcular el tamaño del minimapa basado en el maze real
  let mini_block_size = 12; // Un poco más grande para mejor visibilidad
  let minimap_width = maze_cols * mini_block_size + 20; // +20 para padding
  let minimap_height = maze_rows * mini_block_size + 20;
  
  // Posición del minimapa (esquina superior derecha)
  let minimap_x = framebuffer.width as usize - minimap_width - margin;
  let minimap_y = margin;
  
  // Fondo del minimapa con borde
  framebuffer.set_current_color(Color::new(0, 0, 0, 200));
  for x in minimap_x..minimap_x + minimap_width {
      for y in minimap_y..minimap_y + minimap_height {
          framebuffer.set_pixel(x as u32, y as u32);
      }
  }
  
  // Borde blanco del minimapa
  framebuffer.set_current_color(Color::WHITE);
  // Borde superior e inferior
  for x in minimap_x..minimap_x + minimap_width {
      framebuffer.set_pixel(x as u32, minimap_y as u32);
      framebuffer.set_pixel(x as u32, (minimap_y + minimap_height - 1) as u32);
  }
  // Borde izquierdo y derecho
  for y in minimap_y..minimap_y + minimap_height {
      framebuffer.set_pixel(minimap_x as u32, y as u32);
      framebuffer.set_pixel((minimap_x + minimap_width - 1) as u32, y as u32);
  }
  
  // Dibujar el maze en el minimapa con padding
  let padding = 10;
  for (row_index, row) in maze.iter().enumerate() {
      for (col_index, &cell) in row.iter().enumerate() {
          if cell != ' ' {
              let color = cell_to_color(cell);
              framebuffer.set_current_color(color);
              
              let mini_x = minimap_x + padding + col_index * mini_block_size;
              let mini_y = minimap_y + padding + row_index * mini_block_size;
              
              // Dibujar cada celda del maze
              for x in mini_x..mini_x + mini_block_size - 1 { // -1 para separación entre celdas
                  for y in mini_y..mini_y + mini_block_size - 1 {
                      if x < minimap_x + minimap_width - 1 && y < minimap_y + minimap_height - 1 {
                          framebuffer.set_pixel(x as u32, y as u32);
                      }
                  }
              }
          }
      }
  }
  
  // Calcular la posición del jugador en el minimapa
  let player_grid_x = (player.pos.x / block_size as f32) as usize;
  let player_grid_y = (player.pos.y / block_size as f32) as usize;
  
  let player_mini_x = minimap_x + padding + player_grid_x * mini_block_size + mini_block_size / 2;
  let player_mini_y = minimap_y + padding + player_grid_y * mini_block_size + mini_block_size / 2;
  
  // Dibujar la posición del jugador (punto amarillo más grande)
  framebuffer.set_current_color(Color::YELLOW);
  for dx in -3..=3 {
      for dy in -3..=3 {
          let px = (player_mini_x as i32 + dx) as usize;
          let py = (player_mini_y as i32 + dy) as usize;
          if px < framebuffer.width as usize && py < framebuffer.height as usize {
              framebuffer.set_pixel(px as u32, py as u32);
          }
      }
  }
  
  // Dibujar dirección del jugador (línea roja)
  framebuffer.set_current_color(Color::RED);
  let dir_length = 20.0;
  let end_x = player_mini_x as f32 + player.a.cos() * dir_length;
  let end_y = player_mini_y as f32 + player.a.sin() * dir_length;
  
  line(framebuffer, 
       Vector2::new(player_mini_x as f32, player_mini_y as f32),
       Vector2::new(end_x, end_y));
}

fn render_world(
  framebuffer: &mut Framebuffer,
  maze: &Maze,
  block_size: usize,
  player: &Player,
) {
  let num_rays = framebuffer.width;

  // let hw = framebuffer.width as f32 / 2.0;   // precalculated half width
  let hh = framebuffer.height as f32 / 2.0;  // precalculated half height

  framebuffer.set_current_color(Color::WHITESMOKE);

  for i in 0..num_rays {
    let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
    let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

    // Calculate the height of the stake
    let distance_to_wall = intersect.distance;// how far is this wall from the player
    let distance_to_projection_plane = 70.0; // how far is the "player" from the "camera"
    // this ratio doesn't really matter as long as it is a function of distance
    let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

    // Calculate the position to draw the stake
    let stake_top = (hh - (stake_height / 2.0)) as usize;
    let stake_bottom = (hh + (stake_height / 2.0)) as usize;

    // Draw the stake directly in the framebuffer
    for y in stake_top..stake_bottom {
      framebuffer.set_pixel(i, y as u32); // Assuming white color for the stake
    }
  }
}

fn render_fps(framebuffer: &mut Framebuffer, fps: f32) {
  // Renderizar indicador simple de FPS en la esquina superior izquierda
  let bar_width = (fps / 60.0 * 100.0).min(100.0) as usize;
  
  // Fondo para el indicador de FPS
  framebuffer.set_current_color(Color::new(0, 0, 0, 200));
  for x in 10..120 {
      for y in 10..25 {
          framebuffer.set_pixel(x as u32, y as u32);
      }
  }
  
  // Barra de FPS - verde si >30, amarillo si >20, rojo si <20
  let bar_color = if fps > 30.0 {
      Color::GREEN
  } else if fps > 20.0 {
      Color::YELLOW
  } else {
      Color::RED
  };
  
  framebuffer.set_current_color(bar_color);
  for x in 15..15 + bar_width {
      for y in 15..20 {
          framebuffer.set_pixel(x as u32, y as u32);
      }
  }
  
  // Marco blanco
  framebuffer.set_current_color(Color::WHITE);
  for x in 14..116 {
      framebuffer.set_pixel(x as u32, 14);
      framebuffer.set_pixel(x as u32, 21);
  }
  for y in 14..22 {
      framebuffer.set_pixel(14, y as u32);
      framebuffer.set_pixel(115, y as u32);
  }
}

fn main() {
  let window_width = 1300;
  let window_height = 900;
  let block_size = 100;

  let (mut window, raylib_thread) = raylib::init()
    .size(window_width, window_height)
    .title("Raycaster Example")
    .log_level(TraceLogLevel::LOG_WARNING)
    .build();

  let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
  framebuffer.set_background_color(Color::new(153, 102, 204, 255));

  let maze = load_maze("maze.txt");
  let mut player = Player {
    pos: Vector2::new(150.0, 150.0),
    a: PI / 3.0,
    fov: PI / 3.0,
  };

  let mut fps = 60.0;
  let mut frame_count = 0;
  let mut fps_timer = Instant::now();

  while !window.window_should_close() {
    frame_count += 1;
    if fps_timer.elapsed().as_secs_f32() >= 1.0 {
        fps = frame_count as f32 / fps_timer.elapsed().as_secs_f32();
        frame_count = 0;
        fps_timer = Instant::now();
    }

    // 1. clear framebuffer
    framebuffer.clear();

    // 2. move the player on user input
    process_events(&mut player, &window);

    let mut mode = "2D";

    if window.is_key_down(KeyboardKey::KEY_M) {
      mode = if mode == "2D" { "3D" } else { "2D" };
    }

    // 3. draw stuff
    if mode == "2D" {
      render_maze(&mut framebuffer, &maze, block_size, &player);
    } else {
      render_world(&mut framebuffer, &maze, block_size, &player);
    }

    if mode == "3D" {
      render_minimap(&mut framebuffer, &maze, &player, block_size);
    }

    render_fps(&mut framebuffer, fps);

    // 4. swap buffers
    framebuffer.swap_buffers(&mut window, &raylib_thread);

    thread::sleep(Duration::from_millis(16));
  }
}




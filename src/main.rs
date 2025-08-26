mod line;
mod framebuffer;
mod maze;
mod caster;
mod player;
mod textures;
mod images;
mod menu;

use line::line;
use maze::{Maze,load_maze};
use caster::{cast_ray, Intersect};
use framebuffer::Framebuffer;
use player::{Player, process_events};
use textures::TextureManager;
use menu::{MenuImages};

use raylib::prelude::Texture2D;
use raylib::prelude::*;
use raylib::color::Color;
use std::thread;
use std::time::{Duration, Instant};
use std::f32::consts::PI;

enum GameState {
  Menu,
  Playing,
  Victory,
}

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
  texture_manager: &TextureManager, // Agregar parámetro de texturas
) {
  let num_rays = framebuffer.width;
  let hh = framebuffer.height as f32 / 2.0;
  let texture_size = 128.0; // Tamaño estándar de textura como sugirió tu maestro

  for i in 0..num_rays {
    let current_ray = i as f32 / num_rays as f32;
    let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

    let distance_to_wall = intersect.distance;
    let distance_to_projection_plane = 70.0;
    let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

    let stake_top = (hh - (stake_height / 2.0)) as usize;
    let stake_bottom = (hh + (stake_height / 2.0)) as usize;

    // Calcular punto de impacto
    let hit_x = player.pos.x + distance_to_wall * a.cos();
    let hit_y = player.pos.y + distance_to_wall * a.sin();
    
    // Calcular qué celda del maze fue golpeada
    let map_x = (hit_x / block_size as f32).floor();
    let map_y = (hit_y / block_size as f32).floor();
    
    // Calcular la posición relativa dentro de la celda (0.0 a 1.0)
    let cell_x = hit_x / block_size as f32 - map_x;
    let cell_y = hit_y / block_size as f32 - map_y;
    
    // Determinar qué cara de la pared fue golpeada
    let hit_side = {
        let dx = hit_x - (map_x * block_size as f32 + block_size as f32 / 2.0);
        let dy = hit_y - (map_y * block_size as f32 + block_size as f32 / 2.0);
        
        if dx.abs() > dy.abs() {
            if dx > 0.0 { "east" } else { "west" }
        } else {
            if dy > 0.0 { "south" } else { "north" }
        }
    };
    
    // Calcular coordenada X de la textura basada en la cara golpeada
    let tx = match hit_side {
        "north" | "south" => (cell_x * texture_size) as u32,
        "east" | "west" => (cell_y * texture_size) as u32,
        _ => 0,
    };
    
    // Asegurar que tx esté en rango válido
    let tx = tx.min(127);

    // Renderizar la columna con textura
    for y in stake_top..stake_bottom {
      if y < framebuffer.height as usize {
        // Calcular coordenada Y de la textura (0 a 127)
        let texture_y_ratio = (y - stake_top) as f32 / (stake_bottom - stake_top).max(1) as f32;
        let ty = (texture_y_ratio * texture_size) as u32;
        let ty = ty.min(127);
        
        // Obtener el color del pixel de la textura
        let color = texture_manager.get_pixel_color(intersect.impact, tx, ty);
        
        // Aplicar sombreado basado en la distancia y cara
        let mut shade_factor = (1.0 - (distance_to_wall / 800.0).min(1.0)) * 0.7 + 0.3;
        
        // Diferentes tonos para diferentes caras (efecto 3D)
        shade_factor *= match hit_side {
            "north" => 1.0,    // Cara más clara
            "south" => 0.8,    // Cara más oscura  
            "east" => 0.9,     // Cara intermedia
            "west" => 0.7,     // Cara más oscura
            _ => 0.8,
        };
        
        let shaded_color = Color::new(
          (color.r as f32 * shade_factor) as u8,
          (color.g as f32 * shade_factor) as u8,
          (color.b as f32 * shade_factor) as u8,
          color.a,
        );
        
        framebuffer.set_current_color(shaded_color);
        framebuffer.set_pixel(i, y as u32);
      }
    }
  }
}

fn render_fps(framebuffer: &mut Framebuffer, fps: f32) {
  // Renderizar barra y número de FPS en la esquina superior izquierda
  let bar_width = (fps / 60.0 * 100.0).min(100.0) as usize;
  let fps_text = format!("{:.0}", fps);
  
  // Calcular dimensiones totales
  let char_width = 8;
  let char_height = 9;
  let text_width = fps_text.len() * char_width;
  let total_width = 110.max(text_width + 8); // Asegurar que sea al menos tan ancho como la barra
  let total_height = 35; // Altura para barra + texto + espaciado
  
  // Fondo para todo el indicador de FPS
  framebuffer.set_current_color(Color::new(0, 0, 0, 200));
  for x in 10..10 + total_width {
      for y in 10..10 + total_height {
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
  
  // Dibujar la barra de FPS
  framebuffer.set_current_color(bar_color);
  for x in 15..15 + bar_width {
      for y in 15..20 {
          framebuffer.set_pixel(x as u32, y as u32);
      }
  }
  
  // Dibujar el número de FPS debajo de la barra
  framebuffer.set_current_color(bar_color); // Usar el mismo color que la barra
  let text_start_x = 15; // Alineado con el inicio de la barra
  let text_start_y = 25; // Debajo de la barra
  
  for (i, ch) in fps_text.chars().enumerate() {
    let char_x = text_start_x + i * char_width;
    draw_digit(framebuffer, char_x, text_start_y, ch);
  }
  
  // Marco blanco alrededor de todo
  framebuffer.set_current_color(Color::WHITE);
  // Borde superior e inferior
  for x in 10..10 + total_width {
      framebuffer.set_pixel(x as u32, 10);
      framebuffer.set_pixel(x as u32, (10 + total_height - 1) as u32);
  }
  // Borde izquierdo y derecho
  for y in 10..10 + total_height {
      framebuffer.set_pixel(10, y as u32);
      framebuffer.set_pixel((10 + total_width - 1) as u32, y as u32);
  }
}

fn draw_digit(framebuffer: &mut Framebuffer, x: usize, y: usize, digit: char) {
  // Patrones de píxeles para cada dígito (7x9 píxeles)
  let patterns = match digit {
    '0' => [
      "  ###  ",
      " #   # ",
      " #   # ",
      " #   # ",
      " #   # ",
      " #   # ",
      " #   # ",
      " #   # ",
      "  ###  "
    ],
    '1' => [
      "   #   ",
      "  ##   ",
      "   #   ",
      "   #   ",
      "   #   ",
      "   #   ",
      "   #   ",
      "   #   ",
      " ##### "
    ],
    '2' => [
      " ####  ",
      "#    # ",
      "     # ",
      "    #  ",
      "   #   ",
      "  #    ",
      " #     ",
      "#      ",
      "###### "
    ],
    '3' => [
      " ####  ",
      "#    # ",
      "     # ",
      "  ###  ",
      "     # ",
      "     # ",
      "#    # ",
      "#    # ",
      " ####  "
    ],
    '4' => [
      "   ##  ",
      "  # #  ",
      " #  #  ",
      "#   #  ",
      "###### ",
      "    #  ",
      "    #  ",
      "    #  ",
      "    #  "
    ],
    '5' => [
      "###### ",
      "#      ",
      "#      ",
      "##### ",
      "     # ",
      "     # ",
      "#    # ",
      "#    # ",
      " ####  "
    ],
    '6' => [
      "  ###  ",
      " #   # ",
      " #     ",
      " ####  ",
      " #   # ",
      " #   # ",
      " #   # ",
      " #   # ",
      "  ###  "
    ],
    '7' => [
      "###### ",
      "     # ",
      "    #  ",
      "   #   ",
      "  #    ",
      " #     ",
      "#      ",
      "#      ",
      "#      "
    ],
    '8' => [
      " ####  ",
      "#    # ",
      "#    # ",
      " ####  ",
      "#    # ",
      "#    # ",
      "#    # ",
      "#    # ",
      " ####  "
    ],
    '9' => [
      " ####  ",
      "#    # ",
      "#    # ",
      "#    # ",
      " ##### ",
      "     # ",
      "     # ",
      "#    # ",
      " ####  "
    ],
    _ => [
      "       ",
      "       ",
      "       ",
      "       ",
      "       ",
      "       ",
      "       ",
      "       ",
      "       "
    ]
  };
  
  // Dibujar el patrón
  for (row, pattern) in patterns.iter().enumerate() {
    for (col, ch) in pattern.chars().enumerate() {
      if ch == '#' {
        let px = x + col;
        let py = y + row;
        if px < framebuffer.width as usize && py < framebuffer.height as usize {
          framebuffer.set_pixel(px as u32, py as u32);
        }
      }
    }
  }
}

fn main() {
  let window_width = 1300;
  let window_height = 900;
  let block_size = 100;

  // Solo inicializar UNA ventana
  let (mut window, raylib_thread) = raylib::init()
    .size(window_width, window_height)
    .title("Inside Out Maze - Raycaster")
    .log_level(TraceLogLevel::LOG_WARNING)
    .build();

  let menu_images = match MenuImages::load(&mut window, &raylib_thread) {
    Ok(images) => {
      println!("✓ Imágenes del menú cargadas correctamente");
      Some(images)
    },
    Err(e) => {
      println!("⚠ Error cargando imágenes: {}", e);
      None
    }
  };

  let texture_manager = TextureManager::new(&mut window, &raylib_thread);

  // Inicializar el sistema de audio
  let audio = match RaylibAudio::init_audio_device() {
      Ok(audio) => {
          println!("✓ Sistema de audio inicializado");
          audio
      },
      Err(e) => {
          println!("❌ Error al inicializar audio: {:?}", e);
          println!("Continuando sin audio...");
          // En lugar de pánico, seguimos sin audio
          RaylibAudio::init_audio_device().unwrap_or_else(|_| {
              panic!("No se puede inicializar el sistema de audio en absoluto");
          })
      }
  };
  
  // Intentar cargar música de fondo
  let mut music_opt: Option<Music> = None;
  
  match audio.new_music("assets/background.mp3") {
      Ok(music) => {
          println!("✓ Música cargada correctamente");
          music.set_volume(0.5);
          music.play_stream();
          music_opt = Some(music);
          println!("✓ Música iniciada");
      },
      Err(_) => {
          println!("⚠ No se pudo cargar 'assets/background.mp3'");
          println!("  Continuando sin música...");
      }
  }

  let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
  framebuffer.set_background_color(Color::new(153, 102, 204, 255));

  let mut maze = load_maze("maze_childhood.txt");
  let mut player = Player {
    pos: Vector2::new(150.0, 150.0),
    a: PI / 3.0,
    fov: PI / 3.0,
  };

  let mut fps = 60.0;
  let mut frame_count = 0;
  let mut fps_timer = Instant::now();
  let mut mode = "2D";

  let mut game_state = GameState::Menu;

  while !window.window_should_close() {
    if let Some(ref music) = music_opt {
        music.update_stream();
    }

    frame_count += 1;
    if fps_timer.elapsed().as_secs_f32() >= 1.0 {
        fps = frame_count as f32 / fps_timer.elapsed().as_secs_f32();
        frame_count = 0;
        fps_timer = Instant::now();
    }

    match game_state {
      GameState::Menu => {
          // --- Pantalla de menú ---
          let mut d = window.begin_drawing(&raylib_thread);
          menu::render_menu(&mut d, menu_images.as_ref());

          // Selección de laberinto
          if d.is_key_pressed(KeyboardKey::KEY_ONE) {
              maze = load_maze("maze_childhood.txt");
              player.pos = Vector2::new(150.0, 150.0);
              game_state = GameState::Playing;
          }
          if d.is_key_pressed(KeyboardKey::KEY_TWO) {
              maze = load_maze("maze_teen.txt");
              player.pos = Vector2::new(150.0, 150.0);
              game_state = GameState::Playing;
          }
          if d.is_key_pressed(KeyboardKey::KEY_THREE) {
              maze = load_maze("maze_adulthood.txt");
              player.pos = Vector2::new(150.0, 150.0);
              game_state = GameState::Playing;
          }
      }

      GameState::Playing => {
          // --- Juego principal ---
          process_events(&mut player, &window, &maze, block_size as f32);

          if window.is_key_pressed(KeyboardKey::KEY_M) {
              mode = if mode == "2D" { "3D" } else { "2D" };
              println!("Modo: {}", mode);
          }

          if player.has_reached_goal(&maze, block_size as f32) {
            game_state = GameState::Victory;
          }

          // Limpiar framebuffer al inicio del frame
          framebuffer.clear();

          // Renderizar escena
          if mode == "2D" {
              render_maze(&mut framebuffer, &maze, block_size, &player);
          } else {
              render_world(&mut framebuffer, &maze, block_size, &player, &texture_manager);
              render_minimap(&mut framebuffer, &maze, &player, block_size);
          }

          render_fps(&mut framebuffer, fps);
          framebuffer.swap_buffers(&mut window, &raylib_thread);
      }

      GameState::Victory => {
        let mut d = window.begin_drawing(&raylib_thread);
        menu::render_victory_screen(&mut d);
    
        if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
            game_state = GameState::Menu;
        }
      }
    }

    thread::sleep(Duration::from_millis(16));
  }
}
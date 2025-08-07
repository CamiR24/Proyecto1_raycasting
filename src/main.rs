mod framebuffer;
mod load_maze;
mod player;

use raylib::prelude::*;
use framebuffer::FrameBuffer;
use player::Player;

fn main() {
    let maze = load_maze::load_maze("./maze.txt");

    let player_start = player::find_player_position(&maze);

    let mut player = match player_start {
        Some((x, y)) => Player {
            pos: Vector2::new(x as f32 + 0.5, y as f32 + 0.5),
            a: 0.0,
        },
        None => Player {
            pos: Vector2::new(1.5, 1.5),
            a: 0.0,
        },
    };

    let maze_height = maze.len();
    let maze_width = if maze_height > 0 { maze[0].len() } else { 1 };

    let framebuffer_width = maze_width;
    let framebuffer_height = maze_height;

    let cell_size = 70; // Tamaño de cada celda en píxeles
    let window_width = framebuffer_width * cell_size;
    let window_height = framebuffer_height * cell_size;

    let (mut window, thread) = raylib::init()
        .size(window_width as i32, window_height as i32)
        .title("Raycasting")
        .build();

    window.set_target_fps(15);

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height, Color::WHITE);
    
    let block_size = 1;

    while !window.window_should_close() {
        framebuffer.clear();
        render_maze(&mut framebuffer, &maze, block_size);

        let player_x = player.pos.x as usize;
        let player_y = player.pos.y as usize;

        if player_x < framebuffer_width && player_y < framebuffer_height {
            framebuffer.point(player_x, player_y, Color::RED); // Jugador en rojo
        }
        
        let mut d = window.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        for y in 0..framebuffer_height {
            for x in 0..framebuffer_width {
                let color = framebuffer.get_color(x, y);
                let screen_x = (x * window_width / framebuffer_width) as i32;
                let screen_y = (y * window_height / framebuffer_height) as i32;
                let rect_width = (window_width / framebuffer_width) as i32;
                let rect_height = (window_height / framebuffer_height) as i32;
                
                d.draw_rectangle(screen_x, screen_y, rect_width, rect_height, color);
            }
        }
    }
}

fn draw_cell(
    framebuffer: &mut FrameBuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    let color = match cell {
        '+' | '-' | '|' => Color::BLACK, // Paredes
        'p' => Color::WHITE,             // Espacio donde estaba el jugador (ahora vacío)
        'g' => Color::GREEN,             // Meta
        ' ' => Color::WHITE,             // Camino
        _ => Color::GRAY,                // Otro
    };

    for dx in 0..block_size {
        for dy in 0..block_size {
            let x = xo + dx;
            let y = yo + dy;
            if x < framebuffer.width && y < framebuffer.height {
                framebuffer.point(x, y, color);
            }
        }
    }
}

pub fn render_maze(
    framebuffer: &mut FrameBuffer,
    maze: &Vec<Vec<char>>,
    block_size: usize,
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }
}

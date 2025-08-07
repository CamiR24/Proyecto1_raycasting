mod framebuffer;
mod load_maze;

use raylib::prelude::*;
use framebuffer::FrameBuffer;

fn main() {
    let maze = load_maze::load_maze("./maze.txt");

    // Obtener las dimensiones reales del laberinto
    let maze_height = maze.len();
    let maze_width = if maze_height > 0 { maze[0].len() } else { 1 };

    // Configurar el framebuffer con las dimensiones exactas del laberinto
    let framebuffer_width = maze_width;
    let framebuffer_height = maze_height;

    // Configurar el tamaño de la ventana (puedes ajustar este multiplicador)
    let cell_size = 70; // Tamaño de cada celda en píxeles
    let window_width = framebuffer_width * cell_size;
    let window_height = framebuffer_height * cell_size;

    let (mut window, thread) = raylib::init()
        .size(window_width as i32, window_height as i32)
        .title("Raycasting")
        .build();

    window.set_target_fps(15);

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height, Color::WHITE);
    
    // Cada celda del laberinto ocupará exactamente 1 píxel en el framebuffer
    let block_size = 1;

    while !window.window_should_close() {
        framebuffer.clear();
        render_maze(&mut framebuffer, &maze, block_size);
        
        let mut d = window.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        // Dibujar el framebuffer en la ventana
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
        'p' => Color::BLUE,              // Jugador
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

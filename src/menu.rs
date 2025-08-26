// menu.rs
use raylib::prelude::*;

pub fn render_menu(d: &mut RaylibDrawHandle) {
  // Fondo degradado
  for y in 0..d.get_screen_height() {
    let gradient_ratio = y as f32 / d.get_screen_height() as f32;
    let color = Color::new(
      (20.0 + gradient_ratio * 40.0) as u8,  // De azul oscuro
      (30.0 + gradient_ratio * 50.0) as u8,  // a púrpura
      (60.0 + gradient_ratio * 100.0) as u8,
      255
    );
    d.draw_line(0, y, d.get_screen_width(), y, color);
  }
  
  let title = "🧠 INSIDE OUT MAZE 🧠";
  let title_x = (d.get_screen_width() - measure_text(title, 60)) / 2;
  
  d.draw_text(title, title_x + 3, 53, 60, Color::new(0, 0, 0, 120));
  d.draw_text(title, title_x, 50, 60, Color::new(255, 215, 0, 255)); // Dorado
  
  let subtitle = "Explora los laberintos de la mente";
  let subtitle_x = (d.get_screen_width() - measure_text(subtitle, 30)) / 2;
  d.draw_text(subtitle, subtitle_x, 120, 30, Color::new(200, 200, 255, 255));
  
  let card_width = 350;
  let card_height = 400;
  let card_spacing = 30;
  let total_width = 3 * card_width + 2 * card_spacing;
  let start_x = (d.get_screen_width() - total_width) / 2;
  let card_y = 200;
  
  let characters = [
    ("INFANCIA", "Alegría", Color::new(255, 215, 0, 255), "[1]"),
    ("ADOLESCENCIA", "Ansiedad", Color::new(255, 140, 0, 255), "[2]"),
    ("ADULTEZ", "Nostalgia", Color::new(138, 43, 226, 255), "[3]"),
  ];
  
  for (i, (stage, emotion, color, key)) in characters.iter().enumerate() {
    let card_x = start_x + (i as i32) * (card_width + card_spacing);
    
    d.draw_rectangle(
      card_x + 8, card_y + 8, 
      card_width, card_height,
      Color::new(0, 0, 0, 60)
    );
    
    d.draw_rectangle(
      card_x, card_y,
      card_width, card_height,
      Color::new(40, 40, 70, 220)
    );
    
    d.draw_rectangle_lines(
      card_x, card_y,
      card_width, card_height,
      *color
    );
    
    d.draw_rectangle_lines(
      card_x + 2, card_y + 2,
      card_width - 4, card_height - 4,
      *color
    );
    
    let stage_x = card_x + (card_width - measure_text(stage, 28)) / 2;
    d.draw_text(stage, stage_x, card_y + 200, 28, *color);
    
    let emotion_text = format!("Emoción: {}", emotion);
    let emotion_x = card_x + (card_width - measure_text(&emotion_text, 20)) / 2;
    d.draw_text(&emotion_text, emotion_x, card_y + 240, 20, Color::new(200, 200, 200, 255));
    
    let key_bg_y = card_y + card_height - 60;
    let key_bg_width = 100;
    let key_bg_x = card_x + (card_width - key_bg_width) / 2;
    
    d.draw_rectangle(
      key_bg_x, key_bg_y,
      key_bg_width, 40,
      *color
    );
    
    let key_x = card_x + (card_width - measure_text(&format!("Presiona {}", key), 16)) / 2;
    d.draw_text(&format!("Presiona {}", key), key_x, key_bg_y + 12, 16, Color::WHITE);
  }
  
  let instructions = [
    "🎮 Controles: WASD para moverse, M para cambiar vista 2D/3D",
    "🎯 Objetivo: Encuentra la salida (zona gris) en cada laberinto"
  ];
  
  let inst_start_y = card_y + card_height + 40;
  for (i, instruction) in instructions.iter().enumerate() {
    let inst_x = (d.get_screen_width() - measure_text(instruction, 18)) / 2;
    let inst_y = inst_start_y + (i as i32) * 30;

    d.draw_text(instruction, inst_x + 1, inst_y + 1, 18, Color::new(0, 0, 0, 100));
    d.draw_text(instruction, inst_x, inst_y, 18, Color::new(220, 220, 255, 255));
  }
  
  draw_floating_particles(d);
}

fn draw_floating_particles(d: &mut RaylibDrawHandle) {
  //función decoración
  let time = unsafe { raylib::ffi::GetTime() } as f32;
  
  for i in 0..20 {
    let i_f = i as f32;
    let x = (d.get_screen_width() as f32 * 0.1 + 
             (time * 20.0 + i_f * 18.0).sin() * (d.get_screen_width() as f32 * 0.8)) as i32;
    let y = (50.0 + (time * 15.0 + i_f * 23.0).cos() * 30.0 + i_f * 25.0) as i32;
    
    let alpha = (128.0 + (time * 30.0 + i_f * 10.0).sin() * 127.0) as u8;
    let size = 2 + ((time * 25.0 + i_f * 15.0).sin() * 2.0) as i32;
    
    let colors = [
      Color::new(255, 215, 0, alpha),   // Dorado
      Color::new(255, 140, 0, alpha),   // Naranja
      Color::new(138, 43, 226, alpha),  // Púrpura
      Color::new(50, 205, 50, alpha),   // Verde
      Color::new(220, 20, 60, alpha),   // Rojo
    ];
    
    let color = colors[i % colors.len()];
    
    d.draw_circle(x, y, size as f32, color);
    d.draw_pixel(x - size, y, color);
    d.draw_pixel(x + size, y, color);
    d.draw_pixel(x, y - size, color);
    d.draw_pixel(x, y + size, color);
  }
}

// Función auxiliar para medir texto (necesaria para centrar)
fn measure_text(text: &str, font_size: i32) -> i32 {
  text.len() as i32 * (font_size / 2)
}

//pantalla de victoria 
pub fn render_victory_screen(d: &mut RaylibDrawHandle) {
  for y in 0..d.get_screen_height() {
    let gradient_ratio = y as f32 / d.get_screen_height() as f32;
    let color = Color::new(
      (10.0 + gradient_ratio * 30.0) as u8,
      (50.0 + gradient_ratio * 70.0) as u8,  
      (20.0 + gradient_ratio * 40.0) as u8,
      255
    );
    d.draw_line(0, y, d.get_screen_width(), y, color);
  }

  let title = "🎉 ¡FELICITACIONES! 🎉";
  let title_x = (d.get_screen_width() - measure_text(title, 50)) / 2;
  
  d.draw_text(title, title_x + 3, 253, 50, Color::new(0, 0, 0, 120));
  d.draw_text(title, title_x, 250, 50, Color::new(50, 205, 50, 255)); // Verde brillante
  
  let subtitle = "Has completado el laberinto emocional";
  let subtitle_x = (d.get_screen_width() - measure_text(subtitle, 30)) / 2;
  d.draw_text(subtitle, subtitle_x, 320, 30, Color::new(200, 255, 200, 255));
  
  let instruction = "Presiona [ENTER] para volver al menú";
  let inst_x = (d.get_screen_width() - measure_text(instruction, 25)) / 2;
  d.draw_text(instruction, inst_x + 2, 402, 25, Color::new(0, 0, 0, 100)); // Sombra
  d.draw_text(instruction, inst_x, 400, 25, Color::WHITE);
  
  draw_floating_particles(d);
}


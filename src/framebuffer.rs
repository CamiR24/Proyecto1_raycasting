use raylib::prelude::*;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
    background_color: Color,
    current_color: Color,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize, background_color: Color) -> Self {
        let pixels = vec![background_color; width * height];
        FrameBuffer {
            width,
            height,
            pixels,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(self.background_color);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn point(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }
    
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            Color::BLACK
        }
    }
    
    }
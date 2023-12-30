use std::{time::Instant, cmp::min};
use raylib::{drawing::{RaylibDrawHandle, RaylibDraw}, math::Rectangle, color::Color};

use crate::game_object::GameObject;

pub struct AnimatedTextBox {
    start_time: Instant,
    chars_per_second: u128,
    text: String,
    rectangle: Rectangle,
}  


impl GameObject for AnimatedTextBox {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        let font = d.get_font_default();
        let elapsed = self.start_time.elapsed();
        let solid_block = '\u{25D9}'.to_string(); // Convert char to string
        let num_chars_to_draw = min(((elapsed.as_millis()/100) * self.chars_per_second) as usize, self.text.len());
        let mut text_to_draw = String::from(&self.text[0..num_chars_to_draw]);
        text_to_draw.push_str(&solid_block);

        let border = 1.0;
        let border2 = 10.0;
        
        let rect_inner = Rectangle::new(
            self.rectangle.x + border, 
            self.rectangle.y + border, 
            self.rectangle.width - (border*2.0), 
            self.rectangle.height - (border*2.0));

        let text_box = Rectangle::new(
            self.rectangle.x + border2, 
            self.rectangle.y + border2, 
            self.rectangle.width - (border2*2.0), 
            self.rectangle.height - (border2*2.0));

        d.draw_rectangle_rounded(self.rectangle, 0.05, 5, Color::WHITE);
        d.draw_rectangle_rounded(rect_inner, 0.05,5, Color::BLACK);
        d.draw_text_rec(font, &text_to_draw, text_box, 10.0, 1.0, true, Color::GREEN);
    }

    fn update(&mut self, elapsed_time: f32) {
        
    }
}

impl AnimatedTextBox {
    pub fn new(text: String, chars_per_second: u128, rectangle: Rectangle) -> Self {
        AnimatedTextBox {
            start_time: Instant::now(),
            chars_per_second,
            text,
            rectangle
        }
    }
}
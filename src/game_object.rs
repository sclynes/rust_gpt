use raylib::drawing::RaylibDrawHandle;

pub trait GameObject {
    fn update(&mut self, elapsed_time: f32);
    fn draw(&self, d: &mut RaylibDrawHandle);
}
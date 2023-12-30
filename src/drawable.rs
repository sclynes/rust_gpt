use raylib::drawing::RaylibDrawHandle;

pub trait Drawable {
    fn draw(&self, d: &mut RaylibDrawHandle);
}
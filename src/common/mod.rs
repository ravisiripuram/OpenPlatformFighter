pub mod character;
pub mod constants;
pub mod animation;
pub mod collisonbox;
pub mod player;

pub trait Drawable {
    fn draw(&self);
}
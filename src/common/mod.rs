pub mod fighter;
pub mod animation;
pub mod framedata;
pub mod player;
pub mod state;
pub mod controls;
#[macro_use]
pub mod constants;

pub trait Drawable {
    fn draw(&self);
}
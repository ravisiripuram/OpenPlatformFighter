#![allow(dead_code)]

pub mod animation;
pub mod framedata;
pub mod state;
#[macro_use]
pub mod constants;

pub trait Drawable {
    fn draw(&self);
}
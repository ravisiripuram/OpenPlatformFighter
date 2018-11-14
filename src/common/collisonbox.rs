extern crate piston_window;
use piston_window::types::{Vec2d, Radius, Color};
use piston_window::{ellipse, Ellipse};
use piston_window::Graphics;
use piston_window::math::Matrix2d;
use common::constants::N_SIDES;

pub struct CollisionBox(pub Vec2d, pub Radius);

impl CollisionBox {
    pub fn draw<G: Graphics>(&self, c: Color, t: Matrix2d, g: &mut G) {
        Ellipse::new(c)
        .resolution(N_SIDES)
        .draw(
            ellipse::circle(self.0[0], self.0[1], self.1),
            &Default::default(), t, g
            )
    }
}

#[derive(Copy, Clone)]
pub struct FrameData<'a>(pub Option<&'a [&'a [CollisionBox]]>);

impl<'a> FrameData<'a> {
    pub fn draw<G: Graphics>(&self, f: usize, c: Color, t: Matrix2d, g: &mut G) {
        match self.0 {
            Some(d) => {
                for b in d[f] {
                    b.draw(c, t, g);
                }
            },
            None => {}
        }
    }
}


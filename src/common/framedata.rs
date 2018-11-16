use piston_window::types::{Color};
use piston_window::{ellipse, Ellipse};
use piston_window::Graphics;
use piston_window::math::Matrix2d;
use common::constants::N_SIDES;

#[derive(Copy, Clone)]
pub struct FrameData<'a>(pub &'a [&'a [[f64; 3]]]);

impl<'a> FrameData<'a> {
    pub fn draw<G: Graphics>(&self, f: usize, c: Color, t: Matrix2d, g: &mut G) {
        for b in self.0[f] {
            Ellipse::new(c)
            .resolution(N_SIDES)
            .draw(
                ellipse::circle(b[0], b[1], b[2]),
                &Default::default(), t, g
            );
        }
    }
}


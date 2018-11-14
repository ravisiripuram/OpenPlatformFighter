use common::character::Character;
use piston_window::types::{Vec2d, Radius, Color};
use piston_window::{Graphics, math::*};

pub struct Player<'a> {
    c: Character<'a>,
    pos: Vec2d,
}

impl<'a> Player<'a> {
    pub fn new(c: Character<'a>) -> Self {
        Player {
            c: c,
            pos: [100f64, 100f64]
        }
    }

    pub fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        self.c.draw(multiply(t, translate(self.pos)), g);
    }

    pub fn move_pos(&mut self, mv: Vec2d) {
        self.pos = add(self.pos, mv);
    }
}
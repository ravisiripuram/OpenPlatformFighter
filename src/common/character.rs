extern crate piston_window;
use piston_window::types::{Scalar, Color};
use piston_window::{Graphics, math::Matrix2d};
use std::ops::{Index, IndexMut, Add};
use common::animation::{AnimationState, Animation, N_ANIM_STATES};

type AnimationArray<'a> = [Animation<'a>; N_ANIM_STATES];

#[derive(Default)]
pub struct Character<'a> {
    aa: AnimationArray<'a>,

    astate: AnimationState,
    walkspeed: Scalar,
    fallspeed: Scalar,
    jumpheight: Scalar,
    // color: Color
}

impl<'a> Character<'a> {
    pub fn add_anim(&mut self, a: Animation<'a>) {
        // let s = a.state();
        self.aa[a.state() as usize] = a;
    }

    pub fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        self.aa[self.astate as usize].draw(t, g);
    }

    pub fn update(&mut self, active: bool) {
        self.aa[self.astate as usize].tick(active);
    }
}

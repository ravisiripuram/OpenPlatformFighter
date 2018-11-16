extern crate piston_window;
use piston_window::types::{Scalar, Color};
use piston_window::{Graphics, math::Matrix2d};
use std::ops::{Index, IndexMut, AddAssign};
use common::animation::{AnimationState, Animation, N_ANIM_STATES};

#[derive(Default)]
struct AnimationArray<'a>([Animation<'a>; N_ANIM_STATES]);
impl<'a> AddAssign<Animation<'a>> for AnimationArray<'a> {
    fn add_assign(&mut self, rhs: Animation<'a>) {
        self[rhs.state] = rhs;
    }
}
impl<'a> Index<AnimationState> for AnimationArray<'a> {
    type Output = Animation<'a>;
    fn index(&self, rhs: AnimationState) -> &Self::Output {
        &self.0[rhs as usize]
    }
}
impl<'a> IndexMut<AnimationState> for AnimationArray<'a> {
    fn index_mut(&mut self, rhs: AnimationState) -> &mut Self::Output {
        &mut self.0[rhs as usize]
    }
}


#[derive(Default)]
pub struct Fighter<'a> {
    aa: AnimationArray<'a>,
    astate: AnimationState,
    walkspeed: Scalar,
    fallspeed: Scalar,
    jumpheight: Scalar,
    // color: Color
}

impl<'a> Fighter<'a> {
    pub fn add_anim(&mut self, a: Animation<'a>) {
        self.aa += a;
    }

    pub fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        self.aa[self.astate].draw(t, g);
    }

    pub fn update(&mut self, active: bool) {
        self.aa[self.astate].tick(active);
    }
}

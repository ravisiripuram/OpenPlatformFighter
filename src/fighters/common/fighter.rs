extern crate piston_window;
use piston_window::types::{Scalar, Color};
use piston_window::{Graphics, math::Matrix2d};
use std::ops::{Index, IndexMut, AddAssign};
use common::animation::{AnimationState, Animation, N_ANIM_STATES};
use common::framedata::FrameData;

#[derive(Default)]
pub struct AnimationArray<'a>([Animation<'a>; N_ANIM_STATES]);
impl<'a> AddAssign<Animation<'a>> for AnimationArray<'a> {
    fn add_assign(&mut self, rhs: Animation<'a>) {
        self[rhs.state()] = rhs;
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

pub struct Fighter<'a> {
    pub aa: AnimationArray<'a>,
    pub astate: AnimationState,
    pub walkspeed: Scalar,
    pub fallspeed: Scalar,
    pub jumpheight: Scalar,
    //pub color: Color
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
    pub fn setState(&mut self, state: AnimationState) {
        self.astate = state;
    }
}
impl<'a> Default for Fighter<'a> {
    fn default() -> Self {
        let mut aa: AnimationArray = Default::default(); 
        aa += Animation {state: AnimationState::Idle, hurtboxes:Some(FrameData::default()), ..Default::default()};
        // aa += Animation {state: AnimationState::Idle, hitboxes:Some(FrameData::default()), ..Default::default()};
        // aa += Animation {state: AnimationState::Idle, grabboxes:Some(FrameData::default()), ..Default::default()};
        Fighter {
            aa: aa,
            astate: AnimationState::Idle,
            walkspeed: 10.0,
            fallspeed: 5.0,
            jumpheight: 25.0,
        }
    }
}

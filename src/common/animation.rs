extern crate piston_window;
use piston_window::{Graphics, math::Matrix2d};
use common::framedata::{FrameData};
use common::constants::{HIT_BOX_COLOR, HURT_BOX_COLOR, GRAB_BOX_COLOR};

pub const N_ANIM_STATES: usize = 3;

#[derive(Copy, Clone)]
pub enum AnimationState {
    Idle,
    Walk,
    Jump,
}
impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::Idle
    }
}

#[derive(Copy, Clone)]
pub struct Animation<'a> {
    pub state: AnimationState,
    pub hurtboxes: Option<(FrameData<'a>)>,
    pub hitboxes: Option<(FrameData<'a>)>,
    pub grabboxes: Option<(FrameData<'a>)>,
}
impl<'a> Animation<'a> {
    pub fn state(&self) -> AnimationState {
        self.state
    }
    pub fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        if let Some(d) = self.hurtboxes {
            d.draw(HURT_BOX_COLOR, t, g);
        }
        if let Some(d) = self.hitboxes {
            d.draw(HIT_BOX_COLOR, t, g);
        }
        if let Some(d) = self.grabboxes {
            d.draw(GRAB_BOX_COLOR, t, g);
        }
    }
    pub fn tick(&mut self, a: bool) {
        if let Some(ref mut d) = self.hurtboxes {
            d.tick(a);
        }
        if let Some(ref mut d) = self.hitboxes {
            d.tick(a);
        }
        if let Some(ref mut d) = self.grabboxes {
            d.tick(a);
        }
    }
}
impl<'a> Default for Animation<'a> {
    fn default() -> Self {
        Animation {
            state: AnimationState::Idle,
            hurtboxes: None,
            hitboxes: None,
            grabboxes: None,
        }
    }
}

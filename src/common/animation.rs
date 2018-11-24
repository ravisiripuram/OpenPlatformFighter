use std::fmt;
use std::fmt::Display;
use piston_window::{Graphics, math::Matrix2d};
use common::frame::*;

pub const N_ANIM_STATES: usize = 3;

#[derive(Copy, Clone, PartialEq, Display)]
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

#[derive(Clone)]
pub struct Animation<'a> {
    pub state: AnimationState,
    pub partials: Vec<FrameData<'a>>,
    pub frametypes: Vec<FrameType>,
    pub num_partials: usize,
    pub cur_partial: usize
}
impl<'a> Animation<'a> {
    pub fn new(state: AnimationState, partials: Vec<FrameData<'a>>, frametypes: Vec<FrameType>) -> Self {
        let np = partials.len();
        if np != frametypes.len() {panic!("frametypes len and partials len must be equal");}
        Animation {
            state: state,
            partials: partials,
            frametypes: frametypes,
            num_partials: np,
            cur_partial: 0
        }
    }
    fn next_partial(&mut self) -> bool {
        if self.cur_partial + 1 < self.num_partials {
            self.cur_partial += 1;
            return true;
        }
        return false;
    }
    pub fn state(&self) -> AnimationState {
        self.state
    }
    pub fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        self.partials[self.cur_partial].draw(self.frametypes[self.cur_partial].cur_frame(), t, g);
    }
    pub fn tick(&mut self, active: bool) {
        match self.frametypes[self.cur_partial] {
            FrameType::Single(_) => {
                self.frametypes[self.cur_partial].tick();
                if self.frametypes[self.cur_partial].done() {
                    self.next_partial();
                }
            },
            FrameType::Repeat(_) => {
                self.frametypes[self.cur_partial].tick();
                if !active {
                    self.next_partial();
                }
            },
        }
    }
    pub fn reset(&mut self) {
        self.frametypes[self.cur_partial].reset();
        self.cur_partial = 0;
    }
    pub fn done(&self) -> bool {
        self.frametypes[self.cur_partial].done()
    }
    pub fn interruptable(&self) -> bool {
        self.frametypes[self.cur_partial].interruptable()
    }
}
impl<'a> fmt::Debug for Animation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {} | {} | {}", self.state, self.cur_partial, self.frametypes[self.cur_partial], self.frametypes[self.cur_partial].cur_frame())
    }
}
impl<'a> Default for Animation<'a> {
    fn default() -> Self {
        Animation::new(
            AnimationState::Idle,
            vec![FrameData(&[&[Frame::NoBox],])],
            vec![FrameType::default()],
        )
    }
}

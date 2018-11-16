extern crate piston_window;
use piston_window::{Graphics, math::Matrix2d};
use common::framedata::{FrameData};
use common::constants::{HIT_BOX_COLOR, HURT_BOX_COLOR};

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
pub enum FrameState {
    Loop(LoopFrameState),
    Once(OnceFrameState)
}

impl Default for FrameState {
    fn default() -> FrameState {
        FrameState::Once(Default::default())
    }
}

impl FrameState {
    pub fn tick(&mut self, active: bool) -> usize{
        match self {
            FrameState::Once(a) => {a.cur_frame = if active {(a.cur_frame + 1) % a.end_frame} else {a.cur_frame}; a.cur_frame},
            FrameState::Loop(a) => {
                if active && a.cur_frame < a.loo_frame { a.cur_frame + 1; }
                if active && a.cur_frame >= a.loo_frame { a.cur_frame = a.beg_frame + 1; } 
                if !active && a.cur_frame >= a.loo_frame { a.cur_frame + 1; }
                if a.cur_frame > a.end_frame { a.cur_frame = 0; }
                a.cur_frame
            },
        }
    }
    pub fn cur_frame(&self) -> usize{
        match self {
            FrameState::Once(a) => a.cur_frame,
            FrameState::Loop(a) => a.cur_frame
        }
    }
    pub fn reset(&mut self) {
        match self {
            FrameState::Loop(f) => f.cur_frame = 0,
            FrameState::Once(f) => f.cur_frame = 0
        };
    }

}

#[derive(Copy, Clone, Default)]
pub struct OnceFrameState {
    cur_frame: usize,
    beg_frame: usize,
    end_frame: usize,
}

#[derive(Copy, Clone, Default)]
pub struct LoopFrameState {
    cur_frame: usize,
    loo_frame: usize,
    beg_frame: usize,
    end_frame: usize,
}

#[derive(Copy, Clone)]
pub struct Animation<'a> {
    pub state: AnimationState,
    hurtboxes: Option<FrameData<'a>>,
    hitboxes: Option<FrameData<'a>>,
    frame_state: FrameState,
}

impl<'a> Animation<'a> {
    pub fn state(&self) -> AnimationState {
        self.state
    }
    pub fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        let cf = self.frame_state.cur_frame();

        if let Some(d) = self.hurtboxes {
            d.draw(cf, HURT_BOX_COLOR, t, g);
        }
        if let Some(d) = self.hitboxes {
            d.draw(cf, HIT_BOX_COLOR, t, g);
        }
    }
    pub fn tick(&mut self, a: bool) {
        self.frame_state.tick(a);
    }
}

impl<'a> Default for Animation<'a> {
    fn default() -> Self {
        Animation {
            state: AnimationState::Idle,
            hurtboxes: Some(FrameData(&[
                &[[0.0, 0.0, 10.0],],
                &[[0.0, 0.0, 10.5],],
                &[[0.0, 0.0, 11.0],],
                &[[0.0, 0.0, 12.0],],
                &[[0.0, 0.0, 13.5],],
                &[[0.0, 0.0, 14.5],],
                &[[0.0, 0.0, 15.0],],
                &[[0.0, 0.0, 15.0],],
                &[[0.0, 0.0, 12.0],],
                &[[0.0, 0.0, 12.0],],
                &[[0.0, 0.0, 12.0],],
                &[[0.0, 0.0, 10.0],],
                &[[0.0, 0.0, 10.0],],
                &[[0.0, 0.0, 10.0],],
            ])),
            hitboxes: None,
            frame_state: FrameState::Once(OnceFrameState{
                end_frame: 13,
                ..Default::default()
            })
        }
    }
}

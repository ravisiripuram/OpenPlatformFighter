use std::fmt::Display;
use piston_window::{Graphics, math::Matrix2d, types::Color, ellipse, Ellipse};
use common::constants::{HIT_BOX_COLOR, HURT_BOX_COLOR, GRAB_BOX_COLOR, N_SIDES};

pub const N_ANIM_STATES: usize = 3;

#[derive(Copy, Clone, Default)]
pub struct SingleFrameState {
    cur_frame: usize,
    end_frame: usize,
    faf_frame: usize,
}
impl SingleFrameState {
    pub fn new(end_frame: usize, faf_frame: usize) -> Self {
        SingleFrameState {
            cur_frame: 0,
            end_frame: end_frame,
            faf_frame: faf_frame,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct DoubleFrameState {
    cur_frame: usize,
    beg_frame: usize,
    loo_frame: usize,
    end_frame: usize,
    faf_frame: usize,
}
impl DoubleFrameState {
    pub fn new(end_frame: usize, beg_frame: usize, loo_frame: usize, faf_frame: usize) -> Self {
        DoubleFrameState {
            cur_frame: 0,
            beg_frame: beg_frame,
            loo_frame: loo_frame,
            end_frame: end_frame,
            faf_frame: faf_frame,
        }
    }
}

#[derive(Copy, Clone)]
pub enum FrameState {
    Always(SingleFrameState),
    Active(SingleFrameState),
    Looped(DoubleFrameState),
}
impl FrameState {
    pub fn always(end_frame: usize, faf_frame: usize) -> Self {
        FrameState::Always(SingleFrameState::new(end_frame, faf_frame))
    }
    pub fn active(end_frame: usize, faf_frame: usize) -> Self {
        FrameState::Active(SingleFrameState::new(end_frame, faf_frame))
    }
    pub fn looped(end_frame: usize, beg_frame: usize, loo_frame: usize, faf_frame: usize) -> Self {
        FrameState::Looped(DoubleFrameState::new(end_frame, beg_frame, loo_frame, faf_frame))
    }
}
impl Default for FrameState {
    fn default() -> FrameState {
        FrameState::Always(Default::default())
    }
}

#[derive(Copy, Clone)]
pub struct FrameData<'a>(pub &'a [&'a [[f64; 3]]], pub FrameState);
impl<'a> FrameData<'a> {
    pub fn draw<G: Graphics>(&self, c: Color, t: Matrix2d, g: &mut G) {
        for b in self.0[self.cur_frame()] {
            Ellipse::new(c).resolution(N_SIDES)
            .draw(
                ellipse::circle(b[0], b[1], b[2]),
                &Default::default(), t, g
            );
        }
    }
    pub fn tick(&mut self, active: bool) {
        match self.1 {
            FrameState::Always(ref mut a) => { a.cur_frame = (a.cur_frame + 1) % a.end_frame; },
            FrameState::Active(ref mut a) => { a.cur_frame = (a.cur_frame + 1) % a.end_frame; },
            FrameState::Looped(ref mut a) => {
                if  active && a.cur_frame  < a.loo_frame { a.cur_frame = a.cur_frame + 1; }
                if  active && a.cur_frame >= a.loo_frame { a.cur_frame = a.beg_frame + 1; }
                if !active { a.cur_frame = (a.cur_frame + 1) % a.end_frame; }
            },
        }
        // println!("{}", self.cur_frame())
    }
    pub fn cur_frame(&self) -> usize {
        match self.1 {
            FrameState::Always(a) => a.cur_frame,
            FrameState::Active(a) => a.cur_frame,
            FrameState::Looped(a) => a.cur_frame,
        }
    }
    pub fn reset(&mut self) {
        match self.1 {
            FrameState::Always(ref mut f) => f.cur_frame = 0,
            FrameState::Active(ref mut f) => f.cur_frame = 0,
            FrameState::Looped(ref mut f) => f.cur_frame = 0,
        };
    }
    pub fn done(&self) -> bool {
        match self.1 {
            FrameState::Always(a) => a.cur_frame >= a.end_frame - 1,
            FrameState::Active(a) => a.cur_frame >= a.end_frame - 1,
            FrameState::Looped(a) => a.cur_frame >= a.end_frame - 1,
        }
    }
    pub fn interruptable(&self) -> bool {
        match self.1 {
            FrameState::Always(a) => a.cur_frame >= a.faf_frame,
            FrameState::Active(a) => a.cur_frame >= a.faf_frame,
            FrameState::Looped(a) => a.cur_frame >= a.faf_frame,
        }
    }
}
impl<'a> Default for FrameData<'a> {
    fn default() -> Self {
        FrameData(
            &[
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
            ],
            FrameState::Active(
                SingleFrameState{
                    end_frame: 13,
                    ..Default::default()
                }
            )
        )
    }
}

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

#[derive(Copy, Clone)]
pub struct Animation<'a> {
    pub state: AnimationState,
    pub hurtboxes: Option<(FrameData<'a>)>,
    pub hitboxes:  Option<(FrameData<'a>)>,
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
    pub fn reset(&mut self) {
        if let Some(ref mut d) = self.hurtboxes {
            d.reset();
        }
        if let Some(ref mut d) = self.hitboxes {
            d.reset();
        }
        if let Some(ref mut d) = self.grabboxes {
            d.reset();
        }
    }
    pub fn done(&self) -> bool {
        let mut out: bool = true;
        if let Some(d) = self.hurtboxes {
            out = out && d.done();
        }
        if let Some(d) = self.hitboxes {
            out = out && d.done();
        }
        if let Some(d) = self.grabboxes {
            out = out && d.done();
        }
        out
    }
    pub fn interruptable(&self) -> bool {
        let mut out: bool = true;
        if let Some(d) = self.hurtboxes {
            out = out && d.interruptable();
        }
        if let Some(d) = self.hitboxes {
            out = out && d.interruptable();
        }
        if let Some(d) = self.grabboxes {
            out = out && d.interruptable();
        }
        out
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

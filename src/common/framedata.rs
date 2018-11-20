use piston_window::types::{Color};
use piston_window::{ellipse, Ellipse};
use piston_window::Graphics;
use piston_window::math::Matrix2d;
use common::constants::N_SIDES;

#[derive(Copy, Clone, Default)]
pub struct SingleFrameState {
    cur_frame: usize,
    end_frame: usize,
}

#[derive(Copy, Clone, Default)]
pub struct DoubleFrameState {
    cur_frame: usize,
    loo_frame: usize,
    beg_frame: usize,
    end_frame: usize,
}

#[derive(Copy, Clone)]
pub enum FrameState {
    Always(SingleFrameState),
    Active(SingleFrameState),
    Looped(DoubleFrameState),
    
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
            FrameState::Active(ref mut a) => { a.cur_frame = if active {(a.cur_frame + 1) % a.end_frame} else {a.cur_frame}; },
            FrameState::Looped(ref mut a) => {
                if  active && a.cur_frame  < a.loo_frame { a.cur_frame = a.cur_frame + 1; }
                if  active && a.cur_frame >= a.loo_frame { a.cur_frame = a.beg_frame + 1; } 
                if !active && a.cur_frame >= a.loo_frame { a.cur_frame = a.cur_frame + 1; }
                if a.cur_frame > a.end_frame { a.cur_frame = 0; }
            },
        }
        println!("{}", self.cur_frame())
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


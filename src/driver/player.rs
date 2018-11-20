use fighters::common::fighter::Fighter;
use driver::controls::*;
use common::constants::*;
use common::state::*;
use piston_window::types::{Vec2d};
use piston_window::{Graphics, Button, ButtonArgs, ButtonState, math::*};

#[macro_export]
macro_rules! mag {
    ($x:expr) => {
        {
            ($x[0].powi(2) + $x[1].powi(2)).powf(0.5)
        }
    };
}

pub struct Player<'a> {
    f: Fighter<'a>,
    c: Controls,
    pos: Vec2d,
    vel: Vec2d,
    is: State,
    vs: State,
}

impl<'a> Player<'a> {
    pub fn new(f: Fighter<'a>, c: Controls) -> Self {
        Player {
            f: f,
            c: c,
            pos: [100.0, 100.0],
            vel: [  0.0,   0.0],
            is: State::new(),
            vs: State::new(),
        }
    }

    pub fn update(&mut self) {
        self.vel = [ 0.0,   0.0 ];
        if self.is.is_on(IVal::LInput) {
            self.vel = add(self.vel, LVEC);
        }
        if self.is.is_on(IVal::RInput) {
            self.vel = add(self.vel, RVEC);
        }
        if self.is.is_on(IVal::JInput) {
            self.vel = add(self.vel, UVEC);
        }
        if self.is.is_on(IVal::DInput) {
            self.vel = add(self.vel, DVEC);
        }
        let sl = mag!(self.vel);
        if sl != 0.0 {
            self.vel = mul_scalar(self.vel, self.f.walkspeed/sl);
        }
        // println!("{:?}, {:?}, {:?}", self.pos, self.vel, sl);
        self.move_pos();
        self.f.update(self.is.any());
    }

    pub fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        self.f.draw(multiply(t, translate(self.pos)), g);
    }

    fn move_pos(&mut self) {
        self.pos = add(self.pos, self.vel);
    }

    pub fn update_inputs(&mut self, b: &ButtonArgs) {
        self.update_istate(b);
        // println!("{}", self.is);
        self.update_vstate();        
    }

    fn update_istate(&mut self, b: &ButtonArgs) {
        if let Button::Keyboard(k) = b.button {
            if self.c.contains_key(&k) {
                let u = self.c[&k];
                // println!("{: >5}: {:021b}", format!("{:?}", k), u);
                match b.state {
                    ButtonState::Press => {
                        self.is += u;
                    },
                    ButtonState::Release => {
                        self.is -= u;
                    }
                }
            }
        }
    }

    fn update_vstate(&mut self) {

    }

}
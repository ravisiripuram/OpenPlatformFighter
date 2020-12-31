use piston_window::types::{Vec2d};
use piston_window::{Graphics, Button, ButtonArgs, ButtonState, math::*};

use driver::controls::*;
use common::{state::*, animation::AnimationState, constants::*, fighter::*};

pub struct Player<'a> {
    f: Fighter<'a>,
    c: Controls,
    is: State,
    pub vs: State,
    pub pos: Vec2d,
    vel: Vec2d,
    fvel: Vec2d,
    acc: Vec2d,
    jt: (f64, bool),
}

impl<'a> Player<'a> {
    pub fn new(f: Fighter<'a>, c: Controls) -> Self {
        Player {
            f: f,
            c: c,
            is: State::new(IVal::NoInput.into()),
            vs: State::new(VVal::Grounded.into()),
            pos:  [100.0, 100.0],
            vel:  [  0.0,   0.0],
            fvel: [  0.0,   0.0],
            acc:  [  0.0,   0.0],
            jt:   (0.0, false),
        }
    }
    pub fn update(&mut self, dt: f64) {
        // println!("{:?}", self.is);
        // println!("{:?}", self.vs);
        // println!("{:?}", self.f.aa[self.f.astate]);

        self.f.update(self.is.any());
        if self.is.is_on(IVal::SInput) {
            self.pos = [100.0, 100.0];
            self.vel = [  0.0,   0.0];
            self.fvel = [  0.0,   0.0];
        }
        // let last_astate = self.f.astate;
        if self.is.is_on(IVal::LInput | (IVal::RInput | IVal::DInput)) {
            self.f.set_astate(AnimationState::Walk, true, self.is.rising(IVal::LInput | (IVal::RInput | IVal::DInput)));
        }
        if self.is.rising(IVal::JInput) {
            // println!("rising: {}", self.is);
            if self.f.set_astate(AnimationState::Jump, true, self.is.rising(IVal::JInput)) {
                self.vel = mul([1.0, 0.0], self.vel);
                self.jt.1 = true;
            }
        }
        if self.is.rising(IVal::AInput) {
            // println!("rising: {}", self.is);
            self.f.set_astate(AnimationState::Jab, true, self.is.rising(IVal::AInput));
        }
        if !self.is.any() || (self.is.is_on(IVal::JInput) && !self.is.rising(IVal::JInput)) {
            self.f.set_astate(AnimationState::Idle, false, false);
        }

        self.update_vel(dt);
        self.move_pos(dt);
        // if last_astate != self.f.astate { println!("{}, {}", last_astate, self.f.astate); }
        // self.update_vstate();
        self.is.update();
        self.vs.update();
    }
    pub fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        self.f.draw(multiply(t, translate(self.pos)), g);
    }
    fn move_pos(&mut self, dt: f64) {
        self.pos = add(self.pos, mul_scalar(self.vel, dt));
    }
    fn update_vel(&mut self, dt: f64) {
        self.vel = [0.0, 0.0];
        let mut wvel = [0.0, 0.0];
        if self.is.is_on(IVal::LInput) {
            wvel = add(wvel, LVEC);
            self.vs += VVal::FacingLeft;
        }
        if self.is.is_on(IVal::RInput) {
            wvel = add(wvel, RVEC);
            self.vs -= VVal::FacingLeft;
        }
        if self.is.is_on(IVal::DInput) {
            wvel = add(wvel, DVEC);
        }
        wvel = mul_scalar(wvel, self.f.walkspeed);
        if self.jt.1 {
            self.jt.0 += dt;
            wvel = add(wvel, mul_scalar(UVEC, self.f.jumpspeed*dt));
        }
        if self.jt.0 >= self.f.jumpheight/self.f.jumpspeed {
            self.jt.0 = 0.0;
            self.jt.1 = false;
            self.fvel = mul_scalar(DVEC, self.f.init_fallspeed);
        }

        if self.vs.is_on(VVal::Grounded) {
            self.fvel = [0.0, 0.0];
        } else {
            if self.vs.falling(VVal::Grounded) || self.is.rising(IVal::JInput) {
                // println!("fell, {:?}", self.is);
                self.fvel = mul_scalar(DVEC, self.f.init_fallspeed);
            }
            if self.fvel[1] < self.f.max_fallspeed {
                self.fvel = add(self.fvel, mul_scalar(DVEC, self.f.weight*dt));
            }
        }
        self.vel = add(add(self.vel, wvel), self.fvel);
        // println!("vel:  {:?}", self.vel);
        // println!("jt:   {:?}", self.jumptime);
        // println!("fvel: {:?}", self.fall_vel);
    }
    pub fn update_inputs(&mut self, b: &ButtonArgs) {
        if let Button::Keyboard(k) = b.button {
            if self.c.contains_key(&k) {
                let u = self.c[&k];
                // println!("{: >5}: {:021b}", format!("{:?}", k), u);
                match b.state {
                    ButtonState::Press => {
                        self.is += u;
                        // println!("added:   {:032b}, {:?}", u, k);
                    },
                    ButtonState::Release => {
                        self.is -= u;
                        // println!("removed: {:032b}, {:?}", u, k);
                    }
                }
            }
        }
    }
    fn update_vstate(&mut self) {
        self.vs -= VVal::Grounded;
    }
}
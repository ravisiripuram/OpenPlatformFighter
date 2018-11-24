use piston_window::types::{Vec2d};
use piston_window::{Graphics, Button, ButtonArgs, ButtonState, math::*};

use driver::controls::*;
use common::{state::*, animation::AnimationState, constants::*, fighter::*};

pub struct Player<'a> {
    f: Fighter<'a>,
    c: Controls,
    pos: Vec2d,
    vel: Vec2d,
    fall_vel: Vec2d,
    acc: Vec2d,
    jumptime: (f64, bool),
    is: State,
    vs: State,
    fc: u32,
}

impl<'a> Player<'a> {
    pub fn new(f: Fighter<'a>, c: Controls) -> Self {
        Player {
            fc: 0,
            f: f,
            c: c,
            pos: [100.0, 100.0],
            vel: [  0.0,   0.0],
            fall_vel: [  0.0,   0.0],
            acc: [  0.0,   0.0],
            jumptime: (0.0, false),
            is: State::new(IVal::NoInput.into()),
            vs: State::new(VVal::Grounded.into()),
        }
    }
    pub fn update(&mut self, dt: f64) {
        // println!("{:?}", self.is);
        println!("{:?}", self.f.aa[self.f.astate]);
        if self.is.is_on(IVal::SInput) {
            self.pos = [100.0, 100.0];
            self.vel = [  0.0,   0.0];
            self.fall_vel = [  0.0,   0.0];
        }
        // let last_astate = self.f.astate;
        if self.is.is_on(IVal::LInput | (IVal::RInput | IVal::DInput)) {
            self.f.set_astate(AnimationState::Walk, true, self.is.rising(IVal::LInput | (IVal::RInput | IVal::DInput)));
            self.f.update(true);
        }
        if self.is.rising(IVal::JInput) {
            // println!("rising: {}", self.is);
            if self.f.set_astate(AnimationState::Jump, true, self.is.rising(IVal::JInput)) {
                self.vel = mul([1.0, 0.0], self.vel);
                self.jumptime.1 = true;
            }
            self.f.update(true);
        }
        if !self.is.any() || (self.is.is_on(IVal::JInput) && !self.is.rising(IVal::JInput)) {
            self.f.set_astate(AnimationState::Idle, false, false);
            self.f.update(self.is.any());
        }

        self.update_vel(dt);
        self.move_pos(dt);
        // if last_astate != self.f.astate { println!("{}, {}", last_astate, self.f.astate); }
        self.fc += (dt*FRAMES_PER_SECOND as f64) as u32;
        self.fc %= 30;
        // println!("frame: {:}", self.fc);
        self.update_vstate();
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
        if self.jumptime.1 {
            self.jumptime.0 += dt;
            wvel = add(wvel, mul_scalar(UVEC, self.f.jumpspeed*dt));
        }
        if self.jumptime.0 >= self.f.jumpheight/self.f.jumpspeed {
            self.jumptime.0 = 0.0;
            self.jumptime.1 = false;
            self.fall_vel = mul_scalar(DVEC, self.f.init_fallspeed);
        }

        if self.vs.is_on(VVal::Grounded) {
            self.fall_vel = [0.0, 0.0];
        } else {
            if self.vs.falling(VVal::Grounded) || self.is.rising(IVal::JInput) {
                // println!("fell, {:?}", self.is);
                self.fall_vel = mul_scalar(DVEC, self.f.init_fallspeed);
            }
            if self.fall_vel[1] < self.f.max_fallspeed {
                self.fall_vel = add(self.fall_vel, mul_scalar(DVEC, self.f.weight*dt));
            }
        }

        self.vel = add(add(self.vel, wvel), self.fall_vel);
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
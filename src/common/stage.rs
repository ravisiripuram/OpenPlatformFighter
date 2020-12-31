use piston_window::{Graphics, math::*, types::{Color, Line, Polygon, Vec2d}, line, polygon};
use common::constants::*;
use common::state::VVal;
use driver::player::Player;

pub struct Platform<'a>(pub Polygon<'a>, pub Color);
impl<'a> Platform<'a> {
    pub fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        polygon(self.1, self.0, t, g);
    }
}

pub struct LightPlatform(pub Line, pub Color);
impl LightPlatform {
    pub fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        line(self.1, LIGHTPLATFORM_RADIUS, self.0, t, g);
    }
}

pub struct Stage<'a> {
    pub pos: Vec2d,
    pub platforms: Vec<Platform<'a>>,
    pub lightplatforms: Option<Vec<LightPlatform>>,
}
impl<'a> Stage<'a> {
    pub fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        for p in self.platforms.iter() {
            p.draw(multiply(t, translate(self.pos)), g);
        }
        if let Some(ref lps) = self.lightplatforms {
            for lp in lps.iter() {
                lp.draw(multiply(t, translate(self.pos)), g);
            }
        }
    }

    pub fn collision_check(&self, v: Vec2d) -> bool {
        for p in self.platforms.iter() {
            if (v[0] >= p.0[0][0] + self.pos[0] && v[0] <= p.0[1][0] + self.pos[0]) 
            && (v[1] >= p.0[0][1] + self.pos[1] && v[1] <= p.0[2][1] + self.pos[1]) {
                return true;
            }
        }
        return false;
    }
    
    pub fn update_player(&self, p: &mut Player) {
        if self.collision_check(p.pos) {
            p.vs += VVal::Grounded
        } else {
            p.vs -= VVal::Grounded
        }
    }
    
}
impl<'a> Default for Stage<'a> {
    fn default() -> Self {
        const W: f64 = 200.0;
        const H: f64 = 30.0;
        Stage {
            pos: [(WINDOW_SIZE.0/2.0)-W/2.0, (WINDOW_SIZE.1*0.75)-H/2.0],
            platforms: vec![
                Platform(&[
                    [0.0, 0.0],
                    [W, 0.0],
                    [W, H],
                    [0.0, H],
                ], 
                PLATFORM_COLOR
                )
            ],
            lightplatforms: None,
        }
    }
}
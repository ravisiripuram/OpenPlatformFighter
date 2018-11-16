use piston_window::types::Color as Color;

pub const HURT_BOX_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
pub const HIT_BOX_COLOR: Color  = [0.0, 1.0, 0.0, 1.0];
pub const GRAB_BOX_COLOR: Color = [0.0, 1.0, 0.0, 1.0];

pub const N_SIDES: u32 = 12;
pub const FRAMES_PER_SECOND: u64 = 30;

pub const RVEC : [f64; 2] = [  1.0,  0.0 ];
pub const LVEC : [f64; 2] = [ -1.0,  0.0 ];
pub const UVEC : [f64; 2] = [  0.0, -1.0 ];
pub const DVEC : [f64; 2] = [  0.0,  1.0 ];

// #[macro_export]
// macro_rules! mag {
//     ($x:expr) => {
//         {
//             ($x[0].powi(2) + $x[1].powi(2)).powf(0.5)
//         }
//     };
// }
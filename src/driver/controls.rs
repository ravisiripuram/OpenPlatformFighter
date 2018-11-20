use std::collections::HashMap;
use piston_window::{Key};
use common::state::IVal;

pub type Controls = HashMap<Key, u32>; 

pub fn controls1() -> Controls {
    [
        (Key::A,     IVal::LInput as u32),
        (Key::D,     IVal::RInput as u32),
        (Key::S,     IVal::DInput as u32),
        (Key::W,     IVal::JInput | IVal::UInput),
        (Key::O,     IVal::BInput as u32),
        (Key::I,     IVal::AInput as u32),
        (Key::Space, IVal::SInput as u32),
        (Key::J,     IVal::ZInput as u32),
    ].iter().cloned().collect()
}

pub fn controls2() -> Controls {
    [
        (Key::Left,  IVal::LInput as u32),
        (Key::Right, IVal::RInput as u32),
        (Key::Down,  IVal::DInput as u32),
        (Key::Up,    IVal::JInput as u32),
        // (Key::W,     IVal::UInput),
    ].iter().cloned().collect()
}
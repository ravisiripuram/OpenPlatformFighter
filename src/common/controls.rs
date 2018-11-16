use std::collections::HashMap;
use piston_window::{Key};
use common::state::IVal;

pub type Controls = HashMap<Key, IVal>; 

pub fn controls1() -> Controls {
    [
        (Key::A,     IVal::LInput),
        (Key::D,     IVal::RInput),
        (Key::S,     IVal::DInput),
        (Key::W,     IVal::JInput),
        (Key::W,     IVal::UInput),
        (Key::O,     IVal::BInput),
        (Key::I,     IVal::AInput),
        (Key::Space, IVal::SInput),
        (Key::J,     IVal::ZInput),
    ].iter().cloned().collect()
}

pub fn controls2() -> Controls {
    [
        (Key::Left,  IVal::LInput),
        (Key::Right, IVal::RInput),
        (Key::Down,  IVal::DInput),
        (Key::Up,    IVal::JInput),
        // (Key::W,     IVal::UInput),
    ].iter().cloned().collect()
}
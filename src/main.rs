extern crate piston_window;
extern crate button_tracker;
use piston_window::*;
mod common;
use common::constants::*;
use common::player::*;
use common::controls::*;

fn main() {
    //init
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (320, 240))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    
    window = window.ups(FRAMES_PER_SECOND);
    let mut p1 = Player::new(Default::default(), controls1());

    //game loop
    while let Some(e) = window.next() {
        match e {
            Event::Input(i) => {
                match i {
                    Input::Button(b) => {
                        p1.update_inputs(&b)
                    },
                    _ => {}
                }
            },

            Event::Loop(l) => {
                match l {
                    Loop::Render(_r) => {

                        window.draw_2d(&e, |c, g| {
                            clear([0.5, 1.0, 0.5, 1.0], g);
                            p1.draw(c.transform, g);
                        });
                    },
                    Loop::Update(_u) => {
                        p1.update();
                    },
                    _ => {}
                }

            }
            _ => {}
        }
    }
}
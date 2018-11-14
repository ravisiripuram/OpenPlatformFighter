extern crate piston_window;
use piston_window::*;

mod common;
use common::*;
use common::constants::*;
use common::player::*;
use common::collisonbox::*;

fn main() {
    //init
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let p1 = Player::new(Default::default());

    //game loop
    while let Some(e) = window.next() {
        match e {
            Event::Loop(l) => {
                match l {
                    Loop::Render(_r) => {

                        window.draw_2d(&e, |c, g| {
                            clear([0.5, 1.0, 0.5, 1.0], g);
                            p1.draw(c.transform, g);
                        });
                    }

                    _ => {}
                }

            }

            Event::Input(i) => {
                match i {
                    Input::Button(_b) => {}
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
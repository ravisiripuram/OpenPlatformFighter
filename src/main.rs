#[macro_use]
extern crate enum_display_derive;

extern crate piston_window;
use piston_window::*;

#[macro_use]
mod common;
mod driver;
mod fighters;
use common::constants::*;
use driver::player::Player;
use driver::controls::*;
use fighters::test::*;
use common::stage::Stage;

fn main() {
    //init
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", WINDOW_SIZE)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    window = window.ups(FRAMES_PER_SECOND);
    // window.set_max_fps(FRAMES_PER_SECOND);
    let mut p1 = Player::new(test::new(), controls1());
    let stage = Stage::default();
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
                            clear([0.0, 0.0, 0.0, 1.0], g);
                            stage.draw(c.transform, g);
                            p1.draw(c.transform, g);
                        });
                    },
                    Loop::Update(u) => {
                        p1.update(u.dt);
                    },
                    _ => {}
                }

            }
            _ => {}
        }
    }
}
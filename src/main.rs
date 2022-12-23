extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::piston::PressEvent;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::Button;
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

mod constants;
mod game;
mod snake;
use constants::constants::*;
use game::game::Game;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", [WINDOW_SIZE, WINDOW_SIZE])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.handle_input(key);
        }

        if let Some(args) = event.render_args() {
            game.render(&args);
        }

        if let Some(args) = event.update_args() {
            game.update(&args);
        }
    }
}

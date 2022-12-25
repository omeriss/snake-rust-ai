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

mod ai;
mod constants;
mod game;
mod snake;
use ai::ai_game::ai_game::AiGame;
use ai::train_netwrok::train_network::*;
use constants::constants::*;
use game::game::Game;

fn play() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("snake", [WINDOW_SIZE, WINDOW_SIZE])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(Some(GlGraphics::new(opengl)));

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

fn load_trained() {
    println!("enter network path:");
    let mut path = String::new();
    std::io::stdin().read_line(&mut path);
    path = path.trim().to_string();

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("ai snake", [WINDOW_SIZE, WINDOW_SIZE])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    use crate::ai::neural_network::neural_network::NeuralNetwork;
    let network = NeuralNetwork::new_from_file(vec![24, 40, 40, 4], &path);
    let mut ai_game = AiGame::new_from(Some(GlGraphics::new(opengl)), &network);

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            ai_game.render(&args);
        }

        if let Some(args) = event.update_args() {
            ai_game.update(&args);
        }
    }
}

fn main() {
    let mut option = String::new();
    println!("1: play\n2: load trained\n3: train");
    std::io::stdin().read_line(&mut option);
    option = option.trim().to_string();

    if option.eq("1") {
        play();
    } else if option.eq("2") {
        load_trained();
    } else if option.eq("3") {
        println!("enter save folder path:");
        let mut save_path = String::new();
        std::io::stdin().read_line(&mut save_path);
        save_path = save_path.trim().to_string();

        println!("enter load file path:");
        let mut load_path = String::new();
        std::io::stdin().read_line(&mut load_path);
        load_path = load_path.trim().to_string();

        println!("staring training...\npress ctrl+c at any time to stop the training\nthe best network will be saved in the save folder with the name \"best.bin\"");
        train_network(&save_path, &load_path);
    } else {
        println!("Invalid option");
    }
}

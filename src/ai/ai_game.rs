pub mod ai_game {
    use crate::game::game::*;
    use crate::{
        ai::neural_network::neural_network::NeuralNetwork, constants::constants::GRID_SIZE,
    };
    use ndarray::Array1;
    use opengl_graphics::GlGraphics;
    use piston::{keyboard, RenderArgs, UpdateArgs};

    fn find_in_direction(
        game_map: &Vec<Vec<MapItem>>,
        head: (i32, i32),
        direction: (i32, i32),
    ) -> (u32, u32, u32) {
        let mut snake_distance = 0;
        let mut food_distance = 0;
        let mut wall_distance = 0;

        let mut current = head;

        loop {
            current = (current.0 + direction.0, current.1 + direction.1);
            wall_distance += 1;

            if current.0 < 0
                || current.0 >= GRID_SIZE as i32
                || current.1 < 0
                || current.1 >= GRID_SIZE as i32
            {
                break;
            }

            match game_map[current.1 as usize][current.0 as usize] {
                MapItem::Apple => {
                    food_distance = wall_distance;
                }
                MapItem::Snake => {
                    if snake_distance == 0 {
                        snake_distance = wall_distance;
                    }
                }
                _ => {}
            }
        }

        (snake_distance, food_distance, wall_distance)
    }

    fn get_input(game_map: Vec<Vec<MapItem>>, head: (i32, i32)) -> Array1<f64> {
        const DIRECTIONS: [(i32, i32); 8] = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];

        let mut input = Array1::zeros(24);

        for i in 0..DIRECTIONS.len() {
            let (snake_distance, food_distance, wall_distance) =
                find_in_direction(&game_map, head, DIRECTIONS[i]);

            input[0 + i * 3] = if snake_distance == 0 {
                0.0
            } else {
                //1.0 / snake_distance as f64
                1.0
            };

            input[1 + i * 3] = if food_distance == 0 {
                0.0
            } else {
                //1.0 / food_distance as f64
                1.0
            };

            input[2 + i * 3] = 1.0 / wall_distance as f64;
        }

        input
    }

    pub struct AiGame {
        game: Game,
        neural_network: NeuralNetwork,
    }

    impl AiGame {
        pub fn new(gl: Option<GlGraphics>) -> AiGame {
            AiGame {
                game: Game::new(gl),
                neural_network: NeuralNetwork::new(vec![24, 40, 40, 4]),
            }
        }

        pub fn new_from(gl: Option<GlGraphics>, neural_network: &NeuralNetwork) -> AiGame {
            AiGame {
                game: Game::new(gl),
                neural_network: neural_network.clone(),
            }
        }

        pub fn mutate(&mut self, mutation_percent: f64) {
            self.neural_network.mutate(mutation_percent);
        }

        pub fn render(&mut self, args: &RenderArgs) {
            self.game.render(args);
        }

        pub fn update(&mut self, args: &UpdateArgs) {
            if self.game.is_next_update_move(args.dt) {
                let input = get_input(self.game.get_map(), self.game.get_snake_head());

                // for i in 0..GRID_SIZE {
                //     for j in 0..GRID_SIZE {
                //         match self.game.get_map()[i as usize][j as usize] {
                //             MapItem::Empty => print!("."),
                //             MapItem::Snake => print!("S"),
                //             MapItem::Apple => print!("A"),
                //         }
                //     }
                //     println!();
                // }

                // for i in 0..input.len() {
                //     println!("{}: {} ", i, input[i]);
                // }

                let output = self.neural_network.feed_forward(&input);

                let mut max_index = 0;

                for i in 1..output.len() {
                    if output[i] > output[max_index] {
                        max_index = i;
                    }
                }

                //println!("{} {} {} {}", output[0], output[1], output[2], output[3]);

                match max_index {
                    0 => self.game.handle_input(keyboard::Key::Up),
                    1 => self.game.handle_input(keyboard::Key::Right),
                    2 => self.game.handle_input(keyboard::Key::Down),
                    3 => self.game.handle_input(keyboard::Key::Left),
                    _ => {}
                }
            }

            self.game.update(args);
        }

        pub fn get_game(&self) -> &Game {
            &self.game
        }

        pub fn calc_fitness(&self) -> f64 {
            if self.game.get_score() == 0 || self.game.get_turns() == 22 {
                return 0.0;
            }

            if self.game.get_score() <= 10 {
                return (1 << (self.game.get_score() * 2)) as f64 * (self.game.get_turns() as f64)
                    / 100.0;
            }

            (1 << 20) as f64 * (self.game.get_score() - 9) as f64 * (self.game.get_turns() as f64)
                / 100.0
        }

        pub fn get_neural_network(&self) -> &NeuralNetwork {
            &self.neural_network
        }
    }

    impl Clone for AiGame {
        fn clone(&self) -> Self {
            AiGame {
                game: Game::new(None),
                neural_network: self.neural_network.clone(),
            }
        }
    }
}

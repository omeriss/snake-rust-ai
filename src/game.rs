pub mod game {
    use crate::{
        constants::constants::{GRID_SIZE, SNAKE_SIZE, TIME_BETWEEN_MOVES},
        snake::snake::Snake,
    };
    use opengl_graphics::GlGraphics;
    use piston::{Key, RenderArgs, UpdateArgs};

    pub enum MapItem {
        Empty,
        Snake,
        Apple,
    }

    impl Clone for MapItem {
        fn clone(&self) -> MapItem {
            return match self {
                MapItem::Empty => MapItem::Empty,
                MapItem::Snake => MapItem::Snake,
                MapItem::Apple => MapItem::Apple,
            };
        }
    }

    pub struct Game {
        pub gl: Option<GlGraphics>,
        snake: Snake,
        apple: (i32, i32),
        time_to_move: f64,
        new_direction: (i32, i32),
        turns: u32,
    }

    impl Game {
        pub fn new(gl: Option<GlGraphics>) -> Game {
            Game {
                gl,
                snake: Snake::new(),
                apple: (8, (GRID_SIZE / 2) as i32),
                time_to_move: TIME_BETWEEN_MOVES,
                new_direction: (1, 0),
                turns: 0,
            }
        }

        pub fn render(&mut self, args: &RenderArgs) {
            use graphics::*;

            const BACKROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

            let square = rectangle::square(0.0, 0.0, SNAKE_SIZE);

            self.gl
                .as_mut()
                .unwrap()
                .draw(args.viewport(), |context, gl| {
                    clear(BACKROUND, gl);

                    let transform = context.transform.trans(
                        self.apple.0 as f64 * SNAKE_SIZE,
                        self.apple.1 as f64 * SNAKE_SIZE,
                    );

                    rectangle(RED, square, transform, gl);
                });

            self.snake.render(args, &mut self.gl.as_mut().unwrap())
        }

        pub fn handle_input(&mut self, key: Key) {
            self.new_direction = match key {
                Key::Up => (0, -1),
                Key::Down => (0, 1),
                Key::Left => (-1, 0),
                Key::Right => (1, 0),
                _ => return,
            };
        }

        fn chnage_apple_position(&mut self) {
            let snake = self.snake.get_snake();

            self.apple = (
                rand::random::<i32>().abs() % GRID_SIZE as i32,
                rand::random::<i32>().abs() % GRID_SIZE as i32,
            );

            while snake.contains(&self.apple) {
                self.apple = (
                    rand::random::<i32>().abs() % GRID_SIZE as i32,
                    rand::random::<i32>().abs() % GRID_SIZE as i32,
                );
            }
        }

        pub fn update(&mut self, args: &UpdateArgs) {
            if self.snake.is_dead() {
                return;
            }

            self.time_to_move -= args.dt;

            if self.time_to_move <= 0.0 {
                self.turns += 1;

                if self.snake.get_direction() != (-self.new_direction.0, -self.new_direction.1) {
                    self.snake.change_direction(self.new_direction);
                }

                self.time_to_move = TIME_BETWEEN_MOVES;

                if self.snake.get_next_head() == self.apple {
                    self.snake.update(true);
                    self.chnage_apple_position();
                } else {
                    self.snake.update(false);
                }
            }
        }

        pub fn is_next_update_move(&self, dt: f64) -> bool {
            self.time_to_move - dt <= 0.0
        }

        pub fn get_map(&self) -> Vec<Vec<MapItem>> {
            let mut map = vec![vec![MapItem::Empty; GRID_SIZE as usize]; GRID_SIZE as usize];

            for (x, y) in self.snake.get_snake() {
                map[*y as usize][*x as usize] = MapItem::Snake;
            }

            map[self.apple.1 as usize][self.apple.0 as usize] = MapItem::Apple;

            map
        }

        pub fn get_snake_head(&self) -> (i32, i32) {
            self.snake.get_head()
        }

        pub fn is_alive(&self) -> bool {
            !self.snake.is_dead()
        }

        pub fn get_score(&self) -> u32 {
            self.snake.get_size() as u32 - 2
        }

        pub fn get_turns(&self) -> u32 {
            self.turns
        }
    }
}

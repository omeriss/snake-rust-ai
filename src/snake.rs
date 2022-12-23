pub mod snake {
    use crate::constants::constants::*;
    use opengl_graphics::GlGraphics;
    use piston::{RenderArgs, UpdateArgs};

    pub struct Snake {
        snake: Vec<(i32, i32)>,
        direction: (i32, i32),
    }

    impl Snake {
        pub fn new() -> Snake {
            Snake {
                snake: vec![(3, (GRID_SIZE / 2) as i32), (2, (GRID_SIZE / 2) as i32)],
                direction: (1, 0),
            }
        }

        pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
            use graphics::*;

            const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

            let square = rectangle::square(0.0, 0.0, SNAKE_SIZE);

            gl.draw(args.viewport(), |context, gl| {
                for rec in self.snake.iter() {
                    let transform = context
                        .transform
                        .trans(rec.0 as f64 * SNAKE_SIZE, rec.1 as f64 * SNAKE_SIZE);

                    rectangle(GREEN, square, transform, gl);
                }
            });
        }

        pub fn get_next_head(&self) -> (i32, i32) {
            (
                self.snake[0].0 + self.direction.0,
                self.snake[0].1 + self.direction.1,
            )
        }

        pub fn update(&mut self, extend: bool) {
            if extend {
                self.snake.push(self.snake[self.snake.len() - 1]);
            }

            for i in (1..self.snake.len()).rev() {
                self.snake[i] = self.snake[i - 1];
            }

            self.snake[0] = self.get_next_head();
        }

        pub fn change_direction(&mut self, direction: (i32, i32)) {
            self.direction = direction;
        }

        pub fn get_snake(&self) -> &Vec<(i32, i32)> {
            &self.snake
        }

        pub fn get_direction(&self) -> (i32, i32) {
            self.direction
        }

        pub fn is_dead(&self) -> bool {
            let head = self.snake[0];

            if head.0 < 0 || head.0 >= GRID_SIZE as i32 || head.1 < 0 || head.1 >= GRID_SIZE as i32
            {
                return true;
            }

            for rec in self.snake.iter().skip(1) {
                if rec.0 == head.0 && rec.1 == head.1 {
                    return true;
                }
            }

            false
        }
    }
}

pub mod layer {
    use std::io::{Read, Write};

    use ndarray::{Array, Array1, Array2};
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;
    use rand::Rng;

    fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    #[derive(Debug)]
    pub struct Layer {
        weights: Array2<f64>,
        biases: Array1<f64>,
    }

    impl Layer {
        pub fn mutate(&mut self, mutation_percent: f64) {
            let mut rnd = rand::thread_rng();

            self.weights.mapv_inplace(|x| {
                x * rnd
                    .gen_range((1.0 - mutation_percent / 100.0)..=(1.0 + mutation_percent / 100.0))
            });

            self.biases.mapv_inplace(|x| {
                x * rnd
                    .gen_range((1.0 - mutation_percent / 100.0)..=(1.0 + mutation_percent / 100.0))
            });
        }

        pub fn new(input_size: u32, output_size: u32) -> Layer {
            let uniform = Uniform::new(-1.0, 1.0);

            Layer {
                weights: Array::random((output_size as usize, input_size as usize), uniform),
                biases: Array::random(output_size as usize, uniform),
            }
        }

        pub fn feed_forward(&self, input: &Array1<f64>) -> Array1<f64> {
            let mut output = self.weights.dot(input) + &self.biases;
            output.mapv_inplace(|x| sigmoid(x));
            output
        }

        pub fn write_to_file(&self, file: &mut std::fs::File) {
            for row in self.weights.rows() {
                for elem in row.iter() {
                    file.write(&elem.to_le_bytes());
                }
            }

            for elem in self.biases.iter() {
                file.write(&elem.to_le_bytes());
            }
        }

        pub fn new_from_file(input_size: u32, output_size: u32, file: &mut std::fs::File) -> Layer {
            let mut weights = Array2::<f64>::zeros((output_size as usize, input_size as usize));
            let mut biases = Array1::<f64>::zeros(output_size as usize);

            for mut row in weights.rows_mut() {
                for elem in row.iter_mut() {
                    let mut buf = [0; 8];
                    file.read_exact(&mut buf);
                    *elem = f64::from_le_bytes(buf);
                }
            }

            for elem in biases.iter_mut() {
                let mut buf = [0; 8];
                file.read(&mut buf);
                *elem = f64::from_le_bytes(buf);
            }

            Layer { weights, biases }
        }
    }

    impl Clone for Layer {
        fn clone(&self) -> Layer {
            Layer {
                weights: self.weights.clone(),
                biases: self.biases.clone(),
            }
        }
    }
}

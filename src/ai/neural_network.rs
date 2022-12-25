pub mod neural_network {
    use crate::ai::layer::layer::Layer;

    #[derive(Debug)]

    pub struct NeuralNetwork {
        layers: Vec<Layer>,
    }

    impl NeuralNetwork {
        pub fn mutate(&mut self, mutation_percent: f64) {
            for layer in &mut self.layers {
                layer.mutate(mutation_percent);
            }
        }

        pub fn new(shape: Vec<u32>) -> NeuralNetwork {
            let mut layers: Vec<Layer> = Vec::new();

            for i in 0..shape.len() - 1 {
                layers.push(Layer::new(shape[i], shape[i + 1]));
            }

            NeuralNetwork { layers }
        }

        pub fn feed_forward(&self, input: &ndarray::Array1<f64>) -> ndarray::Array1<f64> {
            let mut output = input.clone();

            for layer in &self.layers {
                output = layer.feed_forward(&output);
            }

            output
        }

        pub fn write_to_file(&self, path: &str) {
            let mut file = std::fs::File::create(path).unwrap();

            for layer in &self.layers {
                layer.write_to_file(&mut file);
            }
        }

        pub fn new_from_file(shape: Vec<u32>, path: &str) -> NeuralNetwork {
            let mut file = std::fs::File::open(path).unwrap();

            let mut layers: Vec<Layer> = Vec::new();

            for i in 0..shape.len() - 1 {
                layers.push(Layer::new_from_file(shape[i], shape[i + 1], &mut file));
            }

            NeuralNetwork { layers }
        }
    }

    impl Clone for NeuralNetwork {
        fn clone(&self) -> NeuralNetwork {
            let mut layers: Vec<Layer> = Vec::new();

            for layer in &self.layers {
                layers.push(layer.clone());
            }

            NeuralNetwork { layers }
        }
    }
}

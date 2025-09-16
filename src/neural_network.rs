use crate::config::IMAGE_SIZE;
use rand::Rng;

#[derive(Debug)]
pub struct NeuralNetwork {
    bias: Vec<f64>, // 10 biases (one per output)
    learning_rate: f64,
    weights: Vec<Vec<f64>>, // 10 sets of weights (784 weights per output)
}

impl NeuralNetwork {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let input_size = IMAGE_SIZE * IMAGE_SIZE; // 784
        let output_size = 10; // 10 digits

        // Initialize weights: 10 sets, each with 784 values
        let mut weights = Vec::new();
        for _ in 0..output_size {
            let mut weight_set = Vec::new();
            for _ in 0..input_size {
                weight_set.push(rng.random_range(-0.5..0.5));
            }
            weights.push(weight_set);
        }

        // Initialize 10 biases
        let mut bias = Vec::new();
        for _ in 0..output_size {
            bias.push(rng.random_range(-0.1..0.1));
        }

        return Self {
            bias,
            learning_rate: 0.1,
            weights,
        };
    }

    pub fn predict(&self, input: &[u8]) -> Vec<f64> {
        // Normalize input pixels
        let mut input_normalized = Vec::new();
        for &pixel in input {
            input_normalized.push(pixel as f64 / 255.0);
        }

        let mut outputs = Vec::new();

        // Calculate output for each of the 10 neurons
        for neuron_idx in 0..10 {
            let mut weighted_sum = self.bias[neuron_idx];

            for i in 0..input_normalized.len() {
                weighted_sum += self.weights[neuron_idx][i] * input_normalized[i];
            }

            outputs.push(self.sigmoid(weighted_sum));
        }

        // Apply softmax to convert to probabilities
        return self.softmax(&outputs);
    }

    pub fn predict_digit(&self, input: &[u8]) -> usize {
        let outputs = self.predict(input);

        let mut max_idx = 0;
        let mut max_val = outputs[0];

        for i in 1..outputs.len() {
            if outputs[i] > max_val {
                max_val = outputs[i];
                max_idx = i;
            }
        }

        return max_idx;
    }

    pub fn train(&mut self, inputs: Vec<u8>, outputs: Vec<u8>, epochs: usize) {
        let image_size = IMAGE_SIZE * IMAGE_SIZE;
        let num_images = inputs.len() / image_size;

        for epoch in 0..epochs {
            let mut total_loss = 0.0;

            for i in 0..num_images {
                // Extract one image from the dataset
                let start_idx = i * image_size;
                let end_idx = start_idx + image_size;
                let image_slice = &inputs[start_idx..end_idx];

                // Normalize pixels
                let mut input_normalized = Vec::new();
                for &pixel in image_slice {
                    input_normalized.push(pixel as f64 / 255.0);
                }

                // Predict probabilities
                let predictions = self.predict_normalized(&input_normalized);

                // One-hot target
                let target_digit = outputs[i] as usize;
                let mut target = vec![0.0; 10];
                target[target_digit] = 1.0;

                // Calculate loss (cross-entropy)
                for j in 0..10 {
                    if predictions[j] > 0.0 {
                        total_loss -= target[j] * predictions[j].ln();
                    }
                }

                // Update weights and biases
                for neuron_idx in 0..10 {
                    let error = target[neuron_idx] - predictions[neuron_idx];
                    let delta = error; // for softmax + cross-entropy

                    for j in 0..input_normalized.len() {
                        self.weights[neuron_idx][j] +=
                            self.learning_rate * delta * input_normalized[j];
                    }

                    self.bias[neuron_idx] += self.learning_rate * delta;
                }
            }

            if epoch % 10 == 0 {
                let avg_loss = total_loss / num_images as f64;
                println!("Epoch {}: Average Loss = {:.4}", epoch, avg_loss);
            }
        }
    }

    fn predict_normalized(&self, input: &[f64]) -> Vec<f64> {
        let mut outputs = Vec::new();

        // Calculate raw output for each neuron
        for neuron_idx in 0..10 {
            let mut weighted_sum = self.bias[neuron_idx];
            for i in 0..input.len() {
                weighted_sum += self.weights[neuron_idx][i] * input[i];
            }
            outputs.push(weighted_sum);
        }

        // Softmax probabilities
        return self.softmax(&outputs);
    }

    fn sigmoid(&self, x: f64) -> f64 {
        return 1.0 / (1.0 + (-x).exp());
    }

    fn softmax(&self, inputs: &[f64]) -> Vec<f64> {
        let mut max_val = f64::NEG_INFINITY;
        for &x in inputs {
            if x > max_val {
                max_val = x;
            }
        }

        let mut exp_values = Vec::new();
        for &x in inputs {
            exp_values.push((x - max_val).exp());
        }

        let sum: f64 = exp_values.iter().sum();

        return exp_values.into_iter().map(|x| x / sum).collect();
    }

    pub fn test_accuracy(&self, test_images: &[u8], test_labels: &[u8]) -> f64 {
        let image_size = IMAGE_SIZE * IMAGE_SIZE;
        let num_test_images = test_images.len() / image_size;
        let mut correct_predictions = 0;

        for i in 0..num_test_images {
            let start_idx = i * image_size;
            let end_idx = start_idx + image_size;
            let image_slice = &test_images[start_idx..end_idx];

            let prediction = self.predict_digit(image_slice);
            let actual = test_labels[i] as usize;

            if prediction == actual {
                correct_predictions += 1;
            }
        }

        return (correct_predictions as f64 / num_test_images as f64) * 100.0;
    }
}

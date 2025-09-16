mod config;
mod neural_network;

use crate::neural_network::NeuralNetwork;
use mnist::MnistBuilder;
use std::env::current_dir;

fn main() {
    let mnist_data_path = format!("{}/data/mnist", current_dir().unwrap().display());
    let mnist = MnistBuilder::new()
        .base_path(&mnist_data_path)
        .label_format_digit()
        .finalize();

    let mut nn = NeuralNetwork::new();

    // Train the neural network with the mnist dataset
    nn.train(mnist.trn_img, mnist.trn_lbl, 100);

    // Test the neural network accuracy with the test dataset
    let accuracy = nn.test_accuracy(&mnist.tst_img, &mnist.tst_lbl);
    println!("Neural Network accuracy on test set: {:.2}%", accuracy);
}

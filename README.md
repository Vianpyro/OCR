# OCR - Optical Character Recognition in Rust

A learning-focused project implementing Optical Character Recognition (OCR) in Rust, with an initial focus on number recognition. This project serves as a hands-on exploration of both the Rust programming language and artificial intelligence concepts.

## 🎯 Project Goals

This project represents my first significant undertaking that combines two areas I'm passionate about learning:
- **Rust Programming**: Exploring systems programming, memory safety, and performance
- **Artificial Intelligence**: Understanding machine learning concepts and computer vision

The primary objective is to build an OCR system capable of recognizing handwritten and printed numbers (0-9), serving as a foundation for more complex character recognition in the future.

## 🧠 How It Works (Current Implementation)

Right now, the project implements a **simple neural network without hidden layers** — essentially a **logistic regression classifier with softmax**.

### Network Architecture
- **Input layer**:
  - Each image is `28x28` pixels (`784` values in total).
  - Pixel values are normalized to the range `0.0–1.0`.

- **Output layer**:
  - 10 neurons (one for each digit `0–9`).
  - Each neuron has its own set of `784` weights + a bias.
  - Outputs are passed through **softmax** to get probabilities.

### Training
- Uses the **MNIST dataset** (handwritten digits).
- For each training image:
  1. Normalize pixel values.
  2. Perform forward pass to compute predictions.
  3. Compare predictions to the true label (one-hot encoded).
  4. Compute **cross-entropy loss**.
  5. Update weights and biases using **gradient descent**.

- The gradient simplifies nicely for softmax + cross-entropy, so the weight updates are straightforward:
```

weight += learning\_rate \* (target - prediction) \* input
bias   += learning\_rate \* (target - prediction)

```

### Prediction
- Given an image, the network computes probabilities for digits `0–9`.
- The digit with the highest probability is returned as the prediction.

### Evaluation
- A `test_accuracy` method compares predictions against labels from the test dataset and returns the overall accuracy as a percentage.

---

## 🚀 Features (Planned)

- [x] **Basic Neural Network** (single-layer, no hidden layers)
- [ ] **Number Recognition**: Detect and classify digits 0-9 from images
- [ ] **Image Preprocessing**: Basic image enhancement and noise reduction
- [ ] **Machine Learning Integration**: Implement or integrate ML models for character recognition
- [ ] **Command Line Interface**: Easy-to-use CLI for processing images
- [ ] **Multiple Input Formats**: Support for common image formats (PNG, JPEG, etc.)
- [ ] **Confidence Scoring**: Provide confidence levels for recognition results

## 🛠️ Technology Stack

- **Language**: Rust
- **Image Processing**: (To be determined - considering `image` crate)
- **Machine Learning (custom)**: Single-layer neural network implemented from scratch
- **CLI Framework**: (Considering `clap` for command-line interface)

## 📋 Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## 🔧 Installation

```bash
# Clone the repository
git clone https://github.com/Vianpyro/OCR.git
cd OCR

# Build the project (when code is available)
cargo build

# Run tests (when implemented)
cargo test
```

## 💻 Usage

*Note: This section will be updated as the project develops.*

```bash
# Example usage (planned)
cargo run -- recognize path/to/image.png
```

## 📂 Dataset

This project uses the **MNIST handwritten digit dataset** as training and testing data.
The dataset files are downloaded from a [GitHub mirror](https://github.com/fgnt/mnist):

* [train-images-idx3-ubyte.gz](https://raw.githubusercontent.com/fgnt/mnist/master/train-images-idx3-ubyte.gz) – Training images (60,000 examples)
* [train-labels-idx1-ubyte.gz](https://raw.githubusercontent.com/fgnt/mnist/master/train-labels-idx1-ubyte.gz) – Labels for the training images
* [t10k-images-idx3-ubyte.gz](https://raw.githubusercontent.com/fgnt/mnist/master/t10k-images-idx3-ubyte.gz) – Test images (10,000 examples)
* [t10k-labels-idx1-ubyte.gz](https://raw.githubusercontent.com/fgnt/mnist/master/t10k-labels-idx1-ubyte.gz) – Labels for the test images

### Purpose of each file

* **Training set (`train-*`)**: Used to train the machine learning model.
* **Test set (`t10k-*`)**: Used only after training to evaluate model accuracy and generalization.

## 🗺️ Roadmap

### Phase 1: Foundation (Current)
- [x] Project setup and documentation
- [x] Implement basic single-layer neural network (input → output, no hidden layers)
- [ ] Choose and integrate image processing libraries
- [ ] Implement basic image loading and preprocessing

### Phase 2: Basic Recognition
- [x] Train the network with MNIST
- [x] Evaluate accuracy on test dataset
- [ ] Expose digit recognition through CLI

### Phase 3: Machine Learning
- [ ] Add **hidden layers** (multilayer perceptron)
- [ ] Experiment with different activation functions (ReLU, tanh)
- [ ] Improve accuracy through feature engineering

### Phase 4: Enhancement
- [ ] Support for multiple fonts and handwriting styles
- [ ] Real-time processing capabilities
- [ ] Web interface or GUI

## 🎓 Learning Objectives

This project is designed as a learning experience to:
- Master Rust ownership, borrowing, and memory management
- Understand the math behind neural networks (forward pass, softmax, cross-entropy, gradient descent)
- Explore machine learning model integration in Rust
- Practice software architecture and project organization
- Learn about performance optimization in systems programming

## 🤝 Contributing

As this is primarily a learning project, contributions and suggestions are welcome! Whether you're also learning Rust, have OCR expertise, or want to share resources, feel free to:

- Open issues for bugs or feature suggestions
- Submit pull requests for improvements
- Share learning resources or documentation improvements
- Provide feedback on code architecture and Rust best practices

## 📚 Resources and References

- [Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Computer Vision and OCR concepts](https://en.wikipedia.org/wiki/Optical_character_recognition)
- [Machine Learning in Rust ecosystem](https://www.arewelearningyet.com/)

## 📝 License

This project is open source and available under the [MIT License](LICENSE).

## 🔍 Current Status

**Project Status**: 🚧 Early Development

Currently, the project implements a **working single-layer neural network** for digit classification, with training and testing loops. Next steps involve adding hidden layers, preprocessing, and a CLI interface.

---

*Built with ❤️ and lots of learning by [Vianpyro](https://github.com/Vianpyro)*

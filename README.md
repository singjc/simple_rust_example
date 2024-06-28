# Simple Rust Example

This repository contains a simple Rust project demonstrating basic project structure and functionality.

## Project Structure

```
simple_rust_example/
├── Cargo.toml
├── src/
│ ├── main.rs
│ └── lib.rs
├── tests/
│ └── integration_test.rs
├── examples/
│ └── example1.rs
├── benches/
│ └── benchmark1.rs
└── .gitignore  
```

## Getting Started

### Prerequisites

- Rust programming language - [Install Rust](https://www.rust-lang.org/tools/install)

### Building the Project

1. Clone the repository:

```
git clone https://github.com/yourusername/simple_rust_example.git
cd simple_rust_example
```

2. Build the project:

```
cargo build
```

### Running the Project

To run the main binary:

```
cargo run
```

### Running Tests

To run all tests:

```
cargo test
```


### Running Examples

To run the example:

```
cargo run --example example1
```


### Running Benchmarks

Benchmarks use the unstable `test` feature. To run them:

1. Switch to the nightly toolchain:
    
    ```
    rustup default nightly
    ```

2. Run the benchmarks:

    ```
    cargo bench
    ```


## Project Overview

This project demonstrates:

- Basic Rust project structure
- A simple function (`add`) in the library
- Unit tests in the library file
- An integration test
- An example of using the library function
- A simple benchmark


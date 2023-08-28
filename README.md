# min-tun

## Description

`min-tun` is a Rust library for finding the minimum available TUN device on Linux systems. This crate provides a simple and reliable way to determine which TUN device names are available for use.

## Prerequisites

- Rust programming language
- Cargo package manager

## Building the Crate

To build the crate, follow these steps:

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/min-tun.git
    ```

    Replace `yourusername` with your actual GitHub username and `min-tun` with your repository name if different.

2. Navigate into the project directory:

    ```bash
    cd min-tun
    ```

3. Build the crate:

    ```bash
    cargo build
    ```

## Using the Crate in Your Rust Project

To use `min-tun` in your Rust project, you need to include it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
min-tun = "0.1.0"  # Replace with the actual version number if different
```

After adding the dependency, you can use the crate in your code. Here's a simple example:

```rust
// main.rs

fn main() {
    match min_tun::find_min_available_tun() {
        Some(tun_name) => println!("Minimum available tun device: {}", tun_name),
        None => println!("Could not find an available tun device"),
    }
}
```

To build and run your project, execute:

```bash
cargo build
cargo run
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


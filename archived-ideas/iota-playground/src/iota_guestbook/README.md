# IOTA Guestbook Application

This is a simple guestbook application written in Rust that interacts with the 
IOTA Layer 1 network.


## Prerequisites

- Rust installed on your system. If you haven't installed Rust yet, you can do 
  so by following the instructions on the official Rust website.
- Basic understanding of Rust programming.
- An IOTA wallet with some IOTA tokens for testing purposes. You can obtain 
  these from the IOTA faucet.


## Building the Application

1. Install Rust from https://www.rust-lang.org/tools/install.
2. Clone this repository and navigate into it.
3. Run `cargo build --release` to build the application.

## Running the Application

1. Set the `IOTA_SEED` environment variable to your IOTA seed.
2. Run `cargo run --release` to execute the application.

## Deploying to the IOTA Mainnet

1. Replace the node URL and the output address in `src/main.rs` with the mainnet values.
2. Follow the building and running instructions above.

## Note

This application is for educational purposes only. Always be cautious when handling real IOTA tokens.

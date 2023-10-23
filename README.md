# Rust-Rocket-Robot
This uses the Rust Language and the Rocket web framework to control a robot

## Getting Started

First, make sure to have Rust installed in your system following [these](https://doc.rust-lang.org/book/ch01-01-installation.html) instructions. Optionally you can build the application on the Pi with `cargo build` but that will take time so it's best to use `cross`. I have provided a `playbooks` directory to allow you to cross compile and then copy the executable into the Pi. More information found [here](https://github.com/cross-rs/cross). Once the application is built you will get warnings but they can be ignored. Run the application with `cargo run` and then go to <ip-address-of-pi>:8000 and it should work. Press the buttons to move the robot either with a mouse, phone or tablet. 


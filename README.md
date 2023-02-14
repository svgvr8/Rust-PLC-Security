# Rust Access Control for PLC
This project is a simple access control program for a PLC device, written in Rust. The program prompts the user to enter a 4-digit password, and if the correct password is entered, the program unlocks the door or device. If an incorrect password is entered, the program triggers an alarm or other security measure.
## How to build
- `apt install gcc-arm-linux-gnueabihf`
- `rustup target add armv7-unknown-linux-gnueabihf`
- `export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=/usr/bin/arm-linux-gnueabihf-gcc`
- `cargo build --target=armv7-unknown-linux-gnueabihf`

## How to run
- `cargo run my_binary`

## How to Use
- Connect the PLC device to your computer via USB or other connection.
- Compile the Rust code using a cross-compiler for the PLC device, such as armv7-unknown-linux-gnueabihf.
- Upload the compiled binary to the PLC device.
- Run the program on the PLC device.
- Follow the on-screen instructions to enter the password and unlock the door or device.
## Why Rust for PLC?
There are several benefits to using Rust for programming PLC devices:

- Rust is a systems programming language, which means it can be used for low-level programming tasks such as device drivers and hardware interfacing.
- Rust is a safe and secure language, with features such as memory safety and thread safety, making it ideal for programming safety-critical systems like those found in PLC devices.
- Rust has a small runtime and a low memory footprint, making it well-suited for use on resource-constrained devices like PLCs.
## Why This Program Uses Rust
This program was written in Rust because it is a safe, secure, and efficient language that is well-suited for programming PLC devices. The program uses Rust's strong type system and memory safety features to prevent common programming errors that can lead to security vulnerabilities or system crashes. Rust's efficient and low-level programming features also make it a good choice for programming devices with limited resources, such as PLCs. Finally, Rust's growing popularity and strong community support make it a good language to learn for anyone interested in PLC programming.

## Cross-Compilation
This program was cross-compiled using a cross-compiler for the PLC device. Cross-compilation is the process of compiling code on one platform (in this case, a computer) to run on another platform (in this case, a PLC device). Cross-compiling allows developers to write code on their development machine, and then compile it for the target platform without needing to install a full development environment on the target device.
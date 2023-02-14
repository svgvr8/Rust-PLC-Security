# Make sure GCC's linker for the target platform is installed on your
# system
apt install gcc-arm-linux-gnueabihf
# Install the standard library for the target platform
rustup target add armv7-unknown-linux-gnueabihf
# Tell cargo to use the linker you just installed rather than the default
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=/usr/bin/arm-linux-gnueabihf-gcc
# Build!
cargo build --target=armv7-unknown-linux-gnueabihf
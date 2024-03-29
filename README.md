![Build for target](https://github.com/aholzbaur/rust-stm32f4-disco-blinky/workflows/Build%20for%20target/badge.svg)

# Blinky example in Rust on the STM32F429-Discovery board

Small project example to be used as template for embedded software in Rust on the STM32F429-Discovery board.

## Hardware

Evaluation hardware STM32F429I-DISC1 (https://www.st.com/content/st_com/en/products/evaluation-tools/product-evaluation-tools/mcu-mpu-eval-tools/stm32-mcu-mpu-eval-tools/stm32-discovery-kits/32f429idiscovery.html#resource)
 - STM32F429ZIT6 with 2 MB flash, 256 KB RAM, 180 MHz
 - 2 user LEDs
 - 1 user button
 - MEMS motion sensor
 - LC display
 - 8 MB SDRAM
 - this hardware revision has onboard ST-LINK/V2-B

## Software
### Windows (as of 09/2020)
- Rust v1.46.0
  - cargo install cargo-binutils
  - cargo install cargo-generate
  - rustup component add llvm-tools-preview
  - rustup target add thumbv7m-none-eabi
- GNU Arm Embedded Toolchain 9-2020-q2
- OpenOCD (xPack release https://github.com/xpack-dev-tools/openocd-xpack/releases/)
- Visual Studio Code
  - Rust plugin (rust-lang.rust)
  - Cortex-Debug plugin (marus25.cortex-debug)
- Visual Studio 2019 Community Edition (necessary for some build tools)
- ST-LINK/V2-B USB driver

If everything is installed properly, the workspace can be opened in VS Code and a hit on F5 should build the project without errors or warnings and automatically start a debug session. The program should be halted at the breakpoint at main().

### Ubuntu 21 (as of 02/2022)
Not sure, if these are really necessary here:

`sudo apt-get install gcc make clang`

Required installation:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install cargo-binutils
```

Building cargo-generate requires (at least) libssl-dev to be installed:

```
sudo apt-get install libssl-dev
cargo install cargo-generate
rustup component add llvm-tools-preview
```
Verify installation for x86_64 according to https://docs.rust-embedded.org/discovery/f3discovery/03-setup/index.html:

```
cargo new first-test
cargo cd first-test
cargo run
cargo size -- --version
```

Then install the embedded toolchain: `rustup target add thumbv7em-none-eabihf`

This project should now be able to be built:
```
cd rust-stm32f4-disco-blinky
cargo build
```

To debug, download the latest ARM GNU Embedded Toolchain (in this case 10.3-2021.10) and run:
```
sudo tar -xvjf gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2 -C /usr/share/
sudo ln -s /usr/share/gcc-arm-none-eabi-10.3-2021.10/bin/* /usr/bin/
sudo apt-get install libncurses5
arm-none-eabi-gcc --version
```

Now follow https://docs.rust-embedded.org/discovery/f3discovery/03-setup/linux.html#udev-rules and https://docs.rust-embedded.org/discovery/f3discovery/03-setup/verify.html until OpenOCD can connect to ST-Link: `openocd -f interface/stlink.cfg -f target/stm32f4x.cfg` and keep it running in a separate terminal.

In another terminal, calling `arm-none-eabi-gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/rust-stm32f4-disco-blinky` should successfully flash the controller and halt it (i.e. at Reset()).

## Links

Project was created following the Rust Embedded Book (https://docs.rust-embedded.org/book/) and the Discovery book (https://rust-embedded.github.io/discovery/).

Based partly on the projects https://github.com/rust-embedded/cortex-m-quickstart, https://github.com/adamgreig/stm32f4-demo and https://github.com/rust-embedded/discovery.

Debugging via Visual Studio Code was enabled following the article on https://dev.to/rubberduck/debugging-rust-arm-cortexm-programs-with-visual-studio-code-336h.

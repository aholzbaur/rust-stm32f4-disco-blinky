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

## Links

Project was created following the Rust Embedded Book (https://docs.rust-embedded.org/book/) and the Discovery book (https://rust-embedded.github.io/discovery/).

Based partly on the projects https://github.com/rust-embedded/cortex-m-quickstart, https://github.com/adamgreig/stm32f4-demo and https://github.com/rust-embedded/discovery.

Debugging via Visual Studio Code was enabled following the article on https://dev.to/rubberduck/debugging-rust-arm-cortexm-programs-with-visual-studio-code-336h.

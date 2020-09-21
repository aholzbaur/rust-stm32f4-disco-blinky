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
- Visual Studio Code
- Visual Studio 2019 Community Edition (necessary for some build tools)
- ST-LINK/V2-B USB driver

## Links

Project was created following the Rust Embedded Book (https://docs.rust-embedded.org/book/) and the Discovery book (https://rust-embedded.github.io/discovery/).
Based partly on the projects https://github.com/adamgreig/stm32f4-demo and https://github.com/rust-embedded/discovery.
Debugging via Visual Studio Code was enabled following the article on https://dev.to/rubberduck/debugging-rust-arm-cortexm-programs-with-visual-studio-code-336h.

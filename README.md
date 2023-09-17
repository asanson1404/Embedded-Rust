# Practical classes to learn Rust

This repo contains two labs written by Samuel Tardieu to learn the Rust programming language :

## LAB1: Creation of a Virtual Machine in Rust

`tp-rust-VM/` is a project where we created a simple Virtual Machine in Rust. This VM has **16 virtual registers** and a **virtual memory of 4096 bytes**. It is able to perform some basics operations such as `move_if`, `store`, `load`, `loadimm` and `sub`. By combining these simple operations, it is possible to process more complexe instructions such as `push`, `pop`, `mult`, `fact` and others.

* The VM code is located in `src/machine.rs`
* Test files are located in `tests/`:<br> 
`tests/assignment.rs` is the easiest test file,<br> 
`tests/basic_operations.rs` is more challenging,<br> 
`tests/complex_execution.rs` is the most difficult test file.<br> 
To be 100% functional, the VM must pass all the tests.

Below are the commands to test the VM :
```shell
$ cargo build
$ cargo test --test assignment
$ cargo test --test basic_operations
$ cargo test --test complex_execution
```

## LAB2: Embedded Rust

The goal of the lab `tp-led-matrix/` is to program the **STM32L475 board** (Arm Cortex-M4) from scratch and display visual elements on a led matrix. You can find more info about this board in the [reference manuel](https://www.st.com/resource/en/reference_manual/rm0351-stm32l47xxx-stm32l48xxx-stm32l49xxx-and-stm32l4axxx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf).

Description of the Rust files:
* `src/` contains all the source of the led matrix and the peripherals configuration.
    * `image.rs` defines structures which represent an individual RBG pixel and the whole 8x8 image made of pixels.
    * `gamma.rs` provides the require [gamma correction](https://en.wikipedia.org/wiki/Gamma_correction) of the led matrix.
    * `matrix.rs` defines the necessary functions to handle correctly the led matrix and associates the GPIOs to the right led matrix's pins.
    * `main.rs` set up correctly the STM32L475 board. It uses the crate `stm32l4xx_hal` to set up the clocks, the usart, the gpios, etc. 

#### How to run this code on the STM32L475 board?

First, you need to download the corresponding target (microcontroller STM32L475VGT6 which contains a Cortex-M4F core) so that Rust can cross-compile for it.
```shell
$ rustup target add thumbv7em-none-eabihf
```

The project has been set to build the code automatically for this target (configured in `.cargo/config.toml`).
```shell
[build]
target = "thumbv7em-none-eabihf" # Cortex-M4F/M7F
```

It is now possible to build the code for the right target :
```shell
# Debug mode
$ cargo build
# release mode
$ cargo build --release
```
(For your information, we used the linker script provided by the `cortex-m-rt` crate and we defined the memory regions in `memory.x`).<br>

You then must ensure that you have either one of `arm-none-eabi-gdb` or `gdb-multiarch` installed on your system. If this is not the case, install it before proceeding.

Open a dedicated terminal and launch :
```shell
$ JLinkGDBServer -device STM32L475VG
```

Finally, when the board is connected, run the program:
```shell
$ cargo run
```

<br>To understand what really happens under the hood, you will see the following line in `.cargo/config.toml`:
```shell
runner = "arm-none-eabi-gdb -q -x jlink.gdb"
```
It executes the `jlink.gdb` script when you type `cargo run`:
```shell
target extended-remote :2331
load
mon reset
c
```





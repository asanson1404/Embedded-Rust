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

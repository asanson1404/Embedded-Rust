//! Module to configure and program our 
//! LED Matrix

use stm32l4xx_hal::{gpio::*, rcc::Clocks, prelude::_embedded_hal_blocking_delay_DelayMs};
use crate::{Color, Image};

pub struct Matrix {
    sb: PC5<Output<PushPull>>,
    lat: PC4<Output<PushPull>>,
    rst: PC3<Output<PushPull>>,
    sck: PB1<Output<PushPull>>,
    sda: PA4<Output<PushPull>>,
    c0: PB2<Output<PushPull>>,
    c1: PA15<Output<PushPull>>,
    c2: PA2<Output<PushPull>>,
    c3: PA7<Output<PushPull>>,
    c4: PA6<Output<PushPull>>,
    c5: PA5<Output<PushPull>>,
    c6: PB0<Output<PushPull>>,
    c7: PA3<Output<PushPull>>,
}

impl Matrix {
    /// Create a new matrix from the control registers and the individual
    /// unconfigured pins. SB and LAT will be set high by default, while
    /// other pins will be set low. After 100ms, RST will be set high, and
    /// the bank 0 will be initialized by calling `init_bank0()` on the
    /// newly constructed structure.
    /// The pins will be set to very high speed mode.
    #[allow(clippy::too_many_arguments)]   // Necessary to avoid a clippy warning
    pub fn new(
        pa2: PA2<Analog>,
        pa3: PA3<Analog>,
        pa4: PA4<Analog>,
        pa5: PA5<Analog>,
        pa6: PA6<Analog>,
        pa7: PA7<Analog>,
        pa15: PA15<Alternate<PushPull, 0>>,
        pb0: PB0<Analog>,
        pb1: PB1<Analog>,
        pb2: PB2<Analog>,
        pc3: PC3<Analog>,
        pc4: PC4<Analog>,
        pc5: PC5<Analog>,
        gpioa_moder: &mut MODER<'A'>,
        gpioa_otyper: &mut OTYPER<'A'>,
        gpiob_moder: &mut MODER<'B'>,
        gpiob_otyper: &mut OTYPER<'B'>,
        gpioc_moder: &mut MODER<'C'>,
        gpioc_otyper: &mut OTYPER<'C'>,
        clocks: Clocks,
    ) -> Self {

        // PA2 in output mode and very high speed mode
        let pa2 = pa2.into_push_pull_output_in_state(gpioa_moder, gpioa_otyper, PinState::Low).set_speed(Speed::VeryHigh);
        // PA3 in output mode and very high speed mode
        let pa3 = pa3.into_push_pull_output_in_state(gpioa_moder, gpioa_otyper, PinState::Low).set_speed(Speed::VeryHigh);
        // PA4 in output mode and very high speed mode
        let pa4 = pa4.into_push_pull_output_in_state(gpioa_moder, gpioa_otyper, PinState::Low).set_speed(Speed::VeryHigh);
        // PA5 in output mode and very high speed mode
        let pa5 = pa5.into_push_pull_output_in_state(gpioa_moder, gpioa_otyper, PinState::Low).set_speed(Speed::VeryHigh);
        // PA6 in output mode and very high speed mode
        let pa6 = pa6.into_push_pull_output_in_state(gpioa_moder, gpioa_otyper, PinState::Low).set_speed(Speed::VeryHigh);
        // PA7 in output mode and very high speed mode
        let pa7 = pa7.into_push_pull_output_in_state(gpioa_moder, gpioa_otyper, PinState::Low).set_speed(Speed::VeryHigh);
        // PA15 in output mode and very high speed mode
        let pa15 = pa15.into_push_pull_output_in_state(gpioa_moder, gpioa_otyper, PinState::Low).set_speed(Speed::VeryHigh);

        // PB0 in output mode and very high speed mode
        let pb0 = pb0.into_push_pull_output_in_state(gpiob_moder, gpiob_otyper, PinState::Low).set_speed(Speed::VeryHigh);
        // PB1 in output mode and very high speed mode
        let pb1 = pb1.into_push_pull_output_in_state(gpiob_moder, gpiob_otyper, PinState::Low).set_speed(Speed::VeryHigh);
        // PB2 in output mode and very high speed mode
        let pb2 = pb2.into_push_pull_output_in_state(gpiob_moder, gpiob_otyper, PinState::Low).set_speed(Speed::VeryHigh);

        // PC3 in output mode and very high speed mode
        let pc3 = pc3.into_push_pull_output_in_state(gpioc_moder, gpioc_otyper, PinState::Low).set_speed(Speed::VeryHigh);
        // PC4 in output mode and very high speed mode
        let pc4 = pc4.into_push_pull_output_in_state(gpioc_moder, gpioc_otyper, PinState::High).set_speed(Speed::VeryHigh);
        // PC5 in output mode and very high speed mode
        let pc5 = pc5.into_push_pull_output_in_state(gpioc_moder, gpioc_otyper, PinState::High).set_speed(Speed::VeryHigh);
        
        // Creating the new matrix from the control registers and the individual
        // unconfigured pins.
        let mut matrix = Matrix {
            sb:  pc5,
            lat: pc4,
            rst: pc3,
            sck: pb1,
            sda: pa4,
            c0:  pb2,
            c1:  pa15,
            c2:  pa2,
            c3:  pa7,
            c4:  pa6,
            c5:  pa5,
            c6:  pb0,
            c7:  pa3
        };

        // Delay of 100 ms
        stm32l4xx_hal::delay::DelayCM::new(clocks).delay_ms(100u8);

        // RST is set high
        matrix.rst.set_high();

        // Initialize the bank 0
        matrix.init_bank0();

        matrix
    }

    /// Make a brief high pulse of the SCK pin
    fn pulse_sck(&mut self) {
        // No need to wait between a high and a low
        // instruction. The processor is enough slow
        self.sck.set_low();
        self.sck.set_high();
        self.sck.set_low();
    }

    /// Make a brief low pulse of the LAT pin
    fn pulse_lat(&mut self) {
        // No need to wait between a high and a low
        // instruction. The processor is enough slow
        self.lat.set_high();
        self.lat.set_low();
        self.lat.set_high();

    }

    /// Set the given row output in the chosen state
    fn row(&mut self, row: usize, state: PinState) {
        match row {
            0 => self.c0.set_state(state),
            1 => self.c1.set_state(state),
            2 => self.c2.set_state(state),
            3 => self.c3.set_state(state),
            4 => self.c4.set_state(state),
            5 => self.c5.set_state(state),
            6 => self.c6.set_state(state),
            7 => self.c7.set_state(state),
            _ => panic!("Undefined row number !")
        }
    }

    /// Send a byte on SDA starting with the MSB and pulse SCK high after each bit
    fn send_byte(&mut self, pixel: u8) {
        for i in (0..8).rev() {
            self.sda.set_state(PinState::from((pixel & (1<<i)) != 0));
            self.pulse_sck();
        }
    }

    /// Activate a specified row
    fn activate_row (&mut self, row: usize) {
        match row {
            0 => self.row(0, PinState::High),
            1 => self.row(1, PinState::High),
            2 => self.row(2, PinState::High),
            3 => self.row(3, PinState::High),
            4 => self.row(4, PinState::High),
            5 => self.row(5, PinState::High),
            6 => self.row(6, PinState::High),
            7 => self.row(7, PinState::High),
            _ => panic!("Undefined row number !")
        }
    }

    /// Send a full row of bytes in BGR order and pulse LAT low. Gamma correction
    /// must be applied to every pixel before sending them. The previous row must
    /// be deactivated and the new one activated.
    pub fn send_row(&mut self, row: usize, pixels: &[Color]) {
        
        for i in (0..8).rev() {
            self.send_byte(pixels[i].gamma_correct().b);
            self.send_byte(pixels[i].gamma_correct().g);
            self.send_byte(pixels[i].gamma_correct().r);
            if i == 5 {
                if row > 0 {self.row(row-1, PinState::Low);}
                else       {self.row(7, PinState::Low);}
            }
        }
        self.pulse_lat();
        self.activate_row(row);

    }

    /// Initialize bank0 by temporarily setting SB to low and sending 144 one bits,
    /// pulsing SCK high after each bit and pulsing LAT low at the end. SB is then
    /// restored to high.
    fn init_bank0(&mut self) {
        self.sb.set_low();
        for _i in 0..18 {
            self.send_byte(255u8);
        }
        self.pulse_lat();
        self.sb.set_high();
    }

    /// Display a full image, row by row, as fast as possible.
    pub fn display_image(&mut self, image: &Image) {
        for i in 0..8 {
            let pixels = image.row(i);
            self.send_row(i, pixels);
        }
    }
}

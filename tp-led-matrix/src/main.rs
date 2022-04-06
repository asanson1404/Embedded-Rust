#![no_std]
#![no_main]

use cortex_m_rt::entry;
//use stm32l4 as _;   // Just to link it in the executable (it provides the vector table)
use stm32l4xx_hal::{pac, prelude::*};
//use panic_rtt_target as _;
use panic_probe as _;
//use rtt_target::{rtt_init_print, rprintln};
use defmt_rtt as _;
use tp_led_matrix::{Image, Color};
use tp_led_matrix::Matrix;

/*
#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
   loop {}
}
*/

#[entry]
fn main() -> ! {
    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    run(cp, dp)
}

fn run(_cp: pac::CorePeripherals, dp: pac::Peripherals) -> ! {
    // Get high-level representations of hardware modules
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    // Setup the clocks at 80MHz using HSI (by default since HSE/MSI are not configured).
    // The flash wait states will be configured accordingly.
    let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);

    //rtt_init_print!();
    //rprintln!("Hello, world!");
    defmt::info!("Hello, world!");
    //panic!("The program stopped");

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);
    
    let mut led_matrix = Matrix::new(
        gpioa.pa2,
        gpioa.pa3,
        gpioa.pa4,
        gpioa.pa5,
        gpioa.pa6,
        gpioa.pa7,
        gpioa.pa15,
        gpiob.pb0,
        gpiob.pb1,
        gpiob.pb2,
        gpioc.pc3,
        gpioc.pc4,
        gpioc.pc5,
        &mut gpioa.moder,
        &mut gpioa.otyper,
        &mut gpiob.moder,
        &mut gpiob.otyper,
        &mut gpioc.moder,
        &mut gpioc.otyper,
        clocks);

    let blue_grad = Image::gradient(Color::BLUE);
    for i in 0..8 {
                    defmt::trace!(
                        "Ligne {} : {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{}",
                        i,
                        blue_grad[(i, 0)].r, blue_grad[(i, 0)].g, blue_grad[(i, 0)].b,
                        blue_grad[(i, 1)].r, blue_grad[(i, 1)].g, blue_grad[(i, 1)].b,
                        blue_grad[(i, 2)].r, blue_grad[(i, 2)].g, blue_grad[(i, 2)].b,
                        blue_grad[(i, 3)].r, blue_grad[(i, 3)].g, blue_grad[(i, 3)].b,
                        blue_grad[(i, 4)].r, blue_grad[(i, 4)].g, blue_grad[(i, 4)].b,
                        blue_grad[(i, 5)].r, blue_grad[(i, 5)].g, blue_grad[(i, 5)].b,
                        blue_grad[(i, 6)].r, blue_grad[(i, 6)].g, blue_grad[(i, 6)].b,
                        blue_grad[(i, 7)].r, blue_grad[(i, 7)].g, blue_grad[(i, 7)].b
                    );
                }
    loop {
        led_matrix.display_image(&blue_grad);
    }

}


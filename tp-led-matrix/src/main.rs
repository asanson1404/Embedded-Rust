#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

//use stm32l4 as _;   // Just to link it in the executable (it provides the vector table)
use stm32l4xx_hal::{pac, prelude::*};
//use panic_rtt_target as _;
use panic_probe as _;
//use rtt_target::{rtt_init_print, rprintln};
use defmt_rtt as _;
use tp_led_matrix::{Image, Color};
use tp_led_matrix::Matrix;
use dwt_systick_monotonic::DwtSystick;
use dwt_systick_monotonic::ExtU32;

/* Définition d'un panic handler (plus utile en fin de TP)
#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
   loop {}
}
*/

#[rtic::app(device = pac, dispatchers = [USART2])]
mod app {

    use super::*;

    #[shared]
    struct Shared {}
    
    #[local]
    struct Local {
        matrix: Matrix,
        image: Image,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MyMonotonic = DwtSystick<80_000_000>;
    type Instant = <MyMonotonic as rtic::Monotonic>::Instant;


    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {

        defmt::info!("defmt correctly initialized");

        let mut _cp = cx.core;
        let dp = cx.device;

        // Create an instance of MyMonotonic timer and return it as the third element of the tuple
        let mut mono = DwtSystick::new(&mut _cp.DCB, _cp.DWT, _cp.SYST, 80_000_000);

        //<============== Initialize the clocks, hardware and matrix using your existing code ====================>

        // Get high-level representations of hardware modules
        let mut rcc = dp.RCC.constrain();
        let mut flash = dp.FLASH.constrain();
        let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

        // Setup the clocks at 80MHz using HSI (by default since HSE/MSI are not configured).
        // The flash wait states will be configured accordingly.
        let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);

        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
        let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);

        let led_matrix = Matrix::new(
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

        let my_image = Image::gradient(Color::BLUE);

        // The display task gets spawned after init() terminates.
        display::spawn(mono.now()).unwrap();

        // Return the resources and the monotonic timer
        (Shared {}, Local { matrix: led_matrix, image: my_image }, init::Monotonics(mono))
    }

    #[idle(local = [count: u32 = 0])]
    fn idle(_cx: idle::Context) -> ! {

        loop {
            // Test if the program enter in the idle task
            /*
            if *_cx.local.count == 10_000 {
                defmt::info!("idle loop has made 10 000 count ");
                *_cx.local.count = 0;
            }
            else {
                *_cx.local.count += 1;
            }
            */
        }
    }

    #[task(local = [matrix, image, next_line: usize = 0])]
    fn display(cx: display::Context, at: Instant) {

        // Display line next_line (cx.local.next_line) of
        // the image (cx.local.image) on the matrix (cx.local.matrix).
        // All those are mutable references.
        let pixels = cx.local.image.row(*cx.local.next_line);
        cx.local.matrix.send_row(*cx.local.next_line, pixels);

        // Increment next_line up to 7 and wraparound to 0
        *cx.local.next_line =(*cx.local.next_line + 1) % 8;

        // The display task gets respawned as soon as the display task terminates 
        // since no task with a higher priority will be waiting
        let mut at1 = at;
        at1 += 1.secs()/(8*60);
        display::spawn_at(at1, at1).unwrap();
    }

}


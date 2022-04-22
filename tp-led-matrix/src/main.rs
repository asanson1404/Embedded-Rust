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
use stm32l4xx_hal::serial::{Config, Event, Rx, Serial};
use core::mem;

/* Définition d'un panic handler (plus utile en fin de TP)
#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
   loop {}
}
*/

#[rtic::app(device = pac, dispatchers = [USART2, USART3])]
mod app {

    use tp_led_matrix::image::Default;

    use super::*;

    #[shared]
    struct Shared {image: Image}
    
    #[local]
    struct Local {
        matrix:     Matrix,
        usart1_rx:  Rx<stm32l4xx_hal::pac::USART1>,
        next_image: Image
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

        // Configuration of PB6 et PB7 into the right mode
        let tx_pb6 = gpiob.pb6.into_alternate::<7>(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
        let rx_pb7 = gpiob.pb7.into_alternate::<7>(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

        // Instanciate the Config structure for serial port usart1
        // and set the baudrate to 38400 bits per second
        let  usart1_config = Config::default().baudrate(38_400_u32.bps());

        // Initialize the serial port using the above structure
        let mut serial = Serial::usart1(dp.USART1, (tx_pb6, rx_pb7), usart1_config, clocks, &mut rcc.apb2);

        // Enable the "RX not empty event"
        serial.listen(Event::Rxne);

        // Get the receiver part of the serial port 
        let serial_rx = serial.split().1 ;

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

        let def_image = Image::default();
        let next_image= Image::default();

        // Usefull when the field image was in the Local structure
        //let my_image = Image::gradient(Color::BLUE);

        // The display and rotate_image tasks get spawned after init() terminates.
        display::spawn(mono.now()).unwrap();
        //rotate_image::spawn(mono.now(), 0).unwrap();

        // Return the resources and the monotonic timer
        (Shared { image: def_image }, Local { matrix: led_matrix, usart1_rx: serial_rx, next_image }, init::Monotonics(mono))
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

    #[task(binds = USART1,
           local = [usart1_rx, next_image, next_pos: usize = 0],
           shared = [image])]
    fn receive_byte(mut cx: receive_byte::Context)
    {
        let next_image: &mut Image = cx.local.next_image;
        let next_pos: &mut usize = cx.local.next_pos;
        if let Ok(b) = cx.local.usart1_rx.read() {

            // Handle the incoming byte according to the SE203 protocol
            // and update next_image
            // Do not forget that next_image.as_mut() might be handy here!
            if b == 0xff {
                *next_pos = 0;
                return;
            }
            next_image.as_mut()[*next_pos] = b;
            *next_pos += 1 ;

            // If the received image is complete, make it available to
            // the display task.
            if *next_pos == 8 * 8 * 3 {
                cx.shared.image.lock(|image| {
                    // Replace the image content by the new one, for example
                    // by swapping them, and reset next_pos
                    mem::swap(image, next_image);
                });
                *next_pos = 0;
            }
        }
    }
 

    /// Task which modifies the image shared ressource every second. 
    /*#[task(shared = [image], priority = 1)]
    fn rotate_image(mut cx: rotate_image::Context, at: Instant, mut color_index: usize) {
        
        cx.shared.image.lock(|image| {
            match color_index {
                0 => *image = Image::gradient(Color::RED),
                1 => *image = Image::gradient(Color::GREEN),
                2 => *image = Image::gradient(Color::BLUE),
                _ => panic!("Incompatible color index"),
            }
        });
        color_index = (color_index + 1) % 3;
        rotate_image::spawn_after(1.secs(), at, color_index).unwrap();
    }*/

    #[task(local = [matrix, next_line: usize = 0], shared = [image], priority = 2)]
    fn display(mut cx: display::Context, at: Instant) {
        
        // Display line next_line (cx.local.next_line) of
        // the image (cx.local.image) on the matrix (cx.local.matrix).
        // All those are mutable references.
        cx.shared.image.lock(|image| {
            // Here you can use image, which is a &mut Image,
            // to display the appropriate row
            cx.local.matrix.send_row(*cx.local.next_line, image.row(*cx.local.next_line));

            // Increment next_line up to 7 and wraparound to 0
            *cx.local.next_line = (*cx.local.next_line + 1) % 8;
        });

        // The display task gets respawned as soon as the display task terminates 
        // since no task with a higher priority will be waiting
        let mut at1 = at;
        at1 += 1.secs()/(8*60);
        display::spawn_at(at1, at1).unwrap();
    }

}


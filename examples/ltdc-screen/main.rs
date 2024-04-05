#![deny(warnings)]
#![no_main]
#![no_std]

// Required
extern crate panic_semihosting;

use cortex_m_rt::entry;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::{Rgb565, RgbColor},
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    text::Text,
};

use stm32f4xx_hal::{
    ltdc::{BluePins, GreenPins, Layer, LtdcPins, PixelFormat, RedPins},
    pac,
    prelude::*,
    rcc::Rcc,
};

mod screen;

// DIMENSIONS
const WIDTH: u16 = 480;
const HEIGHT: u16 = 272;

// Graphics framebuffer
const FB_GRAPHICS_SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);
static mut FB_LAYER1: [u16; FB_GRAPHICS_SIZE] = [0; FB_GRAPHICS_SIZE];

#[entry]
fn main() -> ! {
    let perif = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();

    let rcc_hal: Rcc = perif.RCC.constrain();

    // Set up pins
    let _gpioa = perif.GPIOA.split();
    let _gpiob = perif.GPIOB.split();
    let gpioe = perif.GPIOE.split();
    let gpiog = perif.GPIOG.split();
    let gpioh = perif.GPIOH.split();
    let gpioi = perif.GPIOI.split();
    let gpioj = perif.GPIOJ.split();
    let gpiok = perif.GPIOK.split();

    // This is likely the pinout for the for the 32F746GDISCOVERY
    // (https://www.st.com/resource/en/user_manual/um1670-discovery-kit-with-stm32f429zi-mcu-stmicroelectronics.pdf)
    // Define each pin with a name that reflects its purpose according to the pinout table
    let ltdc_r0 = gpioi.pi15; // LTDC_R0
    let ltdc_r1 = gpioj.pj0;  // LTDC_R1
    let ltdc_r2 = gpioj.pj1;  // LTDC_R2
    let ltdc_r3 = gpioj.pj2;  // LTDC_R3
    let ltdc_r4 = gpioj.pj3;  // LTDC_R4
    let ltdc_r5 = gpioj.pj4;  // LTDC_R5
    let ltdc_r6 = gpioj.pj5;  // LTDC_R6
    let ltdc_r7 = gpioj.pj6;  // LTDC_R7

    let ltdc_g0 = gpioj.pj7;  // LTDC_G0
    let ltdc_g1 = gpioj.pj8;  // LTDC_G1
    let ltdc_g2 = gpioj.pj9;  // LTDC_G2
    let ltdc_g3 = gpioj.pj10; // LTDC_G3
    let ltdc_g4 = gpioj.pj11; // LTDC_G4
    let ltdc_g5 = gpiok.pk0;  // LTDC_G5
    let ltdc_g6 = gpiok.pk1;  // LTDC_G6
    let ltdc_g7 = gpiok.pk2;  // LTDC_G7

    let ltdc_b0 = gpioe.pe4;  // LTDC_B0
    let ltdc_b1 = gpioj.pj13; // LTDC_B1
    let ltdc_b2 = gpioj.pj14; // LTDC_B2
    let ltdc_b3 = gpioj.pj15; // LTDC_B3
    let ltdc_b4 = gpiog.pg12; // LTDC_B4
    let ltdc_b5 = gpiok.pk4;  // LTDC_B5
    let ltdc_b6 = gpiok.pk5;  // LTDC_B6
    let ltdc_b7 = gpiok.pk6;  // LTDC_B7

    let ltdc_hsync = gpioi.pi10; // LTDC_HSYNC
    let ltdc_vsync = gpioi.pi9;  // LTDC_VSYNC
    let ltdc_de = gpiok.pk7;     // LTDC_DE
    let ltdc_clk = gpioi.pi14;   // LTDC_CLK

    let pins = LtdcPins::new(
        RedPins::new(ltdc_r0, ltdc_r1, ltdc_r2, ltdc_r3, ltdc_r4, ltdc_r5, ltdc_r6, ltdc_r7),
        GreenPins::new(ltdc_g0, ltdc_g1, ltdc_g2, ltdc_g3, ltdc_g4, ltdc_g5, ltdc_g6, ltdc_g7),
        BluePins::new(ltdc_b0, ltdc_b1, ltdc_b2, ltdc_b3, ltdc_b4, ltdc_b5, ltdc_b6, ltdc_b7),
        ltdc_hsync,
        ltdc_vsync,
        ltdc_de,
        ltdc_clk,
    );

    // HSE osc out in High Z
    gpioh.ph1.into_floating_input();
    let _clocks = rcc_hal
        .cfgr
        .use_hse(25.MHz())
        .bypass_hse_oscillator()
        .sysclk(216.MHz())
        .hclk(216.MHz())
        .freeze();

    // LCD enable: set it low first to avoid LCD bleed while setting up timings
    let mut disp_on = gpioi.pi12.into_push_pull_output();
    disp_on.set_low();

    // LCD backlight enable
    let mut backlight = gpiok.pk3.into_push_pull_output();
    backlight.set_high();

    let mut display = screen::Stm32F7DiscoDisplay::new(perif.LTDC, perif.DMA2D, pins);
    display
        .controller
        .config_layer(Layer::L1, unsafe { &mut FB_LAYER1 }, PixelFormat::RGB565);

    display.controller.enable_layer(Layer::L1);
    display.controller.reload();

    let display = &mut display;

    // LCD enable: activate LCD !
    disp_on.set_high();

    Rectangle::new(Point::new(0, 0), Size::new(479, 271))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::new(0, 0b11110, 0b11011)))
        .draw(display)
        .ok();

    let c1 = Circle::new(Point::new(20, 20), 2 * 8)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::new(0, 63, 0)));

    let c2 = Circle::new(Point::new(25, 20), 2 * 8)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::new(31, 0, 0)));

    let t = Text::new(
        "Hello Rust!",
        Point::new(100, 100),
        MonoTextStyle::new(&FONT_6X9, RgbColor::WHITE),
    );

    c1.draw(display).ok();
    c2.draw(display).ok();
    t.draw(display).ok();

    for i in 0..300 {
        Circle::new(Point::new(20 + i, 20), 2 * 8)
            .into_styled(PrimitiveStyle::with_fill(RgbColor::GREEN))
            .draw(display)
            .ok();
    }

    #[allow(clippy::empty_loop)]
    loop {}
}

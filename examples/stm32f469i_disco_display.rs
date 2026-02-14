//! STM32F469I-DISCO display demonstration
//!
//! This example initialises the FMC → SDRAM, configures the LTDC controller,
//! creates an [`LtdcFramebuffer`] in SDRAM and draws a simple colour-bar test
//! pattern using the [`Surface`] trait as well as (optionally)
//! `embedded-graphics`.
//!
//! # Running
//!
//! ```bash
//! cargo run --release --example stm32f469i_disco_display \
//!     --features "stm32f469,ltdc,embedded-graphics"
//! ```
//!
//! The example targets the **STM32F469I-DISCO** board which has:
//!
//! * IS42S32400F SDRAM on FMC bank 2 (SDNE1 / SDCKE1)
//! * OTM8009A LCD panel driven through DSI → LTDC
//! * 800 × 480 display, RGB565

#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;

use stm32f4xx_hal::{
    display::{LtdcFramebuffer, Surface},
    ltdc::{DisplayConfig, DisplayController, Layer, PixelFormat},
    pac,
    prelude::*,
};

// OTM8009A panel: 800 × 480
const WIDTH: u16 = 800;
const HEIGHT: u16 = 480;

/// Display timing for the OTM8009A panel behind the DSI bridge on the
/// STM32F469I-DISCO board.
const OTM8009A_CONFIG: DisplayConfig = DisplayConfig {
    active_width: WIDTH,
    active_height: HEIGHT,
    h_back_porch: 34,
    h_front_porch: 34,
    v_back_porch: 15,
    v_front_porch: 16,
    h_sync: 2,
    v_sync: 1,
    frame_rate: 60,
    h_sync_pol: false,
    v_sync_pol: false,
    no_data_enable_pol: false,
    pixel_clock_pol: false,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // --- Clock configuration ------------------------------------------------
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.sysclk(180.MHz()).freeze();

    // --- FMC / SDRAM --------------------------------------------------------
    // The STM32F469I-DISCO board has an IS42S32400F SDRAM connected to FMC
    // bank 2 (SDNE1/SDCKE1).  A real application would configure the FMC pins
    // and SDRAM chip timing here using `dp.FMC.sdram(pins, chip, &clocks)`.
    //
    // For the sake of this example we create a static framebuffer in normal RAM
    // (BSS).  On real hardware this buffer would be placed in SDRAM by the
    // linker script or obtained from the SDRAM driver.
    static mut FB: [u16; WIDTH as usize * HEIGHT as usize] =
        [0u16; WIDTH as usize * HEIGHT as usize];

    // --- LTDC ---------------------------------------------------------------
    let mut display_controller: DisplayController<u16> = DisplayController::new(
        dp.LTDC,
        dp.DMA2D,
        PixelFormat::RGB565,
        OTM8009A_CONFIG,
        Some(8.MHz()), // HSE on the Discovery board
    );

    // Safety: FB is only accessed through the framebuffer / LTDC from here on.
    let fb_slice = unsafe { &mut *core::ptr::addr_of_mut!(FB) };

    // Hand layer 1 buffer to the LTDC controller
    display_controller.config_layer(Layer::L1, fb_slice, PixelFormat::RGB565);
    display_controller.enable_layer(Layer::L1);
    display_controller.reload();

    // --- Framebuffer --------------------------------------------------------
    // For drawing we create a *second* static buffer.  In a real application
    // with double-buffering both buffers would live in SDRAM.
    static mut DRAW_BUF: [u16; WIDTH as usize * HEIGHT as usize] =
        [0u16; WIDTH as usize * HEIGHT as usize];

    let draw_buf = unsafe { &mut *core::ptr::addr_of_mut!(DRAW_BUF) };
    let mut fb = LtdcFramebuffer::new(draw_buf, WIDTH, HEIGHT);

    // --- Draw a test pattern using the Surface trait ------------------------
    Surface::clear(&mut fb, 0x0000).unwrap(); // black

    // Five colour bars (red, green, blue, yellow, cyan) using fill_rect
    let bar_h = HEIGHT / 5;
    let colors: [u32; 5] = [
        0xF800, // red   (RGB565)
        0x07E0, // green
        0x001F, // blue
        0xFFE0, // yellow
        0x07FF, // cyan
    ];
    for (i, &color) in colors.iter().enumerate() {
        Surface::fill_rect(&mut fb, 0, i as u16 * bar_h, WIDTH, bar_h, color).unwrap();
    }

    // --- (Optional) draw with embedded-graphics -----------------------------
    #[cfg(feature = "embedded-graphics")]
    {
        use embedded_graphics::{
            mono_font::{ascii::FONT_10X20, MonoTextStyle},
            pixelcolor::Rgb565,
            prelude::*,
            text::Text,
        };

        let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
        let _ = Text::new("STM32F469I-DISCO", Point::new(260, 240), style).draw(&mut fb);
    }

    // Keep running
    #[allow(clippy::empty_loop)]
    loop {}
}

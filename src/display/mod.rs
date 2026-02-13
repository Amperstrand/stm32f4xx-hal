//! # Display Transport Layer
//!
//! This module provides **transport layers** for driving external display panels.
//! Following the *HAL = Transport* architecture, the HAL provides the communication
//! infrastructure while external crates supply panel-specific initialisation
//! sequences and driver logic.
//!
//! ## Supported Transports
//!
//! | Transport | Feature flag   | Trait implemented                          | Typical panels             |
//! |-----------|----------------|--------------------------------------------|----------------------------|
//! | DSI       | `dsihost`      | `DsiHostCtrlIo` (embedded-display-controller) | OTM8009A, NT35510       |
//! | SPI       | `spi_display`  | `WriteOnlyDataCommand` (display-interface) | ST7789, ILI9341, SSD1306   |
//!
//! ## Architecture
//!
//! ```text
//! ┌────────────────────┐                            ┌──────────────────────┐
//! │   External Driver  │  WriteOnlyDataCommand /    │   stm32f4xx-hal      │
//! │  (st7789, ili9341, │  DsiHostCtrlIo             │                      │
//! │   otm8009a, …)     │◄──────────────────────────►│  display::spi / dsi  │
//! └────────────────────┘                            └──────────────────────┘
//! ```
//!
//! ## Usage
//!
//! Enable the appropriate feature in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! stm32f4xx-hal = { version = "0.23", features = ["stm32f411", "spi_display"] }
//! ```
//!
//! Then create the transport and pass it to your display driver:
//!
//! ```rust,ignore
//! use stm32f4xx_hal::display::SpiDisplay;
//!
//! let spi_display = SpiDisplay::new(spi_device, dc_pin);
//! let mut driver = st7789::ST7789::new(spi_display, ...);
//! ```

// --- SPI transport ---------------------------------------------------------
#[cfg(feature = "spi_display")]
pub mod spi;
#[cfg(feature = "spi_display")]
pub use spi::SpiDisplay;

// --- DSI re-exports --------------------------------------------------------
#[cfg(feature = "dsihost")]
pub use crate::dsi::{
    DsiChannel, DsiCmdModeTransmissionKind, DsiConfig, DsiHost, DsiHostCtrlIo, DsiMode,
    DsiPhyTimers, DsiPllConfig, DsiReadCommand, DsiRefreshHandle, DsiWriteCommand,
};

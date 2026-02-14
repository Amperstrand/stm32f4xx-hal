//! Display support for LTDC-based framebuffers and a unified Surface trait
//!
//! This module provides:
//! - [`LtdcFramebuffer`]: A framebuffer backed by a `&'static mut [u16]` slice
//!   (typically located in SDRAM), configured for use with the LTDC controller.
//! - [`Surface`]: A trait offering a unified drawing interface that works on both
//!   LTDC (e.g. STM32F469) and non-LTDC (e.g. STM32F413) boards.
//!
//! When the `embedded-graphics` feature is enabled, `LtdcFramebuffer<u16>` implements
//! the `embedded_graphics_core::draw_target::DrawTarget` trait for `Rgb565` pixels.

mod framebuffer;
mod surface;

pub use framebuffer::LtdcFramebuffer;
pub use surface::Surface;

//! LTDC framebuffer wrapping an SDRAM-backed buffer

use crate::ltdc::SupportedWord;

/// Framebuffer for use with the LTDC display controller.
///
/// Wraps a `&'static mut [T]` slice (typically allocated in SDRAM) and
/// exposes helpers for pixel-level access.  When the `embedded-graphics`
/// Cargo feature is enabled an [`embedded_graphics_core::draw_target::DrawTarget`]
/// implementation is provided for `LtdcFramebuffer<u16>` (RGB565).
pub struct LtdcFramebuffer<T: 'static + SupportedWord> {
    buffer: &'static mut [T],
    width: u16,
    height: u16,
}

impl<T: 'static + SupportedWord> LtdcFramebuffer<T> {
    /// Create a new framebuffer from an SDRAM-backed slice.
    ///
    /// # Panics
    ///
    /// Panics if `buffer.len() != width * height`.
    pub fn new(buffer: &'static mut [T], width: u16, height: u16) -> Self {
        assert_eq!(
            buffer.len(),
            width as usize * height as usize,
            "buffer length must equal width * height"
        );
        Self {
            buffer,
            width,
            height,
        }
    }

    /// Returns the width in pixels.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Returns the height in pixels.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Returns a pointer to the underlying buffer (for LTDC layer config).
    pub fn as_ptr(&self) -> *const T {
        self.buffer.as_ptr()
    }

    /// Returns a mutable reference to the underlying slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.buffer
    }

    /// Set the pixel at `(x, y)`.
    ///
    /// # Panics
    ///
    /// Panics if `(x, y)` is out of bounds.
    pub fn set_pixel(&mut self, x: u16, y: u16, color: T) {
        self.buffer[y as usize * self.width as usize + x as usize] = color;
    }
}

impl LtdcFramebuffer<u16> {
    /// Fill the entire framebuffer with a single RGB565 colour.
    pub fn clear_color(&mut self, color: u16) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }

    /// Fill a rectangular region with a single RGB565 colour.
    ///
    /// Coordinates are clamped to the framebuffer bounds.
    pub fn fill_rect(&mut self, x: u16, y: u16, w: u16, h: u16, color: u16) {
        let x0 = (x as usize).min(self.width as usize);
        let y0 = (y as usize).min(self.height as usize);
        let x1 = ((x + w) as usize).min(self.width as usize);
        let y1 = ((y + h) as usize).min(self.height as usize);
        let stride = self.width as usize;
        for row in y0..y1 {
            let start = row * stride + x0;
            let end = row * stride + x1;
            for pixel in &mut self.buffer[start..end] {
                *pixel = color;
            }
        }
    }

    /// Write a block of pixels into the framebuffer starting at `(x, y)`.
    ///
    /// Pixels are written row-major.  Coordinates are clamped to the
    /// framebuffer bounds; excess pixels are silently dropped.
    pub fn write_pixels(&mut self, x: u16, y: u16, w: u16, h: u16, pixels: &[u16]) {
        let stride = self.width as usize;
        let mut idx = 0usize;
        for row in 0..h as usize {
            let py = y as usize + row;
            if py >= self.height as usize {
                break;
            }
            for col in 0..w as usize {
                if idx >= pixels.len() {
                    return;
                }
                let px = x as usize + col;
                if px < self.width as usize {
                    self.buffer[py * stride + px] = pixels[idx];
                }
                idx += 1;
            }
        }
    }
}

// --- embedded-graphics DrawTarget -------------------------------------------

#[cfg(feature = "embedded-graphics")]
mod eg {
    use super::*;
    use embedded_graphics_core::{
        draw_target::DrawTarget,
        geometry::{OriginDimensions, Size},
        pixelcolor::{IntoStorage, Rgb565},
        Pixel,
    };

    impl OriginDimensions for LtdcFramebuffer<u16> {
        fn size(&self) -> Size {
            Size::new(self.width as u32, self.height as u32)
        }
    }

    impl DrawTarget for LtdcFramebuffer<u16> {
        type Color = Rgb565;
        type Error = core::convert::Infallible;

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
        {
            let w = self.width as i32;
            let h = self.height as i32;
            let stride = self.width as usize;

            for Pixel(coord, color) in pixels {
                if coord.x >= 0 && coord.x < w && coord.y >= 0 && coord.y < h {
                    self.buffer[coord.y as usize * stride + coord.x as usize] =
                        color.into_storage();
                }
            }
            Ok(())
        }
    }
}

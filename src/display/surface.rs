//! Unified drawing surface trait
//!
//! [`Surface`] provides a common drawing interface that can be implemented for
//! LTDC-backed framebuffers (e.g. STM32F469I-DISCO) as well as register-based
//! display controllers accessed over FSMC (e.g. STM32F413-DISCO).

use crate::ltdc::SupportedWord;

use super::LtdcFramebuffer;

/// Unified drawing surface.
///
/// Implementations are expected for both LTDC framebuffers and FSMC-driven
/// LCD controllers so that higher-level drawing code can target either backend.
pub trait Surface {
    /// Error type returned by drawing operations.
    type Error;

    /// Returns `(width, height)` in pixels.
    fn size(&self) -> (u16, u16);

    /// Fill the entire surface with a single colour value.
    fn clear(&mut self, color: u32) -> Result<(), Self::Error>;

    /// Fill a rectangular area with a single colour value.
    fn fill_rect(&mut self, x: u16, y: u16, w: u16, h: u16, color: u32)
        -> Result<(), Self::Error>;

    /// Write a block of raw pixel values starting at `(x, y)` in row-major
    /// order.
    fn write_pixels(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        pixels: &[u32],
    ) -> Result<(), Self::Error>;
}

impl<T: 'static + SupportedWord + Copy + From<u16>> Surface for LtdcFramebuffer<T> {
    type Error = core::convert::Infallible;

    fn size(&self) -> (u16, u16) {
        (self.width(), self.height())
    }

    fn clear(&mut self, color: u32) -> Result<(), Self::Error> {
        let c = T::from(color as u16);
        for pixel in self.as_mut_slice().iter_mut() {
            *pixel = c;
        }
        Ok(())
    }

    fn fill_rect(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        color: u32,
    ) -> Result<(), Self::Error> {
        let c = T::from(color as u16);
        let stride = self.width() as usize;
        let fb_w = self.width() as usize;
        let fb_h = self.height() as usize;
        let x0 = (x as usize).min(fb_w);
        let y0 = (y as usize).min(fb_h);
        let x1 = ((x + w) as usize).min(fb_w);
        let y1 = ((y + h) as usize).min(fb_h);
        let buf = self.as_mut_slice();
        for row in y0..y1 {
            let start = row * stride + x0;
            let end = row * stride + x1;
            for pixel in &mut buf[start..end] {
                *pixel = c;
            }
        }
        Ok(())
    }

    fn write_pixels(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        pixels: &[u32],
    ) -> Result<(), Self::Error> {
        let stride = self.width() as usize;
        let fb_w = self.width() as usize;
        let fb_h = self.height() as usize;
        let buf = self.as_mut_slice();
        let mut idx = 0usize;
        for row in 0..h as usize {
            let py = y as usize + row;
            if py >= fb_h {
                break;
            }
            for col in 0..w as usize {
                if idx >= pixels.len() {
                    return Ok(());
                }
                let px = x as usize + col;
                if px < fb_w {
                    buf[py * stride + px] = T::from(pixels[idx] as u16);
                }
                idx += 1;
            }
        }
        Ok(())
    }
}

//! Display surface trait for cross-board portability
//!
//! The [`Surface`] trait provides a unified drawing interface that can be
//! implemented on top of different display back-ends:
//!
//! - **LTDC framebuffer** (F429/F469 boards with external SDRAM)
//! - **SPI command-mode displays** (F411/F413 boards with ST7789 or similar)
//!
//! By programming against `Surface` rather than a concrete display type,
//! application code can be ported between boards with minimal changes.
//!
//! # Example
//!
//! ```rust,ignore
//! use stm32f4xx_hal::display::Surface;
//! use embedded_graphics_core::pixelcolor::Rgb565;
//!
//! fn draw_banner<S: Surface>(surface: &mut S) -> Result<(), S::Error> {
//!     surface.clear(Rgb565::BLACK)?;
//!     surface.fill_rect(10, 10, 100, 20, Rgb565::BLUE)?;
//!     Ok(())
//! }
//! ```

use embedded_graphics_core::pixelcolor::Rgb565;

/// A portable drawing surface.
///
/// This trait abstracts the minimal operations required to draw on a display,
/// regardless of the underlying transport (LTDC framebuffer, SPI, etc.).
pub trait Surface {
    /// Error type for drawing operations.
    type Error;

    /// Return the display dimensions as `(width, height)`.
    fn size(&self) -> (u16, u16);

    /// Fill the entire display with `color`.
    fn clear(&mut self, color: Rgb565) -> Result<(), Self::Error>;

    /// Fill a rectangular region with `color`.
    ///
    /// Coordinates are in display pixels. Implementations should silently
    /// clip any portion that falls outside the visible area.
    fn fill_rect(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        color: Rgb565,
    ) -> Result<(), Self::Error>;

    /// Write raw pixel data starting at `(x, y)`, advancing left-to-right.
    ///
    /// Pixels beyond the right edge of the display should wrap to the next
    /// row. Any pixels that exceed the display bounds should be silently
    /// discarded.
    fn write_pixels(&mut self, x: u16, y: u16, pixels: &[Rgb565]) -> Result<(), Self::Error>;
}

// ── Surface impl for LtdcFramebuffer ────────────────────────────────────────

#[cfg(feature = "ltdc")]
impl Surface for super::framebuffer::LtdcFramebuffer {
    type Error = core::convert::Infallible;

    fn size(&self) -> (u16, u16) {
        (self.width(), self.height())
    }

    fn clear(&mut self, color: Rgb565) -> Result<(), Self::Error> {
        use embedded_graphics_core::draw_target::DrawTarget;
        DrawTarget::clear(self, color)
    }

    fn fill_rect(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        color: Rgb565,
    ) -> Result<(), Self::Error> {
        use embedded_graphics_core::draw_target::DrawTarget;
        use embedded_graphics_core::geometry::Point;
        use embedded_graphics_core::geometry::Size;
        use embedded_graphics_core::primitives::Rectangle;

        let area = Rectangle::new(Point::new(x as i32, y as i32), Size::new(w as u32, h as u32));
        self.fill_solid(&area, color)
    }

    fn write_pixels(&mut self, x: u16, y: u16, pixels: &[Rgb565]) -> Result<(), Self::Error> {
        use super::framebuffer::rgb565_to_u16;

        let w = self.width() as usize;
        let h = self.height() as usize;
        let mut px = x as usize;
        let mut py = y as usize;

        for &color in pixels {
            if py >= h {
                break;
            }
            if px < w {
                self.as_mut_slice()[px + w * py] = rgb565_to_u16(color);
            }
            px += 1;
            if px >= w {
                px = 0;
                py += 1;
            }
        }

        Ok(())
    }
}

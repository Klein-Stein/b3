//! Canvas API.

#![warn(missing_docs)]

mod geometry;

pub use geometry::*;

/// Canvas.
#[derive(Debug)]
pub struct Canvas {
    frame: Rect,
}

impl Canvas {
    /// Creates a new canvas.
    ///
    /// # Parameters:
    /// * `frame` - Canvas frame.
    pub fn new(frame: Rect) -> Self {
        Self {
            frame,
        }
    }

    /// Returns a canvas frame.
    pub fn frame(&self) -> &Rect { &self.frame }

    /// Returns a canvas origin point.
    pub fn origin(&self) -> &Point { &self.frame.origin }

    /// Returns a canvas size.
    pub fn size(&self) -> &Size { &self.frame.size }

    /// Resizes a canvas.
    ///
    /// # Parameters:
    /// * `new_size` - New size.
    pub fn resize(&mut self, new_size: Size) { self.frame.size = new_size; }

    /// Moves a canvas.
    ///
    /// # Parameters:
    /// * `new_origin` - New origin point.
    pub fn shift(&mut self, new_origin: Point) { self.frame.origin = new_origin; }
}

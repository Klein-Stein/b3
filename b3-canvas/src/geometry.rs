/// Size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
    /// Width.
    pub width:  u32,
    /// Height.
    pub height: u32,
}

impl Size {
    /// Creates a new size.
    ///
    /// # Parameters:
    /// * `width` - Width.
    /// * `height` - Height.
    pub fn new<U>(width: U, height: U) -> Self
    where
        U: Into<u32>,
    {
        Self {
            width:  width.into(),
            height: height.into(),
        }
    }
}

/// Point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    /// X-coordinate.
    pub x: i32,
    /// Y-coordinate.
    pub y: i32,
}

impl Point {
    /// Creates a new point.
    ///
    /// # Parameters:
    /// * `x` - X-coordinate.
    /// * `y` - Y-coordinate.
    pub fn new<U>(x: U, y: U) -> Self
    where
        U: Into<i32>,
    {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

/// Rectangle
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect {
    /// Origin point.
    ///
    /// This point sets a position of the top-left corner of the rectangle.
    pub origin: Point,
    /// Rectangle size.
    pub size:   Size,
}

impl Rect {
    /// Creates a new rectangle.
    ///
    /// # Parameters:
    /// * `origin` - Origin point.
    /// * `size` - Rectangle size.
    pub fn new<P, S>(origin: P, size: S) -> Self
    where
        P: Into<Point>,
        S: Into<Size>,
    {
        Self {
            origin: origin.into(),
            size:   size.into(),
        }
    }
}

//! A cross-platform UI framework.

#![warn(missing_docs)]

mod window;

#[doc(inline)]
pub use b3_core as core;
#[doc(inline)]
pub use b3_gui as gui;
pub use window::*;

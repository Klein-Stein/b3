use b3_core::{ActiveApplication, Window, WindowId};

/// A window that contains GUI elements.
///
/// A lightweight wrapper to easily bind views to a [window][Window].
#[derive(Debug)]
pub struct GUIWindow {
    /// Window.
    window: Window,
}

impl GUIWindow {
    /// Returns a window instance.
    pub fn window(&self) -> &Window { &self.window }

    /// Returns a mutable window instance.
    pub fn window_mut(&mut self) -> &mut Window { &mut self.window }

    /// Window ID.
    pub fn id(&self) -> WindowId { self.window.id() }

    /// Makes a window visible.
    ///
    /// # Parameters:
    /// * `app` - Active application.
    pub fn show(&mut self, app: &ActiveApplication) { self.window.show(app); }

    /// Makes a window visible.
    ///
    /// # Parameters:
    /// * `app` - Active application.
    pub fn show_modal(&mut self, app: &ActiveApplication) { self.window.show_modal(app); }
}

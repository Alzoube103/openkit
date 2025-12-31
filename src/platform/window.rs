//! Window management.

use crate::geometry::Size;
use crate::theme::Theme;

use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window as WinitWindow, WindowAttributes, WindowId};

/// Window configuration.
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub size: Size,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub resizable: bool,
    pub decorations: bool,
    pub transparent: bool,
    pub visible: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "OpenKit".to_string(),
            size: Size::new(800.0, 600.0),
            min_size: Some(Size::new(200.0, 150.0)),
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
            visible: true,
        }
    }
}

/// A platform window.
pub struct Window {
    inner: Arc<WinitWindow>,
    config: WindowConfig,
}

impl Window {
    /// Create a new window.
    pub fn new(event_loop: &ActiveEventLoop, config: WindowConfig) -> Result<Self, super::PlatformError> {
        let mut attrs = WindowAttributes::default()
            .with_title(&config.title)
            .with_inner_size(LogicalSize::new(config.size.width, config.size.height))
            .with_resizable(config.resizable)
            .with_decorations(config.decorations)
            .with_transparent(config.transparent)
            .with_visible(config.visible);

        if let Some(min) = config.min_size {
            attrs = attrs.with_min_inner_size(LogicalSize::new(min.width, min.height));
        }

        if let Some(max) = config.max_size {
            attrs = attrs.with_max_inner_size(LogicalSize::new(max.width, max.height));
        }

        let window = event_loop
            .create_window(attrs)
            .map_err(|e| super::PlatformError::WindowCreation(e.to_string()))?;

        Ok(Self {
            inner: Arc::new(window),
            config,
        })
    }

    /// Get the window ID.
    pub fn id(&self) -> WindowId {
        self.inner.id()
    }

    /// Get the inner winit window.
    pub fn inner(&self) -> &WinitWindow {
        &self.inner
    }

    /// Get a reference to the Arc<WinitWindow>.
    pub fn inner_arc(&self) -> Arc<WinitWindow> {
        self.inner.clone()
    }

    /// Get the window title.
    pub fn title(&self) -> &str {
        &self.config.title
    }

    /// Set the window title.
    pub fn set_title(&mut self, title: &str) {
        self.config.title = title.to_string();
        self.inner.set_title(title);
    }

    /// Get the window size.
    pub fn size(&self) -> Size {
        let size = self.inner.inner_size();
        Size::new(size.width as f32, size.height as f32)
    }

    /// Get the window scale factor.
    pub fn scale_factor(&self) -> f64 {
        self.inner.scale_factor()
    }

    /// Request a redraw.
    pub fn request_redraw(&self) {
        self.inner.request_redraw();
    }

    /// Check if the window prefers dark theme.
    pub fn theme(&self) -> Theme {
        match self.inner.theme() {
            Some(winit::window::Theme::Dark) => Theme::Dark,
            Some(winit::window::Theme::Light) => Theme::Light,
            None => Theme::Light,
        }
    }

    /// Set the cursor visibility.
    pub fn set_cursor_visible(&self, visible: bool) {
        self.inner.set_cursor_visible(visible);
    }

    /// Set the window to be maximized.
    pub fn set_maximized(&self, maximized: bool) {
        self.inner.set_maximized(maximized);
    }

    /// Set the window to be minimized.
    pub fn set_minimized(&self, minimized: bool) {
        self.inner.set_minimized(minimized);
    }

    /// Check if the window has focus.
    pub fn has_focus(&self) -> bool {
        self.inner.has_focus()
    }
}

/// Builder for creating windows.
pub struct WindowBuilder {
    config: WindowConfig,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            config: WindowConfig::default(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.config.size = Size::new(width, height);
        self
    }

    pub fn min_size(mut self, width: f32, height: f32) -> Self {
        self.config.min_size = Some(Size::new(width, height));
        self
    }

    pub fn max_size(mut self, width: f32, height: f32) -> Self {
        self.config.max_size = Some(Size::new(width, height));
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.config.resizable = resizable;
        self
    }

    pub fn decorations(mut self, decorations: bool) -> Self {
        self.config.decorations = decorations;
        self
    }

    pub fn transparent(mut self, transparent: bool) -> Self {
        self.config.transparent = transparent;
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.config.visible = visible;
        self
    }

    pub fn build(self, event_loop: &ActiveEventLoop) -> Result<Window, super::PlatformError> {
        Window::new(event_loop, self.config)
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

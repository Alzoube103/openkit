//! Window widget with title bar and OS-appropriate controls.

use super::{Widget, WidgetBase, WidgetId, LayoutContext, PaintContext, EventContext};
use crate::css::{ClassList, WidgetState};
use crate::event::{Event, EventResult, MouseEventKind, MouseButton};
use crate::geometry::{BorderRadius, Color, Point, Rect, Size};
use crate::layout::{Constraints, LayoutResult};
use crate::render::Painter;

/// Operating system style for window controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowControlsStyle {
    /// macOS style: traffic light buttons on the left
    MacOS,
    /// Windows style: icon buttons on the right
    Windows,
    /// GNOME style: icon buttons on the right
    Gnome,
    /// KDE Plasma style: icon buttons on the right
    Kde,
    /// Minimal style: just a close button
    Minimal,
    /// No window controls
    None,
}

impl WindowControlsStyle {
    /// Detect the current OS and return appropriate style.
    pub fn native() -> Self {
        #[cfg(target_os = "macos")]
        return WindowControlsStyle::MacOS;

        #[cfg(target_os = "windows")]
        return WindowControlsStyle::Windows;

        #[cfg(target_os = "linux")]
        {
            // Try to detect desktop environment
            if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
                let desktop = desktop.to_lowercase();
                if desktop.contains("gnome") || desktop.contains("unity") {
                    return WindowControlsStyle::Gnome;
                }
                if desktop.contains("kde") || desktop.contains("plasma") {
                    return WindowControlsStyle::Kde;
                }
            }
            // Default to GNOME style on Linux
            WindowControlsStyle::Gnome
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        WindowControlsStyle::Windows
    }

    /// Check if controls are on the left side.
    pub fn controls_on_left(&self) -> bool {
        matches!(self, WindowControlsStyle::MacOS)
    }
}

impl Default for WindowControlsStyle {
    fn default() -> Self {
        Self::native()
    }
}

/// Window chrome variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindowVariant {
    /// Standard window with title bar
    #[default]
    Standard,
    /// Borderless window (no title bar)
    Borderless,
    /// Utility window (smaller title bar)
    Utility,
    /// Dialog window
    Dialog,
    /// Splash screen (no decorations, centered)
    Splash,
}

/// A window widget with title bar and controls.
///
/// The window provides a container with OS-appropriate decorations
/// including close, minimize, and maximize buttons.
///
/// # Example
///
/// ```rust,ignore
/// use openkit::prelude::*;
///
/// // Standard window with native controls
/// let window = Window::new()
///     .title("My Application")
///     .size(800.0, 600.0)
///     .content(col![16;
///         Label::new("Hello, World!"),
///         Button::new("Click me"),
///     ]);
///
/// // macOS-style window
/// let mac_window = Window::new()
///     .title("macOS Style")
///     .controls_style(WindowControlsStyle::MacOS)
///     .content(Label::new("Traffic lights!"));
///
/// // Borderless window
/// let borderless = Window::new()
///     .variant(WindowVariant::Borderless)
///     .content(Label::new("No title bar"));
/// ```
pub struct Window {
    base: WidgetBase,
    title: String,
    icon: Option<String>,
    variant: WindowVariant,
    controls_style: WindowControlsStyle,
    content: Option<Box<dyn Widget>>,
    /// Title bar height
    title_bar_height: f32,
    /// Whether the window is focused/active
    is_active: bool,
    /// Whether the window is maximized
    is_maximized: bool,
    /// Whether the window can be resized
    resizable: bool,
    /// Whether the window can be minimized
    minimizable: bool,
    /// Whether the window can be maximized
    maximizable: bool,
    /// Hover state for close button
    close_hovered: bool,
    /// Hover state for minimize button
    minimize_hovered: bool,
    /// Hover state for maximize button
    maximize_hovered: bool,
    /// Callbacks
    on_close: Option<Box<dyn Fn() + Send + Sync>>,
    on_minimize: Option<Box<dyn Fn() + Send + Sync>>,
    on_maximize: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Window {
    /// Create a new window.
    pub fn new() -> Self {
        Self {
            base: WidgetBase::new().with_class("window"),
            title: String::new(),
            icon: None,
            variant: WindowVariant::default(),
            controls_style: WindowControlsStyle::default(),
            content: None,
            title_bar_height: 32.0,
            is_active: true,
            is_maximized: false,
            resizable: true,
            minimizable: true,
            maximizable: true,
            close_hovered: false,
            minimize_hovered: false,
            maximize_hovered: false,
            on_close: None,
            on_minimize: None,
            on_maximize: None,
        }
    }

    /// Set the window title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the window icon (emoji or icon name).
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the window variant.
    pub fn variant(mut self, variant: WindowVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the window controls style.
    pub fn controls_style(mut self, style: WindowControlsStyle) -> Self {
        self.controls_style = style;
        self
    }

    /// Set the window content.
    pub fn content<W: Widget + 'static>(mut self, content: W) -> Self {
        self.content = Some(Box::new(content));
        self
    }

    /// Set the title bar height.
    pub fn title_bar_height(mut self, height: f32) -> Self {
        self.title_bar_height = height;
        self
    }

    /// Set whether the window is resizable.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Set whether the window can be minimized.
    pub fn minimizable(mut self, minimizable: bool) -> Self {
        self.minimizable = minimizable;
        self
    }

    /// Set whether the window can be maximized.
    pub fn maximizable(mut self, maximizable: bool) -> Self {
        self.maximizable = maximizable;
        self
    }

    /// Set the close handler.
    pub fn on_close<F>(mut self, handler: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_close = Some(Box::new(handler));
        self
    }

    /// Set the minimize handler.
    pub fn on_minimize<F>(mut self, handler: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_minimize = Some(Box::new(handler));
        self
    }

    /// Set the maximize handler.
    pub fn on_maximize<F>(mut self, handler: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_maximize = Some(Box::new(handler));
        self
    }

    /// Set the active/focused state.
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    /// Add a CSS class.
    pub fn class(mut self, class: &str) -> Self {
        self.base.classes.add(class);
        self
    }

    fn has_title_bar(&self) -> bool {
        !matches!(self.variant, WindowVariant::Borderless | WindowVariant::Splash)
    }

    fn get_title_bar_height(&self) -> f32 {
        if !self.has_title_bar() {
            return 0.0;
        }
        match self.variant {
            WindowVariant::Utility => self.title_bar_height * 0.75,
            _ => self.title_bar_height,
        }
    }

    fn control_button_size(&self) -> f32 {
        match self.controls_style {
            WindowControlsStyle::MacOS => 12.0,
            WindowControlsStyle::Windows => 46.0, // Width
            _ => 32.0,
        }
    }

    fn control_button_height(&self) -> f32 {
        match self.controls_style {
            WindowControlsStyle::MacOS => 12.0,
            WindowControlsStyle::Windows => self.get_title_bar_height(),
            _ => self.get_title_bar_height(),
        }
    }

    fn get_close_button_rect(&self, title_bar_rect: Rect) -> Rect {
        let size = self.control_button_size();
        let height = self.control_button_height();

        match self.controls_style {
            WindowControlsStyle::MacOS => {
                // Left side, first button (close)
                let x = title_bar_rect.x() + 8.0;
                let y = title_bar_rect.y() + (title_bar_rect.height() - size) / 2.0;
                Rect::new(x, y, size, size)
            }
            WindowControlsStyle::Windows => {
                // Right side, last button (close)
                let x = title_bar_rect.x() + title_bar_rect.width() - size;
                Rect::new(x, title_bar_rect.y(), size, height)
            }
            _ => {
                // Right side, last button
                let x = title_bar_rect.x() + title_bar_rect.width() - size - 4.0;
                let y = title_bar_rect.y() + (title_bar_rect.height() - height) / 2.0;
                Rect::new(x, y, size, height)
            }
        }
    }

    fn get_maximize_button_rect(&self, title_bar_rect: Rect) -> Rect {
        let size = self.control_button_size();
        let height = self.control_button_height();

        match self.controls_style {
            WindowControlsStyle::MacOS => {
                // Left side, third button (maximize/zoom)
                let x = title_bar_rect.x() + 8.0 + size * 2.0 + 8.0;
                let y = title_bar_rect.y() + (title_bar_rect.height() - size) / 2.0;
                Rect::new(x, y, size, size)
            }
            WindowControlsStyle::Windows => {
                // Right side, second button from right
                let x = title_bar_rect.x() + title_bar_rect.width() - size * 2.0;
                Rect::new(x, title_bar_rect.y(), size, height)
            }
            _ => {
                let x = title_bar_rect.x() + title_bar_rect.width() - size * 2.0 - 8.0;
                let y = title_bar_rect.y() + (title_bar_rect.height() - height) / 2.0;
                Rect::new(x, y, size, height)
            }
        }
    }

    fn get_minimize_button_rect(&self, title_bar_rect: Rect) -> Rect {
        let size = self.control_button_size();
        let height = self.control_button_height();

        match self.controls_style {
            WindowControlsStyle::MacOS => {
                // Left side, second button (minimize)
                let x = title_bar_rect.x() + 8.0 + size + 4.0;
                let y = title_bar_rect.y() + (title_bar_rect.height() - size) / 2.0;
                Rect::new(x, y, size, size)
            }
            WindowControlsStyle::Windows => {
                // Right side, third button from right
                let x = title_bar_rect.x() + title_bar_rect.width() - size * 3.0;
                Rect::new(x, title_bar_rect.y(), size, height)
            }
            _ => {
                let x = title_bar_rect.x() + title_bar_rect.width() - size * 3.0 - 12.0;
                let y = title_bar_rect.y() + (title_bar_rect.height() - height) / 2.0;
                Rect::new(x, y, size, height)
            }
        }
    }

    fn paint_title_bar(&self, painter: &mut Painter, rect: Rect, ctx: &PaintContext) {
        let theme = ctx.style_ctx.theme;
        let title_bar_height = self.get_title_bar_height();
        let title_bar_rect = Rect::new(rect.x(), rect.y(), rect.width(), title_bar_height);

        // Title bar background
        let bg_color = if self.is_active {
            theme.colors.card
        } else {
            theme.colors.muted
        };

        let r = theme.radii.lg * theme.typography.base_size;
        let radius = BorderRadius::new(r, r, 0.0, 0.0);
        painter.fill_rounded_rect(title_bar_rect, bg_color, radius);

        // Title bar border
        painter.fill_rect(
            Rect::new(title_bar_rect.x(), title_bar_rect.y() + title_bar_rect.height() - 1.0, title_bar_rect.width(), 1.0),
            theme.colors.border,
        );

        // Paint window controls
        self.paint_controls(painter, title_bar_rect, ctx);

        // Paint title
        self.paint_title(painter, title_bar_rect, ctx);
    }

    fn paint_controls(&self, painter: &mut Painter, title_bar_rect: Rect, ctx: &PaintContext) {
        let theme = ctx.style_ctx.theme;

        match self.controls_style {
            WindowControlsStyle::MacOS => {
                self.paint_macos_controls(painter, title_bar_rect, theme);
            }
            WindowControlsStyle::Windows => {
                self.paint_windows_controls(painter, title_bar_rect, theme);
            }
            WindowControlsStyle::Gnome | WindowControlsStyle::Kde => {
                self.paint_linux_controls(painter, title_bar_rect, theme);
            }
            WindowControlsStyle::Minimal => {
                self.paint_minimal_controls(painter, title_bar_rect, theme);
            }
            WindowControlsStyle::None => {}
        }
    }

    fn paint_macos_controls(&self, painter: &mut Painter, title_bar_rect: Rect, theme: &crate::theme::ThemeData) {
        let close_rect = self.get_close_button_rect(title_bar_rect);
        let minimize_rect = self.get_minimize_button_rect(title_bar_rect);
        let maximize_rect = self.get_maximize_button_rect(title_bar_rect);
        let radius = BorderRadius::all(6.0);

        // Colors for macOS traffic lights
        let (close_color, min_color, max_color) = if self.is_active {
            (
                Color::rgb(1.0, 0.376, 0.341),  // Red
                Color::rgb(1.0, 0.741, 0.180),  // Yellow
                Color::rgb(0.157, 0.804, 0.251), // Green
            )
        } else {
            let inactive = theme.colors.muted_foreground.with_alpha(0.5);
            (inactive, inactive, inactive)
        };

        // Close button
        let close_bg = if self.close_hovered { close_color.darken(10.0) } else { close_color };
        painter.fill_rounded_rect(close_rect, close_bg, radius);
        if self.close_hovered {
            // Draw X
            let cx = close_rect.x() + close_rect.width() / 2.0;
            let cy = close_rect.y() + close_rect.height() / 2.0;
            painter.draw_text("×", Point::new(cx - 3.0, cy + 4.0), Color::BLACK.with_alpha(0.6), 10.0);
        }

        // Minimize button
        if self.minimizable {
            let min_bg = if self.minimize_hovered { min_color.darken(10.0) } else { min_color };
            painter.fill_rounded_rect(minimize_rect, min_bg, radius);
            if self.minimize_hovered {
                let cx = minimize_rect.x() + minimize_rect.width() / 2.0;
                let cy = minimize_rect.y() + minimize_rect.height() / 2.0;
                painter.draw_text("−", Point::new(cx - 3.0, cy + 4.0), Color::BLACK.with_alpha(0.6), 10.0);
            }
        }

        // Maximize button
        if self.maximizable {
            let max_bg = if self.maximize_hovered { max_color.darken(10.0) } else { max_color };
            painter.fill_rounded_rect(maximize_rect, max_bg, radius);
            if self.maximize_hovered {
                let cx = maximize_rect.x() + maximize_rect.width() / 2.0;
                let cy = maximize_rect.y() + maximize_rect.height() / 2.0;
                painter.draw_text("+", Point::new(cx - 3.0, cy + 4.0), Color::BLACK.with_alpha(0.6), 10.0);
            }
        }
    }

    fn paint_windows_controls(&self, painter: &mut Painter, title_bar_rect: Rect, theme: &crate::theme::ThemeData) {
        let close_rect = self.get_close_button_rect(title_bar_rect);
        let minimize_rect = self.get_minimize_button_rect(title_bar_rect);
        let maximize_rect = self.get_maximize_button_rect(title_bar_rect);
        let font_size = 10.0;

        let fg_color = theme.colors.foreground;

        // Close button (red on hover)
        let close_bg = if self.close_hovered {
            Color::rgb(0.898, 0.224, 0.208) // Windows red
        } else {
            Color::TRANSPARENT
        };
        let close_fg = if self.close_hovered { Color::WHITE } else { fg_color };
        painter.fill_rect(close_rect, close_bg);
        let cx = close_rect.x() + close_rect.width() / 2.0;
        let cy = close_rect.y() + close_rect.height() / 2.0;
        painter.draw_text("✕", Point::new(cx - 4.0, cy + 4.0), close_fg, font_size);

        // Maximize button
        if self.maximizable {
            let max_bg = if self.maximize_hovered {
                theme.colors.accent.with_alpha(0.1)
            } else {
                Color::TRANSPARENT
            };
            painter.fill_rect(maximize_rect, max_bg);
            let icon = if self.is_maximized { "❐" } else { "☐" };
            let mx = maximize_rect.x() + maximize_rect.width() / 2.0;
            let my = maximize_rect.y() + maximize_rect.height() / 2.0;
            painter.draw_text(icon, Point::new(mx - 4.0, my + 4.0), fg_color, font_size);
        }

        // Minimize button
        if self.minimizable {
            let min_bg = if self.minimize_hovered {
                theme.colors.accent.with_alpha(0.1)
            } else {
                Color::TRANSPARENT
            };
            painter.fill_rect(minimize_rect, min_bg);
            let mx = minimize_rect.x() + minimize_rect.width() / 2.0;
            let my = minimize_rect.y() + minimize_rect.height() / 2.0;
            painter.draw_text("─", Point::new(mx - 4.0, my + 4.0), fg_color, font_size);
        }
    }

    fn paint_linux_controls(&self, painter: &mut Painter, title_bar_rect: Rect, theme: &crate::theme::ThemeData) {
        let close_rect = self.get_close_button_rect(title_bar_rect);
        let minimize_rect = self.get_minimize_button_rect(title_bar_rect);
        let maximize_rect = self.get_maximize_button_rect(title_bar_rect);
        let radius = BorderRadius::all(4.0);
        let font_size = 12.0;

        let fg_color = theme.colors.foreground;
        let hover_bg = theme.colors.accent.with_alpha(0.2);

        // Close button
        let close_bg = if self.close_hovered {
            theme.colors.destructive.with_alpha(0.8)
        } else {
            Color::TRANSPARENT
        };
        let close_fg = if self.close_hovered { Color::WHITE } else { fg_color };
        painter.fill_rounded_rect(close_rect, close_bg, radius);
        let cx = close_rect.x() + close_rect.width() / 2.0;
        let cy = close_rect.y() + close_rect.height() / 2.0;
        painter.draw_text("✕", Point::new(cx - 4.0, cy + 5.0), close_fg, font_size);

        // Maximize button
        if self.maximizable {
            let max_bg = if self.maximize_hovered { hover_bg } else { Color::TRANSPARENT };
            painter.fill_rounded_rect(maximize_rect, max_bg, radius);
            let icon = if self.is_maximized { "❐" } else { "☐" };
            let mx = maximize_rect.x() + maximize_rect.width() / 2.0;
            let my = maximize_rect.y() + maximize_rect.height() / 2.0;
            painter.draw_text(icon, Point::new(mx - 4.0, my + 5.0), fg_color, font_size);
        }

        // Minimize button
        if self.minimizable {
            let min_bg = if self.minimize_hovered { hover_bg } else { Color::TRANSPARENT };
            painter.fill_rounded_rect(minimize_rect, min_bg, radius);
            let mx = minimize_rect.x() + minimize_rect.width() / 2.0;
            let my = minimize_rect.y() + minimize_rect.height() / 2.0;
            painter.draw_text("─", Point::new(mx - 4.0, my + 5.0), fg_color, font_size);
        }
    }

    fn paint_minimal_controls(&self, painter: &mut Painter, title_bar_rect: Rect, theme: &crate::theme::ThemeData) {
        let close_rect = self.get_close_button_rect(title_bar_rect);
        let radius = BorderRadius::all(4.0);

        let close_bg = if self.close_hovered {
            theme.colors.destructive
        } else {
            theme.colors.muted
        };
        let close_fg = if self.close_hovered { Color::WHITE } else { theme.colors.foreground };

        painter.fill_rounded_rect(close_rect, close_bg, radius);
        let cx = close_rect.x() + close_rect.width() / 2.0;
        let cy = close_rect.y() + close_rect.height() / 2.0;
        painter.draw_text("✕", Point::new(cx - 4.0, cy + 5.0), close_fg, 12.0);
    }

    fn paint_title(&self, painter: &mut Painter, title_bar_rect: Rect, ctx: &PaintContext) {
        let theme = ctx.style_ctx.theme;
        let font_size = 13.0;

        let text_color = if self.is_active {
            theme.colors.foreground
        } else {
            theme.colors.muted_foreground
        };

        // Calculate title position
        let title_y = title_bar_rect.y() + (title_bar_rect.height() + font_size * 0.8) / 2.0;

        // For macOS, center the title. For others, position after controls or icon
        let title_x = match self.controls_style {
            WindowControlsStyle::MacOS => {
                // Center title
                let title_width = self.title.len() as f32 * font_size * 0.5;
                title_bar_rect.x() + (title_bar_rect.width() - title_width) / 2.0
            }
            _ => {
                // Left side with padding
                let mut x = title_bar_rect.x() + 12.0;

                // Add icon if present
                if let Some(ref icon) = self.icon {
                    painter.draw_text(icon, Point::new(x, title_y), text_color, font_size);
                    x += font_size + 8.0;
                }

                x
            }
        };

        painter.draw_text(&self.title, Point::new(title_x, title_y), text_color, font_size);
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Window {
    fn id(&self) -> WidgetId {
        self.base.id
    }

    fn type_name(&self) -> &'static str {
        "window"
    }

    fn element_id(&self) -> Option<&str> {
        self.base.element_id.as_deref()
    }

    fn classes(&self) -> &ClassList {
        &self.base.classes
    }

    fn state(&self) -> WidgetState {
        self.base.state
    }

    fn intrinsic_size(&self, ctx: &LayoutContext) -> Size {
        let title_bar_height = self.get_title_bar_height();

        let content_size = if let Some(content) = &self.content {
            content.intrinsic_size(ctx)
        } else {
            Size::new(300.0, 200.0)
        };

        Size::new(content_size.width, content_size.height + title_bar_height)
    }

    fn layout(&mut self, constraints: Constraints, ctx: &LayoutContext) -> LayoutResult {
        let title_bar_height = self.get_title_bar_height();

        // Layout content
        let content_constraints = Constraints {
            min_width: constraints.min_width,
            min_height: (constraints.min_height - title_bar_height).max(0.0),
            max_width: constraints.max_width,
            max_height: (constraints.max_height - title_bar_height).max(0.0),
        };

        let content_size = if let Some(content) = &mut self.content {
            let result = content.layout(content_constraints, ctx);
            content.set_bounds(Rect::new(
                self.base.bounds.x(),
                self.base.bounds.y() + title_bar_height,
                result.size.width,
                result.size.height,
            ));
            result.size
        } else {
            Size::new(constraints.max_width, constraints.max_height - title_bar_height)
        };

        let size = Size::new(
            content_size.width,
            content_size.height + title_bar_height,
        );

        self.base.bounds.size = size;
        LayoutResult::new(size)
    }

    fn paint(&self, painter: &mut Painter, rect: Rect, ctx: &PaintContext) {
        let theme = ctx.style_ctx.theme;
        let radius = BorderRadius::all(theme.radii.lg * theme.typography.base_size);

        // Window shadow
        let shadow_rect = Rect::new(
            rect.x() + 4.0,
            rect.y() + 8.0,
            rect.width(),
            rect.height(),
        );
        painter.fill_rounded_rect(shadow_rect, Color::BLACK.with_alpha(0.2), radius);

        // Window background
        painter.fill_rounded_rect(rect, theme.colors.background, radius);

        // Window border
        painter.stroke_rect(rect, theme.colors.border, 1.0);

        // Paint title bar
        if self.has_title_bar() {
            self.paint_title_bar(painter, rect, ctx);
        }

        // Paint content
        if let Some(content) = &self.content {
            let title_bar_height = self.get_title_bar_height();
            let content_rect = Rect::new(
                rect.x(),
                rect.y() + title_bar_height,
                rect.width(),
                rect.height() - title_bar_height,
            );

            // Clip to content area and paint
            content.paint(painter, content_rect, ctx);
        }
    }

    fn handle_event(&mut self, event: &Event, ctx: &mut EventContext) -> EventResult {
        let title_bar_height = self.get_title_bar_height();
        let title_bar_rect = Rect::new(
            self.base.bounds.x(),
            self.base.bounds.y(),
            self.base.bounds.width(),
            title_bar_height,
        );

        match event {
            Event::Mouse(mouse) => {
                let close_rect = self.get_close_button_rect(title_bar_rect);
                let minimize_rect = self.get_minimize_button_rect(title_bar_rect);
                let maximize_rect = self.get_maximize_button_rect(title_bar_rect);

                let in_close = close_rect.contains(mouse.position);
                let in_minimize = self.minimizable && minimize_rect.contains(mouse.position);
                let in_maximize = self.maximizable && maximize_rect.contains(mouse.position);

                match mouse.kind {
                    MouseEventKind::Move | MouseEventKind::Enter => {
                        let old_close = self.close_hovered;
                        let old_min = self.minimize_hovered;
                        let old_max = self.maximize_hovered;

                        self.close_hovered = in_close;
                        self.minimize_hovered = in_minimize;
                        self.maximize_hovered = in_maximize;

                        if old_close != self.close_hovered
                            || old_min != self.minimize_hovered
                            || old_max != self.maximize_hovered
                        {
                            ctx.request_redraw();
                        }
                    }
                    MouseEventKind::Leave => {
                        if self.close_hovered || self.minimize_hovered || self.maximize_hovered {
                            self.close_hovered = false;
                            self.minimize_hovered = false;
                            self.maximize_hovered = false;
                            ctx.request_redraw();
                        }
                    }
                    MouseEventKind::Up if mouse.button == Some(MouseButton::Left) => {
                        if in_close {
                            if let Some(handler) = &self.on_close {
                                handler();
                            }
                            return EventResult::Handled;
                        }
                        if in_minimize {
                            if let Some(handler) = &self.on_minimize {
                                handler();
                            }
                            return EventResult::Handled;
                        }
                        if in_maximize {
                            self.is_maximized = !self.is_maximized;
                            if let Some(handler) = &self.on_maximize {
                                handler();
                            }
                            ctx.request_redraw();
                            return EventResult::Handled;
                        }
                    }
                    _ => {}
                }

                // Forward to content if not in title bar
                if let Some(content) = &mut self.content {
                    if mouse.position.y > self.base.bounds.y() + title_bar_height {
                        return content.handle_event(event, ctx);
                    }
                }
            }
            _ => {
                // Forward other events to content
                if let Some(content) = &mut self.content {
                    return content.handle_event(event, ctx);
                }
            }
        }

        EventResult::Ignored
    }

    fn bounds(&self) -> Rect {
        self.base.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.base.bounds = bounds;

        // Update content bounds
        let title_bar_height = self.get_title_bar_height();
        if let Some(content) = &mut self.content {
            content.set_bounds(Rect::new(
                bounds.x(),
                bounds.y() + title_bar_height,
                bounds.width(),
                bounds.height() - title_bar_height,
            ));
        }
    }

    fn children(&self) -> &[Box<dyn Widget>] {
        if let Some(content) = &self.content {
            std::slice::from_ref(content)
        } else {
            &[]
        }
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        if let Some(content) = &mut self.content {
            std::slice::from_mut(content)
        } else {
            &mut []
        }
    }
}

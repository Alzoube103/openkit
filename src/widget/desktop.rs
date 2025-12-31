//! Desktop widget with wallpaper and icon grid.

use super::{Widget, WidgetBase, WidgetId, LayoutContext, PaintContext, EventContext};
use crate::css::{ClassList, WidgetState};
use crate::event::{Event, EventResult, MouseEventKind, MouseButton};
use crate::geometry::{Color, Point, Rect, Size};
use crate::layout::{Constraints, LayoutResult};
use crate::render::Painter;

/// Desktop icon item.
#[derive(Debug, Clone)]
pub struct DesktopIcon {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Icon (emoji or icon name)
    pub icon: String,
    /// Grid position (column, row)
    pub position: (usize, usize),
    /// Whether this icon is selected
    pub selected: bool,
}

impl DesktopIcon {
    /// Create a new desktop icon.
    pub fn new(id: impl Into<String>, name: impl Into<String>, icon: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            icon: icon.into(),
            position: (0, 0),
            selected: false,
        }
    }

    /// Set the grid position.
    pub fn at(mut self, col: usize, row: usize) -> Self {
        self.position = (col, row);
        self
    }
}

/// A desktop widget with wallpaper background and icon grid.
///
/// # Example
///
/// ```rust,ignore
/// use openkit::prelude::*;
///
/// let desktop = Desktop::new()
///     .wallpaper_color(Color::from_hex("#1a1a2e").unwrap())
///     .icon(DesktopIcon::new("home", "Home", "🏠").at(0, 0))
///     .icon(DesktopIcon::new("trash", "Trash", "🗑️").at(0, 1))
///     .icon(DesktopIcon::new("terminal", "Terminal", "💻").at(1, 0))
///     .on_icon_click(|id| println!("Clicked: {}", id))
///     .on_icon_double_click(|id| println!("Open: {}", id));
/// ```
pub struct Desktop {
    base: WidgetBase,
    /// Background color (used if no wallpaper image)
    wallpaper_color: Color,
    /// Path to wallpaper image
    wallpaper_path: Option<String>,
    /// Desktop icons
    icons: Vec<DesktopIcon>,
    /// Icon size
    icon_size: f32,
    /// Grid cell size
    cell_size: f32,
    /// Grid padding from edges
    grid_padding: f32,
    /// Currently hovered icon
    hovered_icon: Option<String>,
    /// Last click time for double-click detection
    last_click_time: Option<std::time::Instant>,
    last_click_id: Option<String>,
    /// Callbacks
    on_icon_click: Option<Box<dyn Fn(&str) + Send + Sync>>,
    on_icon_double_click: Option<Box<dyn Fn(&str) + Send + Sync>>,
    on_right_click: Option<Box<dyn Fn(Point) + Send + Sync>>,
}

impl Desktop {
    /// Create a new desktop.
    pub fn new() -> Self {
        Self {
            base: WidgetBase::new().with_class("desktop"),
            wallpaper_color: Color::rgb(0.1, 0.1, 0.15),
            wallpaper_path: None,
            icons: Vec::new(),
            icon_size: 48.0,
            cell_size: 80.0,
            grid_padding: 16.0,
            hovered_icon: None,
            last_click_time: None,
            last_click_id: None,
            on_icon_click: None,
            on_icon_double_click: None,
            on_right_click: None,
        }
    }

    /// Set the wallpaper color.
    pub fn wallpaper_color(mut self, color: Color) -> Self {
        self.wallpaper_color = color;
        self
    }

    /// Set the wallpaper image path.
    pub fn wallpaper(mut self, path: impl Into<String>) -> Self {
        self.wallpaper_path = Some(path.into());
        self
    }

    /// Add a desktop icon.
    pub fn icon(mut self, icon: DesktopIcon) -> Self {
        self.icons.push(icon);
        self
    }

    /// Set multiple icons.
    pub fn icons(mut self, icons: Vec<DesktopIcon>) -> Self {
        self.icons = icons;
        self
    }

    /// Set the icon size.
    pub fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = size;
        self
    }

    /// Set the grid cell size.
    pub fn cell_size(mut self, size: f32) -> Self {
        self.cell_size = size;
        self
    }

    /// Set the icon click handler.
    pub fn on_icon_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.on_icon_click = Some(Box::new(handler));
        self
    }

    /// Set the icon double-click handler.
    pub fn on_icon_double_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.on_icon_double_click = Some(Box::new(handler));
        self
    }

    /// Set the right-click handler (for context menu).
    pub fn on_right_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(Point) + Send + Sync + 'static,
    {
        self.on_right_click = Some(Box::new(handler));
        self
    }

    /// Add a CSS class.
    pub fn class(mut self, class: &str) -> Self {
        self.base.classes.add(class);
        self
    }

    /// Get the rect for an icon at a grid position.
    fn get_icon_rect(&self, col: usize, row: usize) -> Rect {
        let x = self.base.bounds.x() + self.grid_padding + (col as f32) * self.cell_size;
        let y = self.base.bounds.y() + self.grid_padding + (row as f32) * self.cell_size;
        Rect::new(x, y, self.cell_size, self.cell_size)
    }

    /// Find which icon is at a point.
    fn icon_at_point(&self, point: Point) -> Option<&DesktopIcon> {
        for icon in &self.icons {
            let rect = self.get_icon_rect(icon.position.0, icon.position.1);
            if rect.contains(point) {
                return Some(icon);
            }
        }
        None
    }

    /// Select an icon by ID.
    pub fn select_icon(&mut self, id: &str) {
        for icon in &mut self.icons {
            icon.selected = icon.id == id;
        }
    }

    /// Clear selection.
    pub fn clear_selection(&mut self) {
        for icon in &mut self.icons {
            icon.selected = false;
        }
    }
}

impl Default for Desktop {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Desktop {
    fn id(&self) -> WidgetId {
        self.base.id
    }

    fn type_name(&self) -> &'static str {
        "desktop"
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

    fn intrinsic_size(&self, _ctx: &LayoutContext) -> Size {
        // Desktop fills available space
        Size::new(f32::MAX, f32::MAX)
    }

    fn layout(&mut self, constraints: Constraints, _ctx: &LayoutContext) -> LayoutResult {
        let size = Size::new(constraints.max_width, constraints.max_height);
        self.base.bounds.size = size;
        LayoutResult::new(size)
    }

    fn paint(&self, painter: &mut Painter, rect: Rect, ctx: &PaintContext) {
        let theme = ctx.style_ctx.theme;

        // Draw wallpaper background
        painter.fill_rect(rect, self.wallpaper_color);

        // TODO: Draw wallpaper image when image loading is implemented

        // Draw icons
        for icon in &self.icons {
            let cell_rect = self.get_icon_rect(icon.position.0, icon.position.1);
            
            // Selection/hover background
            if icon.selected {
                painter.fill_rect(
                    cell_rect,
                    theme.colors.accent.with_alpha(0.3),
                );
            } else if self.hovered_icon.as_ref() == Some(&icon.id) {
                painter.fill_rect(
                    cell_rect,
                    theme.colors.accent.with_alpha(0.15),
                );
            }

            // Icon
            let icon_x = cell_rect.x() + (cell_rect.width() - self.icon_size) / 2.0;
            let icon_y = cell_rect.y() + 8.0;
            painter.draw_text(
                &icon.icon,
                Point::new(icon_x, icon_y + self.icon_size * 0.8),
                Color::WHITE,
                self.icon_size,
            );

            // Label
            let label_y = icon_y + self.icon_size + 8.0;
            let font_size = 11.0;
            let label_x = cell_rect.x() + (cell_rect.width() - icon.name.len() as f32 * font_size * 0.5) / 2.0;
            painter.draw_text(
                &icon.name,
                Point::new(label_x, label_y + font_size),
                Color::WHITE,
                font_size,
            );
        }
    }

    fn handle_event(&mut self, event: &Event, ctx: &mut EventContext) -> EventResult {
        match event {
            Event::Mouse(mouse) => {
                match mouse.kind {
                    MouseEventKind::Move => {
                        let icon = self.icon_at_point(mouse.position);
                        let new_hovered = icon.map(|i| i.id.clone());
                        if new_hovered != self.hovered_icon {
                            self.hovered_icon = new_hovered;
                            ctx.request_redraw();
                        }
                    }
                    MouseEventKind::Down if mouse.button == Some(MouseButton::Left) => {
                        if let Some(icon) = self.icon_at_point(mouse.position) {
                            let icon_id = icon.id.clone();
                            
                            // Check for double-click
                            let now = std::time::Instant::now();
                            let is_double_click = if let (Some(last_time), Some(last_id)) = 
                                (&self.last_click_time, &self.last_click_id) 
                            {
                                now.duration_since(*last_time).as_millis() < 500 && last_id == &icon_id
                            } else {
                                false
                            };

                            if is_double_click {
                                if let Some(handler) = &self.on_icon_double_click {
                                    handler(&icon_id);
                                }
                                self.last_click_time = None;
                                self.last_click_id = None;
                            } else {
                                self.select_icon(&icon_id);
                                if let Some(handler) = &self.on_icon_click {
                                    handler(&icon_id);
                                }
                                self.last_click_time = Some(now);
                                self.last_click_id = Some(icon_id);
                            }
                            ctx.request_redraw();
                            return EventResult::Handled;
                        } else {
                            // Clicked on empty space - clear selection
                            self.clear_selection();
                            ctx.request_redraw();
                        }
                    }
                    MouseEventKind::Down if mouse.button == Some(MouseButton::Right) => {
                        if let Some(handler) = &self.on_right_click {
                            handler(mouse.position);
                        }
                        return EventResult::Handled;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        EventResult::Ignored
    }

    fn bounds(&self) -> Rect {
        self.base.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.base.bounds = bounds;
    }
}

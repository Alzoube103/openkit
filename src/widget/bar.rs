//! Bar widget for taskbar/panel-like UI elements.

use super::{Widget, WidgetBase, WidgetId, LayoutContext, PaintContext, EventContext};
use crate::css::{ClassList, WidgetState};
use crate::event::{Event, EventResult};
use crate::geometry::{BorderRadius, Color, Rect, Size, EdgeInsets};
use crate::layout::{Constraints, LayoutResult};
use crate::render::Painter;

/// Position of the bar on the screen edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BarPosition {
    /// Top edge of the screen (default)
    #[default]
    Top,
    /// Bottom edge of the screen
    Bottom,
    /// Left edge of the screen
    Left,
    /// Right edge of the screen
    Right,
}

impl BarPosition {
    /// Check if this is a horizontal bar.
    pub fn is_horizontal(&self) -> bool {
        matches!(self, BarPosition::Top | BarPosition::Bottom)
    }

    /// Check if this is a vertical bar.
    pub fn is_vertical(&self) -> bool {
        matches!(self, BarPosition::Left | BarPosition::Right)
    }
}

/// Bar style variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BarVariant {
    /// Solid background (default)
    #[default]
    Solid,
    /// Transparent with blur effect
    Transparent,
    /// Floating bar with rounded corners and margin
    Floating,
    /// Minimal with no background
    Minimal,
}

/// A bar widget for taskbars, panels, docks, and status bars.
///
/// The bar can be positioned at any edge of the screen and contains
/// three sections: start, center, and end.
///
/// # Example
///
/// ```rust,ignore
/// use openkit::prelude::*;
///
/// // Top taskbar with start menu, clock, and system tray
/// let taskbar = Bar::new()
///     .position(BarPosition::Top)
///     .height(40.0)
///     .start(row![8;
///         IconButton::new("☰").tooltip("Applications"),
///     ])
///     .center(row![8;
///         Label::new("12:00 PM"),
///     ])
///     .end(row![8;
///         IconButton::new("🔊"),
///         IconButton::new("📶"),
///         IconButton::new("🔋"),
///     ]);
///
/// // Dock-style bottom bar
/// let dock = Bar::new()
///     .position(BarPosition::Bottom)
///     .variant(BarVariant::Floating)
///     .center(row![4;
///         IconButton::new("📁").size(IconButtonSize::Large),
///         IconButton::new("🌐").size(IconButtonSize::Large),
///         IconButton::new("📧").size(IconButtonSize::Large),
///     ]);
/// ```
pub struct Bar {
    base: WidgetBase,
    position: BarPosition,
    variant: BarVariant,
    /// Height for horizontal bars, width for vertical bars
    thickness: f32,
    /// Padding inside the bar
    padding: EdgeInsets,
    /// Content for the start section (left for horizontal, top for vertical)
    start: Option<Box<dyn Widget>>,
    /// Content for the center section
    center: Option<Box<dyn Widget>>,
    /// Content for the end section (right for horizontal, bottom for vertical)
    end: Option<Box<dyn Widget>>,
    /// Gap between sections
    gap: f32,
    /// Whether the bar should auto-hide
    auto_hide: bool,
    /// Whether the bar is currently visible (for auto-hide)
    visible: bool,
    /// Custom background color
    background: Option<Color>,
    /// Border radius (mainly for floating variant)
    border_radius: Option<f32>,
}

impl Bar {
    /// Create a new bar.
    pub fn new() -> Self {
        Self {
            base: WidgetBase::new().with_class("bar"),
            position: BarPosition::default(),
            variant: BarVariant::default(),
            thickness: 40.0,
            padding: EdgeInsets::symmetric(8.0, 12.0),
            start: None,
            center: None,
            end: None,
            gap: 8.0,
            auto_hide: false,
            visible: true,
            background: None,
            border_radius: None,
        }
    }

    /// Set the bar position.
    pub fn position(mut self, position: BarPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the bar variant.
    pub fn variant(mut self, variant: BarVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the bar thickness (height for horizontal, width for vertical).
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Alias for thickness when horizontal.
    pub fn height(self, height: f32) -> Self {
        self.thickness(height)
    }

    /// Alias for thickness when vertical.
    pub fn width(self, width: f32) -> Self {
        self.thickness(width)
    }

    /// Set the padding.
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = EdgeInsets::all(padding);
        self
    }

    /// Set horizontal and vertical padding.
    pub fn padding_xy(mut self, horizontal: f32, vertical: f32) -> Self {
        self.padding = EdgeInsets::symmetric(vertical, horizontal);
        self
    }

    /// Set the start section content.
    pub fn start<W: Widget + 'static>(mut self, widget: W) -> Self {
        self.start = Some(Box::new(widget));
        self
    }

    /// Set the center section content.
    pub fn center<W: Widget + 'static>(mut self, widget: W) -> Self {
        self.center = Some(Box::new(widget));
        self
    }

    /// Set the end section content.
    pub fn end<W: Widget + 'static>(mut self, widget: W) -> Self {
        self.end = Some(Box::new(widget));
        self
    }

    /// Set the gap between sections.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Enable or disable auto-hide.
    pub fn auto_hide(mut self, auto_hide: bool) -> Self {
        self.auto_hide = auto_hide;
        self
    }

    /// Set a custom background color.
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Set a custom border radius.
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = Some(radius);
        self
    }

    /// Add a CSS class.
    pub fn class(mut self, class: &str) -> Self {
        self.base.classes.add(class);
        self
    }

    /// Show the bar (for auto-hide).
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide the bar (for auto-hide).
    pub fn hide(&mut self) {
        self.visible = false;
    }

    fn get_background_color(&self, theme: &crate::theme::ThemeData) -> Color {
        if let Some(color) = self.background {
            return color;
        }

        match self.variant {
            BarVariant::Solid => theme.colors.card,
            BarVariant::Transparent => theme.colors.background.with_alpha(0.8),
            BarVariant::Floating => theme.colors.card,
            BarVariant::Minimal => Color::TRANSPARENT,
        }
    }

    fn get_border_radius(&self, theme: &crate::theme::ThemeData) -> BorderRadius {
        let radius = match self.variant {
            BarVariant::Floating => {
                self.border_radius.unwrap_or(theme.radii.lg * theme.typography.base_size)
            }
            _ => self.border_radius.unwrap_or(0.0),
        };
        BorderRadius::all(radius)
    }

    fn get_margin(&self) -> f32 {
        match self.variant {
            BarVariant::Floating => 8.0,
            _ => 0.0,
        }
    }
}

impl Default for Bar {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Bar {
    fn id(&self) -> WidgetId {
        self.base.id
    }

    fn type_name(&self) -> &'static str {
        "bar"
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
        let margin = self.get_margin() * 2.0;
        if self.position.is_horizontal() {
            // Horizontal bar: full width, fixed height
            Size::new(f32::MAX, self.thickness + margin)
        } else {
            // Vertical bar: fixed width, full height
            Size::new(self.thickness + margin, f32::MAX)
        }
    }

    fn layout(&mut self, constraints: Constraints, ctx: &LayoutContext) -> LayoutResult {
        let margin = self.get_margin();
        
        let size = if self.position.is_horizontal() {
            Size::new(
                constraints.max_width,
                self.thickness + margin * 2.0,
            )
        } else {
            Size::new(
                self.thickness + margin * 2.0,
                constraints.max_height,
            )
        };

        self.base.bounds.size = size;

        // Calculate content area
        let content_rect = Rect::new(
            self.base.bounds.x() + margin + self.padding.left,
            self.base.bounds.y() + margin + self.padding.top,
            size.width - margin * 2.0 - self.padding.left - self.padding.right,
            size.height - margin * 2.0 - self.padding.top - self.padding.bottom,
        );

        // Layout sections based on orientation
        if self.position.is_horizontal() {
            self.layout_horizontal(content_rect, ctx);
        } else {
            self.layout_vertical(content_rect, ctx);
        }

        LayoutResult::new(size)
    }

    fn paint(&self, painter: &mut Painter, rect: Rect, ctx: &PaintContext) {
        let theme = ctx.style_ctx.theme;
        let margin = self.get_margin();
        let radius = self.get_border_radius(theme);
        let bg_color = self.get_background_color(theme);

        // Calculate bar rect (with margin for floating variant)
        let bar_rect = Rect::new(
            rect.x() + margin,
            rect.y() + margin,
            rect.width() - margin * 2.0,
            rect.height() - margin * 2.0,
        );

        // Draw shadow for floating variant
        if self.variant == BarVariant::Floating {
            let shadow_rect = Rect::new(
                bar_rect.x() + 2.0,
                bar_rect.y() + 4.0,
                bar_rect.width(),
                bar_rect.height(),
            );
            painter.fill_rounded_rect(shadow_rect, Color::BLACK.with_alpha(0.15), radius);
        }

        // Draw background
        if bg_color != Color::TRANSPARENT {
            painter.fill_rounded_rect(bar_rect, bg_color, radius);
        }

        // Draw border for solid variant
        if self.variant == BarVariant::Solid {
            let border_color = theme.colors.border;
            match self.position {
                BarPosition::Top => {
                    painter.fill_rect(
                        Rect::new(bar_rect.x(), bar_rect.y() + bar_rect.height() - 1.0, bar_rect.width(), 1.0),
                        border_color,
                    );
                }
                BarPosition::Bottom => {
                    painter.fill_rect(
                        Rect::new(bar_rect.x(), bar_rect.y(), bar_rect.width(), 1.0),
                        border_color,
                    );
                }
                BarPosition::Left => {
                    painter.fill_rect(
                        Rect::new(bar_rect.x() + bar_rect.width() - 1.0, bar_rect.y(), 1.0, bar_rect.height()),
                        border_color,
                    );
                }
                BarPosition::Right => {
                    painter.fill_rect(
                        Rect::new(bar_rect.x(), bar_rect.y(), 1.0, bar_rect.height()),
                        border_color,
                    );
                }
            }
        }

        // Draw border for floating variant
        if self.variant == BarVariant::Floating {
            painter.stroke_rect(bar_rect, theme.colors.border.with_alpha(0.3), 1.0);
        }

        // Paint children
        if let Some(start) = &self.start {
            start.paint(painter, start.bounds(), ctx);
        }
        if let Some(center) = &self.center {
            center.paint(painter, center.bounds(), ctx);
        }
        if let Some(end) = &self.end {
            end.paint(painter, end.bounds(), ctx);
        }
    }

    fn handle_event(&mut self, event: &Event, ctx: &mut EventContext) -> EventResult {
        // Forward events to children
        if let Some(end) = &mut self.end {
            if end.handle_event(event, ctx) == EventResult::Handled {
                return EventResult::Handled;
            }
        }
        if let Some(center) = &mut self.center {
            if center.handle_event(event, ctx) == EventResult::Handled {
                return EventResult::Handled;
            }
        }
        if let Some(start) = &mut self.start {
            if start.handle_event(event, ctx) == EventResult::Handled {
                return EventResult::Handled;
            }
        }

        EventResult::Ignored
    }

    fn bounds(&self) -> Rect {
        self.base.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.base.bounds = bounds;
    }

    fn children(&self) -> &[Box<dyn Widget>] {
        // Can't easily return all children as a slice, return empty
        &[]
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut []
    }
}

impl Bar {
    fn layout_horizontal(&mut self, content_rect: Rect, ctx: &LayoutContext) {
        let available_width = content_rect.width();
        let content_height = content_rect.height();

        // Calculate child constraints
        let child_constraints = Constraints {
            min_width: 0.0,
            min_height: 0.0,
            max_width: available_width,
            max_height: content_height,
        };

        // Layout start section
        let start_width = if let Some(start) = &mut self.start {
            let result = start.layout(child_constraints, ctx);
            start.set_bounds(Rect::new(
                content_rect.x(),
                content_rect.y() + (content_height - result.size.height) / 2.0,
                result.size.width,
                result.size.height,
            ));
            result.size.width
        } else {
            0.0
        };

        // Layout end section
        let end_width = if let Some(end) = &mut self.end {
            let result = end.layout(child_constraints, ctx);
            end.set_bounds(Rect::new(
                content_rect.x() + available_width - result.size.width,
                content_rect.y() + (content_height - result.size.height) / 2.0,
                result.size.width,
                result.size.height,
            ));
            result.size.width
        } else {
            0.0
        };

        // Layout center section (centered in remaining space)
        if let Some(center) = &mut self.center {
            let center_constraints = Constraints {
                min_width: 0.0,
                min_height: 0.0,
                max_width: available_width - start_width - end_width - self.gap * 2.0,
                max_height: content_height,
            };
            let result = center.layout(center_constraints, ctx);
            
            // Center horizontally
            let center_x = content_rect.x() + (available_width - result.size.width) / 2.0;
            center.set_bounds(Rect::new(
                center_x,
                content_rect.y() + (content_height - result.size.height) / 2.0,
                result.size.width,
                result.size.height,
            ));
        }
    }

    fn layout_vertical(&mut self, content_rect: Rect, ctx: &LayoutContext) {
        let content_width = content_rect.width();
        let available_height = content_rect.height();

        // Calculate child constraints
        let child_constraints = Constraints {
            min_width: 0.0,
            min_height: 0.0,
            max_width: content_width,
            max_height: available_height,
        };

        // Layout start section (top)
        let start_height = if let Some(start) = &mut self.start {
            let result = start.layout(child_constraints, ctx);
            start.set_bounds(Rect::new(
                content_rect.x() + (content_width - result.size.width) / 2.0,
                content_rect.y(),
                result.size.width,
                result.size.height,
            ));
            result.size.height
        } else {
            0.0
        };

        // Layout end section (bottom)
        let end_height = if let Some(end) = &mut self.end {
            let result = end.layout(child_constraints, ctx);
            end.set_bounds(Rect::new(
                content_rect.x() + (content_width - result.size.width) / 2.0,
                content_rect.y() + available_height - result.size.height,
                result.size.width,
                result.size.height,
            ));
            result.size.height
        } else {
            0.0
        };

        // Layout center section (centered in remaining space)
        if let Some(center) = &mut self.center {
            let center_constraints = Constraints {
                min_width: 0.0,
                min_height: 0.0,
                max_width: content_width,
                max_height: available_height - start_height - end_height - self.gap * 2.0,
            };
            let result = center.layout(center_constraints, ctx);
            
            // Center vertically
            let center_y = content_rect.y() + (available_height - result.size.height) / 2.0;
            center.set_bounds(Rect::new(
                content_rect.x() + (content_width - result.size.width) / 2.0,
                center_y,
                result.size.width,
                result.size.height,
            ));
        }
    }
}

//! Tooltip widget for displaying hover information.

use super::{Widget, WidgetBase, WidgetId, LayoutContext, PaintContext, EventContext};
use crate::css::{ClassList, WidgetState};
use crate::event::{Event, EventResult};
use crate::geometry::{BorderRadius, Color, Point, Rect, Size};
use crate::layout::{Constraints, LayoutResult};
use crate::render::Painter;

/// Tooltip position relative to anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPosition {
    /// Above the anchor (default)
    #[default]
    Top,
    /// Below the anchor
    Bottom,
    /// Left of the anchor
    Left,
    /// Right of the anchor
    Right,
}

/// A tooltip widget.
///
/// Tooltips are typically shown on hover to provide additional information.
///
/// # Example
///
/// ```rust,ignore
/// use openkit::prelude::*;
///
/// // Standalone tooltip
/// let tooltip = Tooltip::new("This is helpful information")
///     .position(TooltipPosition::Top);
///
/// // Show at a specific position
/// tooltip.show_at(Point::new(100.0, 200.0));
/// ```
pub struct Tooltip {
    base: WidgetBase,
    text: String,
    position: TooltipPosition,
    anchor_point: Point,
    visible: bool,
    max_width: f32,
}

impl Tooltip {
    /// Create a new tooltip.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            base: WidgetBase::new().with_class("tooltip"),
            text: text.into(),
            position: TooltipPosition::default(),
            anchor_point: Point::ZERO,
            visible: false,
            max_width: 250.0,
        }
    }

    /// Set the position relative to anchor.
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the maximum width.
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Add a CSS class.
    pub fn class(mut self, class: &str) -> Self {
        self.base.classes.add(class);
        self
    }

    /// Show the tooltip at a position.
    pub fn show_at(&mut self, anchor: Point) {
        self.anchor_point = anchor;
        self.visible = true;
    }

    /// Hide the tooltip.
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Check if visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set the text.
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    fn calculate_tooltip_rect(&self) -> Rect {
        let padding = 8.0;
        let font_size = 12.0;

        // Simple width calculation (would need proper text measurement)
        let text_width = (self.text.len() as f32 * font_size * 0.55).min(self.max_width);
        let width = text_width + padding * 2.0;
        let height = font_size + padding * 2.0;

        let (x, y) = match self.position {
            TooltipPosition::Top => (
                self.anchor_point.x - width / 2.0,
                self.anchor_point.y - height - 8.0,
            ),
            TooltipPosition::Bottom => (
                self.anchor_point.x - width / 2.0,
                self.anchor_point.y + 8.0,
            ),
            TooltipPosition::Left => (
                self.anchor_point.x - width - 8.0,
                self.anchor_point.y - height / 2.0,
            ),
            TooltipPosition::Right => (
                self.anchor_point.x + 8.0,
                self.anchor_point.y - height / 2.0,
            ),
        };

        Rect::new(x, y, width, height)
    }
}

impl Widget for Tooltip {
    fn id(&self) -> WidgetId {
        self.base.id
    }

    fn type_name(&self) -> &'static str {
        "tooltip"
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
        if self.visible {
            let rect = self.calculate_tooltip_rect();
            Size::new(rect.width(), rect.height())
        } else {
            Size::ZERO
        }
    }

    fn layout(&mut self, _constraints: Constraints, _ctx: &LayoutContext) -> LayoutResult {
        let rect = self.calculate_tooltip_rect();
        self.base.bounds = rect;
        LayoutResult::new(Size::new(rect.width(), rect.height()))
    }

    fn paint(&self, painter: &mut Painter, _rect: Rect, ctx: &PaintContext) {
        if !self.visible {
            return;
        }

        let theme = ctx.style_ctx.theme;
        let rect = self.calculate_tooltip_rect();
        let radius = BorderRadius::all(theme.radii.sm * theme.typography.base_size);

        // Shadow
        let shadow_rect = Rect::new(rect.x() + 1.0, rect.y() + 2.0, rect.width(), rect.height());
        painter.fill_rounded_rect(shadow_rect, Color::BLACK.with_alpha(0.15), radius);

        // Background
        painter.fill_rounded_rect(rect, theme.colors.popover, radius);
        painter.stroke_rect(rect, theme.colors.border, 1.0);

        // Text
        let text_x = rect.x() + 8.0;
        let text_y = rect.y() + rect.height() * 0.7;
        painter.draw_text(&self.text, Point::new(text_x, text_y), theme.colors.popover_foreground, 12.0);

        // Arrow/pointer (simplified - just a small triangle indicator)
        let arrow_size = 6.0;
        let arrow_color = theme.colors.popover;

        match self.position {
            TooltipPosition::Top => {
                // Arrow pointing down
                let arrow_x = rect.x() + rect.width() / 2.0;
                let arrow_y = rect.y() + rect.height();
                painter.fill_rect(
                    Rect::new(arrow_x - arrow_size / 2.0, arrow_y - 1.0, arrow_size, arrow_size / 2.0),
                    arrow_color,
                );
            }
            TooltipPosition::Bottom => {
                // Arrow pointing up
                let arrow_x = rect.x() + rect.width() / 2.0;
                let arrow_y = rect.y();
                painter.fill_rect(
                    Rect::new(arrow_x - arrow_size / 2.0, arrow_y - arrow_size / 2.0 + 1.0, arrow_size, arrow_size / 2.0),
                    arrow_color,
                );
            }
            _ => {}
        }
    }

    fn handle_event(&mut self, _event: &Event, _ctx: &mut EventContext) -> EventResult {
        EventResult::Ignored
    }

    fn bounds(&self) -> Rect {
        self.base.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.base.bounds = bounds;
    }
}

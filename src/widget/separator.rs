//! Separator widget for visual division.

use super::{Widget, WidgetBase, WidgetId, LayoutContext, PaintContext, EventContext};
use crate::css::{ClassList, WidgetState};
use crate::event::{Event, EventResult};
use crate::geometry::{Rect, Size};
use crate::layout::{Constraints, LayoutResult};
use crate::render::Painter;

/// Separator orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorOrientation {
    /// Horizontal separator (default)
    #[default]
    Horizontal,
    /// Vertical separator
    Vertical,
}

/// A visual separator widget.
///
/// # Example
///
/// ```rust,ignore
/// use openkit::prelude::*;
///
/// col![8;
///     label!("Section 1"),
///     Separator::horizontal(),
///     label!("Section 2"),
/// ]
///
/// row![8;
///     button!("Left"),
///     Separator::vertical(),
///     button!("Right"),
/// ]
/// ```
pub struct Separator {
    base: WidgetBase,
    orientation: SeparatorOrientation,
    thickness: f32,
    margin: f32,
}

impl Separator {
    /// Create a new horizontal separator.
    pub fn horizontal() -> Self {
        Self {
            base: WidgetBase::new().with_class("separator"),
            orientation: SeparatorOrientation::Horizontal,
            thickness: 1.0,
            margin: 0.0,
        }
    }

    /// Create a new vertical separator.
    pub fn vertical() -> Self {
        Self {
            base: WidgetBase::new().with_class("separator"),
            orientation: SeparatorOrientation::Vertical,
            thickness: 1.0,
            margin: 0.0,
        }
    }

    /// Set the thickness.
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set the margin (space on both sides).
    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    /// Add a CSS class.
    pub fn class(mut self, class: &str) -> Self {
        self.base.classes.add(class);
        self
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self::horizontal()
    }
}

impl Widget for Separator {
    fn id(&self) -> WidgetId {
        self.base.id
    }

    fn type_name(&self) -> &'static str {
        "separator"
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
        match self.orientation {
            SeparatorOrientation::Horizontal => Size::new(f32::MAX, self.thickness + self.margin * 2.0),
            SeparatorOrientation::Vertical => Size::new(self.thickness + self.margin * 2.0, f32::MAX),
        }
    }

    fn layout(&mut self, constraints: Constraints, _ctx: &LayoutContext) -> LayoutResult {
        let size = match self.orientation {
            SeparatorOrientation::Horizontal => {
                Size::new(constraints.max_width, self.thickness + self.margin * 2.0)
            }
            SeparatorOrientation::Vertical => {
                Size::new(self.thickness + self.margin * 2.0, constraints.max_height)
            }
        };
        self.base.bounds.size = size;
        LayoutResult::new(size)
    }

    fn paint(&self, painter: &mut Painter, rect: Rect, ctx: &PaintContext) {
        let theme = ctx.style_ctx.theme;
        let color = theme.colors.border;

        let line_rect = match self.orientation {
            SeparatorOrientation::Horizontal => {
                Rect::new(
                    rect.x() + self.margin,
                    rect.y() + self.margin,
                    rect.width() - self.margin * 2.0,
                    self.thickness,
                )
            }
            SeparatorOrientation::Vertical => {
                Rect::new(
                    rect.x() + self.margin,
                    rect.y() + self.margin,
                    self.thickness,
                    rect.height() - self.margin * 2.0,
                )
            }
        };

        painter.fill_rect(line_rect, color);
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

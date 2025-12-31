//! Progress bar widget.

use super::{Widget, WidgetBase, WidgetId, LayoutContext, PaintContext, EventContext};
use crate::css::{ClassList, WidgetState};
use crate::event::{Event, EventResult};
use crate::geometry::{BorderRadius, Color, Rect, Size};
use crate::layout::{Constraints, LayoutResult};
use crate::render::Painter;

/// Progress bar variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressVariant {
    /// Default linear progress bar
    #[default]
    Linear,
    /// Striped animated progress bar
    Striped,
    /// Indeterminate (loading) progress bar
    Indeterminate,
}

/// Progress bar size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressSize {
    /// Small (2px height)
    Small,
    /// Medium (4px height) - default
    #[default]
    Medium,
    /// Large (8px height)
    Large,
}

impl ProgressSize {
    fn height(&self) -> f32 {
        match self {
            ProgressSize::Small => 2.0,
            ProgressSize::Medium => 4.0,
            ProgressSize::Large => 8.0,
        }
    }
}

/// A progress bar widget.
///
/// # Example
///
/// ```rust,ignore
/// use openkit::prelude::*;
///
/// // Determinate progress
/// let progress = Progress::new()
///     .value(0.75)
///     .show_label(true);
///
/// // Indeterminate loading
/// let loading = Progress::new()
///     .variant(ProgressVariant::Indeterminate);
///
/// // Striped progress
/// let striped = Progress::new()
///     .variant(ProgressVariant::Striped)
///     .value(0.5);
/// ```
pub struct Progress {
    base: WidgetBase,
    value: f32,
    variant: ProgressVariant,
    size: ProgressSize,
    color: Option<Color>,
    show_label: bool,
}

impl Progress {
    /// Create a new progress bar.
    pub fn new() -> Self {
        Self {
            base: WidgetBase::new().with_class("progress"),
            value: 0.0,
            variant: ProgressVariant::default(),
            size: ProgressSize::default(),
            color: None,
            show_label: false,
        }
    }

    /// Set the progress value (0.0 to 1.0).
    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(0.0, 1.0);
        self
    }

    /// Set the variant.
    pub fn variant(mut self, variant: ProgressVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the size.
    pub fn size(mut self, size: ProgressSize) -> Self {
        self.size = size;
        self
    }

    /// Set a custom color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set whether to show the percentage label.
    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    /// Add a CSS class.
    pub fn class(mut self, class: &str) -> Self {
        self.base.classes.add(class);
        self
    }

    /// Get the current value.
    pub fn get_value(&self) -> f32 {
        self.value
    }

    /// Set the value programmatically.
    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(0.0, 1.0);
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Progress {
    fn id(&self) -> WidgetId {
        self.base.id
    }

    fn type_name(&self) -> &'static str {
        "progress"
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
        let height = if self.show_label {
            self.size.height() + 20.0
        } else {
            self.size.height()
        };
        Size::new(200.0, height)
    }

    fn layout(&mut self, constraints: Constraints, ctx: &LayoutContext) -> LayoutResult {
        let intrinsic = self.intrinsic_size(ctx);
        let size = Size::new(
            constraints.max_width.min(intrinsic.width.max(constraints.min_width)),
            intrinsic.height,
        );
        self.base.bounds.size = size;
        LayoutResult::new(size)
    }

    fn paint(&self, painter: &mut Painter, rect: Rect, ctx: &PaintContext) {
        let theme = ctx.style_ctx.theme;
        let bar_height = self.size.height();
        let radius = BorderRadius::all(bar_height / 2.0);

        let bar_rect = if self.show_label {
            Rect::new(rect.x(), rect.y(), rect.width(), bar_height)
        } else {
            rect
        };

        // Track background
        painter.fill_rounded_rect(bar_rect, theme.colors.muted, radius);

        // Fill
        let fill_color = self.color.unwrap_or(theme.colors.primary);

        match self.variant {
            ProgressVariant::Linear | ProgressVariant::Striped => {
                let fill_width = bar_rect.width() * self.value;
                let fill_rect = Rect::new(bar_rect.x(), bar_rect.y(), fill_width, bar_rect.height());
                painter.fill_rounded_rect(fill_rect, fill_color, radius);

                // Striped pattern (simplified - would need animation in real implementation)
                if self.variant == ProgressVariant::Striped && fill_width > 0.0 {
                    let stripe_color = Color::WHITE.with_alpha(0.2);
                    let stripe_width: f32 = 10.0;
                    let mut x = bar_rect.x();
                    while x < bar_rect.x() + fill_width {
                        let stripe_rect = Rect::new(
                            x,
                            bar_rect.y(),
                            (stripe_width / 2.0).min(bar_rect.x() + fill_width - x),
                            bar_rect.height(),
                        );
                        painter.fill_rect(stripe_rect, stripe_color);
                        x += stripe_width;
                    }
                }
            }
            ProgressVariant::Indeterminate => {
                // Animated indeterminate bar (simplified - static position)
                let segment_width = bar_rect.width() * 0.3;
                let segment_x = bar_rect.x() + (bar_rect.width() - segment_width) * 0.3; // Would animate
                let segment_rect = Rect::new(segment_x, bar_rect.y(), segment_width, bar_rect.height());
                painter.fill_rounded_rect(segment_rect, fill_color, radius);
            }
        }

        // Label
        if self.show_label && self.variant != ProgressVariant::Indeterminate {
            let label = format!("{:.0}%", self.value * 100.0);
            let label_y = bar_rect.y() + bar_height + 16.0;
            painter.draw_text(
                &label,
                crate::geometry::Point::new(rect.x() + rect.width() - 30.0, label_y),
                theme.colors.foreground,
                12.0,
            );
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

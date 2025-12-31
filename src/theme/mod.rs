//! Theme system for OpenKit.
//!
//! Provides a Tailwind-inspired design token system with light and dark themes.

mod tokens;

pub use tokens::*;

use crate::geometry::Color;
use std::collections::HashMap;

/// Theme variant (light or dark).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    /// Auto-detect from system preference
    Auto,
}

/// The complete theme configuration.
#[derive(Debug, Clone)]
pub struct ThemeData {
    /// Theme variant
    pub variant: Theme,
    /// Whether dark mode is currently active
    pub is_dark: bool,
    /// Color tokens
    pub colors: ThemeColors,
    /// Spacing scale
    pub spacing: SpacingScale,
    /// Typography tokens
    pub typography: Typography,
    /// Border radius tokens
    pub radii: BorderRadii,
    /// Shadow tokens
    pub shadows: Shadows,
    /// Custom CSS variables
    pub custom_vars: HashMap<String, String>,
}

impl ThemeData {
    /// Create the default light theme.
    pub fn light() -> Self {
        Self {
            variant: Theme::Light,
            is_dark: false,
            colors: ThemeColors::light(),
            spacing: SpacingScale::default(),
            typography: Typography::default(),
            radii: BorderRadii::default(),
            shadows: Shadows::light(),
            custom_vars: HashMap::new(),
        }
    }

    /// Create the default dark theme.
    pub fn dark() -> Self {
        Self {
            variant: Theme::Dark,
            is_dark: true,
            colors: ThemeColors::dark(),
            spacing: SpacingScale::default(),
            typography: Typography::default(),
            radii: BorderRadii::default(),
            shadows: Shadows::dark(),
            custom_vars: HashMap::new(),
        }
    }

    /// Set a custom CSS variable.
    pub fn set_var(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.custom_vars.insert(name.into(), value.into());
    }

    /// Get a custom CSS variable.
    pub fn get_var(&self, name: &str) -> Option<&String> {
        self.custom_vars.get(name)
    }

    /// Resolve a CSS variable name to its value.
    pub fn resolve_var(&self, name: &str) -> Option<String> {
        // First check custom vars
        if let Some(value) = self.custom_vars.get(name) {
            return Some(value.clone());
        }

        // Then check built-in tokens
        match name {
            // Colors
            "--background" => Some(self.colors.background.to_css()),
            "--foreground" => Some(self.colors.foreground.to_css()),
            "--primary" => Some(self.colors.primary.to_css()),
            "--primary-foreground" => Some(self.colors.primary_foreground.to_css()),
            "--secondary" => Some(self.colors.secondary.to_css()),
            "--secondary-foreground" => Some(self.colors.secondary_foreground.to_css()),
            "--muted" => Some(self.colors.muted.to_css()),
            "--muted-foreground" => Some(self.colors.muted_foreground.to_css()),
            "--accent" => Some(self.colors.accent.to_css()),
            "--accent-foreground" => Some(self.colors.accent_foreground.to_css()),
            "--destructive" => Some(self.colors.destructive.to_css()),
            "--destructive-foreground" => Some(self.colors.destructive_foreground.to_css()),
            "--border" => Some(self.colors.border.to_css()),
            "--input" => Some(self.colors.input.to_css()),
            "--ring" => Some(self.colors.ring.to_css()),
            "--card" => Some(self.colors.card.to_css()),
            "--card-foreground" => Some(self.colors.card_foreground.to_css()),
            "--popover" => Some(self.colors.popover.to_css()),
            "--popover-foreground" => Some(self.colors.popover_foreground.to_css()),

            // Border radius
            "--radius" => Some(format!("{}rem", self.radii.default)),
            "--radius-sm" => Some(format!("{}rem", self.radii.sm)),
            "--radius-md" => Some(format!("{}rem", self.radii.md)),
            "--radius-lg" => Some(format!("{}rem", self.radii.lg)),
            "--radius-xl" => Some(format!("{}rem", self.radii.xl)),
            "--radius-full" => Some("9999px".to_string()),

            // Spacing
            "--space-1" => Some(format!("{}rem", self.spacing.get(1))),
            "--space-2" => Some(format!("{}rem", self.spacing.get(2))),
            "--space-3" => Some(format!("{}rem", self.spacing.get(3))),
            "--space-4" => Some(format!("{}rem", self.spacing.get(4))),
            "--space-5" => Some(format!("{}rem", self.spacing.get(5))),
            "--space-6" => Some(format!("{}rem", self.spacing.get(6))),
            "--space-8" => Some(format!("{}rem", self.spacing.get(8))),
            "--space-10" => Some(format!("{}rem", self.spacing.get(10))),
            "--space-12" => Some(format!("{}rem", self.spacing.get(12))),

            _ => None,
        }
    }
}

impl Default for ThemeData {
    fn default() -> Self {
        Self::light()
    }
}

/// Extension trait for Color to produce CSS strings.
pub trait ColorExt {
    fn to_css(&self) -> String;
}

impl ColorExt for Color {
    fn to_css(&self) -> String {
        let [r, g, b, a] = self.to_rgba8();
        if a == 255 {
            format!("rgb({}, {}, {})", r, g, b)
        } else {
            format!("rgba({}, {}, {}, {:.3})", r, g, b, self.a)
        }
    }
}

/// Detect the system theme preference.
pub fn detect_system_theme() -> Theme {
    // This will be implemented per-platform
    // For now, default to light
    Theme::Light
}

//! CSS loading and management.
//!
//! Provides functionality to load custom CSS from files and strings,
//! allowing users to override the framework's default styles.
//!
//! # Example
//!
//! ```rust,ignore
//! use openkit::prelude::*;
//! use openkit::css::StyleManager;
//!
//! let mut styles = StyleManager::new();
//!
//! // Load from file
//! styles.load_file("./styles/custom.css")?;
//!
//! // Load from string
//! styles.load_css(r#"
//!     .my-button {
//!         background-color: #3b82f6;
//!         border-radius: 8px;
//!     }
//!     .my-button:hover {
//!         background-color: #2563eb;
//!     }
//! "#)?;
//!
//! // Use in app
//! App::new()
//!     .styles(styles)
//!     .run(|| { /* ... */ });
//! ```

use crate::css::{CssParser, StyleSheet, StyleRule};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Manages CSS stylesheets for the application.
///
/// StyleManager handles loading, parsing, and cascading of CSS from multiple sources:
/// - Framework default styles
/// - Theme styles (light/dark)
/// - User custom styles
/// - Inline styles
#[derive(Debug, Clone, Default)]
pub struct StyleManager {
    /// Default framework styles (lowest priority)
    default_styles: StyleSheet,
    /// Theme-specific styles
    theme_styles: StyleSheet,
    /// User-provided custom styles (highest priority before inline)
    custom_styles: Vec<StyleSheet>,
    /// Named style modules
    modules: HashMap<String, StyleSheet>,
    /// CSS custom properties (variables)
    variables: HashMap<String, String>,
    /// Whether to watch files for changes (hot reload)
    watch_files: bool,
    /// Loaded file paths for hot reload
    loaded_files: Vec<String>,
}

impl StyleManager {
    /// Create a new StyleManager with default framework styles.
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.load_default_styles();
        manager
    }

    /// Create an empty StyleManager without default styles.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Load the framework's default styles.
    fn load_default_styles(&mut self) {
        let default_css = include_str!("default.css");
        match CssParser::parse_stylesheet(default_css) {
            Ok(sheet) => {
                self.default_styles = sheet;
            }
            Err(e) => {
                log::warn!("Failed to parse default CSS: {:?}", e);
            }
        }
    }

    /// Load CSS from a file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the CSS file
    ///
    /// # Returns
    ///
    /// Result indicating success or an error message
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// styles.load_file("./assets/custom.css")?;
    /// styles.load_file("./theme/dark.css")?;
    /// ```
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), CssLoadError> {
        let path = path.as_ref();
        let css = fs::read_to_string(path)
            .map_err(|e| CssLoadError::FileRead {
                path: path.display().to_string(),
                error: e.to_string(),
            })?;

        let sheet = CssParser::parse_stylesheet(&css)
            .map_err(|e| CssLoadError::Parse {
                source: path.display().to_string(),
                error: format!("{:?}", e),
            })?;

        self.custom_styles.push(sheet);
        self.loaded_files.push(path.display().to_string());

        Ok(())
    }

    /// Load CSS from a string.
    ///
    /// # Arguments
    ///
    /// * `css` - CSS content as a string
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// styles.load_css(r#"
    ///     .primary-button {
    ///         background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    ///         color: white;
    ///         padding: 12px 24px;
    ///         border-radius: 8px;
    ///     }
    /// "#)?;
    /// ```
    pub fn load_css(&mut self, css: &str) -> Result<(), CssLoadError> {
        let sheet = CssParser::parse_stylesheet(css)
            .map_err(|e| CssLoadError::Parse {
                source: "<inline>".to_string(),
                error: format!("{:?}", e),
            })?;

        self.custom_styles.push(sheet);
        Ok(())
    }

    /// Load CSS as a named module that can be enabled/disabled.
    ///
    /// # Arguments
    ///
    /// * `name` - Module name for reference
    /// * `css` - CSS content
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// styles.load_module("animations", r#"
    ///     @keyframes fadeIn {
    ///         from { opacity: 0; }
    ///         to { opacity: 1; }
    ///     }
    /// "#)?;
    /// ```
    pub fn load_module(&mut self, name: &str, css: &str) -> Result<(), CssLoadError> {
        let sheet = CssParser::parse_stylesheet(css)
            .map_err(|e| CssLoadError::Parse {
                source: format!("module:{}", name),
                error: format!("{:?}", e),
            })?;

        self.modules.insert(name.to_string(), sheet);
        Ok(())
    }

    /// Load a CSS module from a file.
    pub fn load_module_file<P: AsRef<Path>>(&mut self, name: &str, path: P) -> Result<(), CssLoadError> {
        let path = path.as_ref();
        let css = fs::read_to_string(path)
            .map_err(|e| CssLoadError::FileRead {
                path: path.display().to_string(),
                error: e.to_string(),
            })?;

        self.load_module(name, &css)
    }

    /// Remove a loaded module.
    pub fn unload_module(&mut self, name: &str) -> bool {
        self.modules.remove(name).is_some()
    }

    /// Check if a module is loaded.
    pub fn has_module(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }

    /// Set theme-specific styles.
    pub fn set_theme_styles(&mut self, css: &str) -> Result<(), CssLoadError> {
        let sheet = CssParser::parse_stylesheet(css)
            .map_err(|e| CssLoadError::Parse {
                source: "<theme>".to_string(),
                error: format!("{:?}", e),
            })?;

        self.theme_styles = sheet;
        Ok(())
    }

    /// Set a CSS custom property (variable).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// styles.set_variable("--primary-color", "#3b82f6");
    /// styles.set_variable("--border-radius", "8px");
    /// ```
    pub fn set_variable(&mut self, name: &str, value: &str) {
        self.variables.insert(name.to_string(), value.to_string());
    }

    /// Get a CSS custom property value.
    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }

    /// Set multiple CSS variables at once.
    pub fn set_variables(&mut self, vars: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>) {
        for (name, value) in vars {
            self.variables.insert(name.into(), value.into());
        }
    }

    /// Get the combined stylesheet with proper cascade order.
    ///
    /// Order (lowest to highest priority):
    /// 1. Default framework styles
    /// 2. Theme styles
    /// 3. Modules (in insertion order)
    /// 4. Custom styles (in load order)
    pub fn combined_stylesheet(&self) -> StyleSheet {
        let mut combined = StyleSheet::default();

        // Add default styles
        combined.merge(self.default_styles.clone());

        // Add theme styles
        combined.merge(self.theme_styles.clone());

        // Add modules
        for sheet in self.modules.values() {
            combined.merge(sheet.clone());
        }

        // Add custom styles (highest priority)
        for sheet in &self.custom_styles {
            combined.merge(sheet.clone());
        }

        combined
    }

    /// Get all rules matching a selector pattern.
    pub fn get_rules(&self, pattern: &str) -> Vec<StyleRule> {
        let combined = self.combined_stylesheet();
        combined.rules
            .into_iter()
            .filter(|rule| {
                // Simple pattern matching - could be enhanced
                rule.selector.parts.iter().any(|part| {
                    match part {
                        crate::css::SelectorPart::Class(name) => name.contains(pattern),
                        crate::css::SelectorPart::Type(name) => name.contains(pattern),
                        crate::css::SelectorPart::Id(name) => name.contains(pattern),
                        _ => false,
                    }
                })
            })
            .collect()
    }

    /// Clear all custom styles (keeps default and theme styles).
    pub fn clear_custom(&mut self) {
        self.custom_styles.clear();
        self.loaded_files.clear();
    }

    /// Clear all styles including defaults.
    pub fn clear_all(&mut self) {
        self.default_styles = StyleSheet::default();
        self.theme_styles = StyleSheet::default();
        self.custom_styles.clear();
        self.modules.clear();
        self.variables.clear();
        self.loaded_files.clear();
    }

    /// Reload all loaded files (useful for hot reload).
    pub fn reload_files(&mut self) -> Result<(), CssLoadError> {
        let files = self.loaded_files.clone();
        self.custom_styles.clear();
        self.loaded_files.clear();

        for file in files {
            self.load_file(&file)?;
        }

        Ok(())
    }

    /// Enable file watching for hot reload.
    pub fn enable_watch(&mut self) {
        self.watch_files = true;
    }

    /// Disable file watching.
    pub fn disable_watch(&mut self) {
        self.watch_files = false;
    }

    /// Get the number of loaded stylesheets.
    pub fn stylesheet_count(&self) -> usize {
        1 + // default
        (if self.theme_styles.rules.is_empty() { 0 } else { 1 }) +
        self.modules.len() +
        self.custom_styles.len()
    }

    /// Get the total number of CSS rules.
    pub fn rule_count(&self) -> usize {
        self.default_styles.rules.len() +
        self.theme_styles.rules.len() +
        self.modules.values().map(|s| s.rules.len()).sum::<usize>() +
        self.custom_styles.iter().map(|s| s.rules.len()).sum::<usize>()
    }
}

/// Errors that can occur when loading CSS.
#[derive(Debug, Clone)]
pub enum CssLoadError {
    /// Failed to read a CSS file
    FileRead {
        path: String,
        error: String,
    },
    /// Failed to parse CSS
    Parse {
        source: String,
        error: String,
    },
    /// Invalid CSS value
    InvalidValue {
        property: String,
        value: String,
    },
}

impl std::fmt::Display for CssLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssLoadError::FileRead { path, error } => {
                write!(f, "Failed to read CSS file '{}': {}", path, error)
            }
            CssLoadError::Parse { source, error } => {
                write!(f, "Failed to parse CSS from {}: {}", source, error)
            }
            CssLoadError::InvalidValue { property, value } => {
                write!(f, "Invalid value '{}' for property '{}'", value, property)
            }
        }
    }
}

impl std::error::Error for CssLoadError {}

/// Builder for creating style configurations.
pub struct StyleBuilder {
    manager: StyleManager,
}

impl StyleBuilder {
    pub fn new() -> Self {
        Self {
            manager: StyleManager::new(),
        }
    }

    /// Start with no default styles.
    pub fn empty() -> Self {
        Self {
            manager: StyleManager::empty(),
        }
    }

    /// Load CSS from a file.
    pub fn file<P: AsRef<Path>>(mut self, path: P) -> Result<Self, CssLoadError> {
        self.manager.load_file(path)?;
        Ok(self)
    }

    /// Load CSS from a string.
    pub fn css(mut self, css: &str) -> Result<Self, CssLoadError> {
        self.manager.load_css(css)?;
        Ok(self)
    }

    /// Load a named module.
    pub fn module(mut self, name: &str, css: &str) -> Result<Self, CssLoadError> {
        self.manager.load_module(name, css)?;
        Ok(self)
    }

    /// Set a CSS variable.
    pub fn var(mut self, name: &str, value: &str) -> Self {
        self.manager.set_variable(name, value);
        self
    }

    /// Set multiple variables.
    pub fn vars(mut self, vars: impl IntoIterator<Item = (&'static str, &'static str)>) -> Self {
        for (name, value) in vars {
            self.manager.set_variable(name, value);
        }
        self
    }

    /// Build the StyleManager.
    pub fn build(self) -> StyleManager {
        self.manager
    }
}

impl Default for StyleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_css_string() {
        let mut manager = StyleManager::new();
        let result = manager.load_css(r#"
            .test-class {
                color: red;
                padding: 10px;
            }
        "#);
        assert!(result.is_ok());
        assert!(manager.rule_count() > 0);
    }

    #[test]
    fn test_css_variables() {
        let mut manager = StyleManager::new();
        manager.set_variable("--primary", "#3b82f6");
        manager.set_variable("--radius", "8px");

        assert_eq!(manager.get_variable("--primary"), Some(&"#3b82f6".to_string()));
        assert_eq!(manager.get_variable("--radius"), Some(&"8px".to_string()));
        assert_eq!(manager.get_variable("--unknown"), None);
    }

    #[test]
    fn test_modules() {
        let mut manager = StyleManager::new();

        manager.load_module("buttons", ".btn { padding: 8px; }").unwrap();
        assert!(manager.has_module("buttons"));

        manager.unload_module("buttons");
        assert!(!manager.has_module("buttons"));
    }

    #[test]
    fn test_style_builder() {
        let manager = StyleBuilder::new()
            .css(".custom { color: blue; }").unwrap()
            .var("--accent", "#f00")
            .build();

        assert!(manager.rule_count() > 0);
        assert_eq!(manager.get_variable("--accent"), Some(&"#f00".to_string()));
    }

    #[test]
    fn test_clear_custom() {
        let mut manager = StyleManager::new();
        let initial_count = manager.rule_count();

        manager.load_css(".extra { color: green; }").unwrap();
        assert!(manager.rule_count() > initial_count);

        manager.clear_custom();
        assert_eq!(manager.rule_count(), initial_count);
    }
}

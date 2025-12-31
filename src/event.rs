//! Event types for OpenKit.

use crate::geometry::Point;

/// A unique identifier for a widget.
pub type WidgetId = u64;

/// Top-level event types.
#[derive(Debug, Clone)]
pub enum Event {
    /// Window events
    Window(WindowEvent),
    /// Mouse events
    Mouse(MouseEvent),
    /// Keyboard events
    Key(KeyEvent),
    /// Focus events
    Focus(FocusEvent),
}

/// Window-related events.
#[derive(Debug, Clone)]
pub enum WindowEvent {
    /// Window was resized
    Resized { width: u32, height: u32 },
    /// Window was moved
    Moved { x: i32, y: i32 },
    /// Window close requested
    CloseRequested,
    /// Window gained focus
    Focused,
    /// Window lost focus
    Unfocused,
    /// Window scale factor changed (DPI)
    ScaleFactorChanged { scale_factor: f64 },
    /// Theme changed (light/dark)
    ThemeChanged { dark: bool },
}

/// Mouse button types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

/// Mouse-related events.
#[derive(Debug, Clone)]
pub struct MouseEvent {
    /// Type of mouse event
    pub kind: MouseEventKind,
    /// Position relative to the window
    pub position: Point,
    /// Mouse button (for click events)
    pub button: Option<MouseButton>,
    /// Modifier keys held during the event
    pub modifiers: Modifiers,
}

/// Kinds of mouse events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventKind {
    /// Mouse button pressed
    Down,
    /// Mouse button released
    Up,
    /// Mouse moved
    Move,
    /// Mouse entered the widget
    Enter,
    /// Mouse left the widget
    Leave,
    /// Mouse wheel scrolled
    Scroll { delta_x: i32, delta_y: i32 },
}

impl MouseEvent {
    pub fn new(kind: MouseEventKind, position: Point) -> Self {
        Self {
            kind,
            position,
            button: None,
            modifiers: Modifiers::empty(),
        }
    }

    pub fn with_button(mut self, button: MouseButton) -> Self {
        self.button = Some(button);
        self
    }

    pub fn with_modifiers(mut self, modifiers: Modifiers) -> Self {
        self.modifiers = modifiers;
        self
    }
}

/// Keyboard-related events.
#[derive(Debug, Clone)]
pub struct KeyEvent {
    /// Type of key event
    pub kind: KeyEventKind,
    /// The key that was pressed
    pub key: Key,
    /// Physical key code (if available)
    pub physical_key: Option<PhysicalKey>,
    /// Text input (if this was a character input)
    pub text: Option<String>,
    /// Modifier keys held during the event
    pub modifiers: Modifiers,
    /// Is this a repeat event?
    pub is_repeat: bool,
}

/// Kinds of key events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventKind {
    /// Key was pressed
    Down,
    /// Key was released
    Up,
}

/// Logical key representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // Numbers
    Num0, Num1, Num2, Num3, Num4,
    Num5, Num6, Num7, Num8, Num9,

    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    // Navigation
    Up, Down, Left, Right,
    Home, End, PageUp, PageDown,

    // Editing
    Backspace, Delete, Insert,
    Enter, Tab,

    // Modifiers (when pressed alone)
    Shift, Control, Alt, Super,

    // Special
    Escape, Space,
    CapsLock, NumLock, ScrollLock,
    PrintScreen, Pause,

    // Punctuation
    Minus, Equal, BracketLeft, BracketRight,
    Backslash, Semicolon, Quote, Grave,
    Comma, Period, Slash,

    // Numpad
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4,
    Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd, NumpadSubtract, NumpadMultiply, NumpadDivide,
    NumpadEnter, NumpadDecimal,

    // Unknown key
    Unknown,
}

/// Physical key code (scan code).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalKey(pub u32);

/// Modifier key states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub super_key: bool,
}

impl Modifiers {
    pub const fn empty() -> Self {
        Self {
            shift: false,
            control: false,
            alt: false,
            super_key: false,
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.shift && !self.control && !self.alt && !self.super_key
    }

    pub fn command(&self) -> bool {
        // On macOS, Command is Super. On Windows/Linux, it's Control.
        #[cfg(target_os = "macos")]
        { self.super_key }
        #[cfg(not(target_os = "macos"))]
        { self.control }
    }
}

/// Focus-related events.
#[derive(Debug, Clone)]
pub enum FocusEvent {
    /// Widget gained focus
    FocusIn { widget_id: WidgetId },
    /// Widget lost focus
    FocusOut { widget_id: WidgetId },
}

/// Result of event handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResult {
    /// Event was handled, stop propagation
    Handled,
    /// Event was not handled, continue propagation
    Ignored,
}

impl EventResult {
    pub fn is_handled(&self) -> bool {
        matches!(self, EventResult::Handled)
    }
}

/// Event handler callback type.
pub type EventHandler<T> = Box<dyn Fn(&T) -> EventResult + Send + Sync>;

/// Click event data.
#[derive(Debug, Clone)]
pub struct ClickEvent {
    pub position: Point,
    pub button: MouseButton,
    pub modifiers: Modifiers,
    pub click_count: u32,
}

/// Change event data for inputs.
#[derive(Debug, Clone)]
pub struct ChangeEvent<T> {
    pub value: T,
}

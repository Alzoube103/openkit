//! Custom CSS Loading Example
//!
//! Demonstrates how to load custom CSS to override framework styles.
//!
//! Run with: `cargo run --example custom_css`

use openkit::prelude::*;

fn main() {
    // Create a style manager with custom styles
    let mut styles = StyleManager::new();

    // Load CSS from a string to override default styles
    styles
        .load_css(
            r#"
        /* Custom button styles */
        .gradient-btn {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border-radius: 12px;
            padding: 16px 32px;
            font-weight: 600;
        }

        .gradient-btn:hover {
            background: linear-gradient(135deg, #764ba2 0%, #667eea 100%);
        }

        /* Custom card style */
        .glass-card {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255, 255, 255, 0.2);
            border-radius: 16px;
            padding: 24px;
        }

        /* Override primary color */
        :root {
            --primary: #8b5cf6;
            --primary-foreground: #ffffff;
        }

        /* Custom label styles */
        .hero-text {
            font-size: 2.5rem;
            font-weight: 800;
            color: var(--primary);
        }

        .subtitle-text {
            font-size: 1.125rem;
            color: var(--muted-foreground);
        }

        /* Neon button effect */
        .neon-btn {
            background-color: transparent;
            border: 2px solid #00ff88;
            color: #00ff88;
            padding: 12px 24px;
            border-radius: 8px;
        }

        .neon-btn:hover {
            background-color: #00ff88;
            color: #000000;
            box-shadow: 0 0 20px #00ff88;
        }
    "#,
        )
        .expect("Failed to load CSS");

    // Set custom CSS variables
    styles.set_variable("--accent-color", "#f59e0b");
    styles.set_variable("--card-spacing", "24px");

    // Run the app with custom styles
    App::new()
        .title("Custom CSS Demo")
        .size(600.0, 500.0)
        .styles(styles)
        .run(|| {
            col![24;
                // Hero section with custom CSS classes
                Label::new("Custom CSS Demo").class("hero-text"),
                Label::new("Override framework styles with your own CSS").class("subtitle-text"),

                spacer!(),

                // Custom styled buttons using .class()
                row![16;
                    Button::new("Gradient Button")
                        .class("gradient-btn")
                        .on_click(|| println!("Gradient clicked!")),
                    Button::new("Neon Button")
                        .class("neon-btn")
                        .on_click(|| println!("Neon clicked!")),
                ],

                spacer!(),

                // Standard buttons showing overridden primary color
                row![12;
                    button!("Primary (Overridden)", {
                        println!("Primary clicked!");
                    }),
                    button!("Secondary", Secondary, {
                        println!("Secondary clicked!");
                    }),
                ],

                spacer!(),

                // Checkbox and text field
                row![16;
                    checkbox!("Enable feature", |checked| {
                        println!("Feature enabled: {}", checked);
                    }),
                    textfield!("Enter value...", |text| {
                        println!("Text: {}", text);
                    }),
                ],
            ]
        })
        .expect("Failed to run app");
}

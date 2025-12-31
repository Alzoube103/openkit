//! OpenKit main executable - runs the example application.
//!
//! Demonstrates the macro-based DX for building UIs.

use openkit::prelude::*;

fn main() {
    App::new()
        .title("OpenKit Demo")
        .size(800.0, 600.0)
        .theme(Theme::Auto)
        .run(|| {
            // Using the declarative macro syntax
            col![24;
                label!("Welcome to OpenKit!", class: "heading"),
                label!("A cross-platform UI framework with CSS styling."),

                // Button row with different variants
                row![12;
                    button!("Primary", { println!("Primary clicked!"); }),
                    button!("Secondary", Secondary),
                    button!("Outline", Outline),
                    button!("Ghost", Ghost),
                    button!("Destructive", Destructive),
                ],

                // Checkbox example
                row![12;
                    checkbox!("Enable dark mode", |checked| {
                        println!("Dark mode: {}", if checked { "ON" } else { "OFF" });
                    }),
                    checkbox!("Notifications", true, |checked| {
                        println!("Notifications: {}", checked);
                    }),
                ],

                // Text field example
                textfield!("Enter your name...", |value| {
                    println!("Name: {}", value);
                }),
            ]
        })
        .expect("Failed to run application");
}

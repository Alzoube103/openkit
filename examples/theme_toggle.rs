//! Theme toggle example demonstrating dark/light theme switching.
//!
//! Shows all button variants and widget types using the macro DX.

use openkit::prelude::*;

fn main() {
    App::new()
        .title("Theme Toggle Demo")
        .size(600.0, 400.0)
        .theme(Theme::Auto)
        .run(|| {
            col![24;
                // Header
                label!("Theme Toggle Demo", class: "title"),
                label!("The theme automatically matches your system preference."),
                label!("Try changing your OS dark/light mode setting!"),

                // All button variants
                row![16;
                    button!("Primary", { println!("Primary clicked"); }),
                    button!("Secondary", Secondary, { println!("Secondary clicked"); }),
                    button!("Outline", Outline, { println!("Outline clicked"); }),
                    button!("Ghost", Ghost, { println!("Ghost clicked"); }),
                    button!("Destructive", Destructive, { println!("Destructive clicked"); }),
                ],

                // Checkboxes
                row![16;
                    checkbox!("Dark mode enabled", |checked| {
                        println!("Dark mode: {}", if checked { "ON" } else { "OFF" });
                    }),
                    checkbox!("Notifications", true, |checked| {
                        println!("Notifications: {}", if checked { "ON" } else { "OFF" });
                    }),
                ],

                // Text input
                textfield!("Type something here...", |value| {
                    println!("Input: {}", value);
                }),
            ]
        })
        .expect("Failed to run application");
}

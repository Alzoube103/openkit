//! Hello World example for OpenKit.
//!
//! The simplest possible OpenKit application using macros.

use openkit::prelude::*;

fn main() {
    App::new()
        .title("Hello OpenKit")
        .size(400.0, 300.0)
        .theme(Theme::Light)
        .run(|| {
            col![16;
                label!("Hello, OpenKit!"),
                button!("Click me!", {
                    println!("Button clicked!");
                }),
            ]
        })
        .expect("Failed to run application");
}

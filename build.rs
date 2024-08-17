use std::collections::HashMap;

fn main() {
    slint_build::compile(
        "ui/window.slint",
    ).unwrap();
}
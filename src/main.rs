
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = SlintDemoWindow::new()?;
    ui.run()
}

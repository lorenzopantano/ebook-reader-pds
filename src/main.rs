use druid::{AppLauncher, PlatformError, WindowDesc};
mod ui;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui::start_window_ui()).show_titlebar(false);
    let data = 0_u32;
    AppLauncher::with_window(main_window).log_to_console().launch(data)
}
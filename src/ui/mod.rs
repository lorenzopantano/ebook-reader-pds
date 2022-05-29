use druid::widget::{Button, Flex, Label, MainAxisAlignment, Padding};
use druid::{LocalizedString, Widget, WidgetExt, Color};

pub fn start_window_ui() -> impl Widget<u32> {
    // The label text will be computed dynamically based on the current locale and count
    let text = LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment").on_click(|_ctx, data, _env| *data += 1).padding(5.0);
    let counter = Flex::column().with_child(label).with_child(button);

    // App bar
    let title = Label::new("EBook Reader - PDS").with_text_color(Color::rgb(255.0, 255.0, 255.0));
    let close_button = Button::new("Close").on_click(|ctx, _data, _env| ctx.window().close());

    let mut app_bar_row = Flex::row();
    app_bar_row.set_must_fill_main_axis(true);
    app_bar_row.add_child(title);
    app_bar_row.set_main_axis_alignment(MainAxisAlignment::SpaceBetween);
    app_bar_row.add_child(close_button);
    let app_bar = Padding::new(30.0, app_bar_row);

    Flex::column().with_child(app_bar).with_child(counter)
}
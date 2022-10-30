use gtk::gdk::Display;
use gtk::glib::clone;
use gtk::{glib, Entry};
use gtk::{prelude::*, Box, Button, CssProvider, Label, StyleContext};
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "com.stacksparrow4.KBShareRS";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let main_menu_label = Label::builder()
        .label("KB Share")
        .css_classes(vec!["header".to_string()])
        .build();

    let start_server_button = Button::builder().label("Start Server").build();
    let start_client_button = Button::builder().label("Start Client").build();

    let window_box = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    window_box.append(&main_menu_label);
    window_box.append(&start_server_button);
    window_box.append(&start_client_button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("KB Share")
        .child(&window_box)
        .build();

    let server_window = build_server_window(app);

    start_server_button.connect_clicked(clone!(@weak window =>
        move |_| {
        server_window.present();
        window.close();
    }));

    window.present();
}

fn build_server_window(app: &Application) -> ApplicationWindow {
    let port_label = Label::builder().label("Port for server").build();
    let port_entry = Entry::builder().text("1234").build();

    let port_box = Box::builder().build();
    port_box.append(&port_label);
    port_box.append(&port_entry);

    let start_server = Button::builder().label("START").build();

    let window_box = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    window_box.append(&port_box);
    window_box.append(&start_server);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("KB Share Server")
        .child(&window_box)
        .build();

    return window;
}

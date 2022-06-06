mod window;
mod node_button;

use gtk::gdk::Display;
use gtk::{prelude::*, CssProvider, StyleContext, Button, ApplicationWindow};
use gtk::{gio, Application};
use window::Window;

const APP_ID: &str = "org.galaxymenu.constellation";

fn main() {
    // Register and include resources
    gio::resources_register_include!("templates_main.gresource")
        .expect("Failed to register resources.");

    // Create a new application tmp
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/functions/fn.accelerator_parse.html
    // Set keyboard accelerator to trigger "win.close".
    app.set_accels_for_action("win.close", &["<Ctrl>W"]);

    // Run the application
    app.run();
}
/*
fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}
*/

fn spacer() -> gtk::Label {
    gtk::Label::builder()
        .label(" ")
        .build()
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        /*
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        */
        .build();

        let button2 = Button::builder()
        .label("Press me!")
        /*
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        */
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World          !");
    });

    let grid = gtk::Grid::builder()
        .row_homogeneous(true)
        .column_homogeneous(true)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    grid.attach(&button, 0, 0, 1, 1);
    grid.attach(&spacer(), 1, 0, 1, 1);
    grid.attach(&spacer(), 0, 1, 1, 1);
    grid.attach(&button2, 1, 1, 1, 1);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&grid)
        .build();

    // Present window
    window.present();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

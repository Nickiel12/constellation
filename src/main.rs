mod window;

use gtk::prelude::*;
use gtk::{gio, Application};
use window::Window;

const APP_ID: &str = "org.gtk-rs.CompositeTemplatesMain";

fn main() {
    // Register and include resources
    gio::resources_register_include!("templates_main.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/functions/fn.accelerator_parse.html
    // Set keyboard accelerator to trigger "win.close".
    app.set_accels_for_action("win.close", &["<Ctrl>W"]);

    // Run the application
    app.run();
}
fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}
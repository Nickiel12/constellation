use gtk::gio;

fn main() {
    gio::compile_resources(
        "src/resources",
        "src/resources/resources.gresource.xml",
        "templates_main.gresource",
    );
}
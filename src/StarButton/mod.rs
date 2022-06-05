mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct ConstellationNodeButton(ObjectSubclass<imp::ConstellationNodeButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ConstellationNodeButton {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `ConstellationNodeButton`.")
    }
}

impl Default for ConstellationNodeButton {
    fn default() -> Self {
        Self::new()
    }
}
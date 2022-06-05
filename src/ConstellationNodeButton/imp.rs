use std::cell::Cell;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct ConstellationNodeButton {
    number: Cell<i32>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ConstellationNodeButton {
    const NAME: &'static str = "ConstellationNodeButton";
    type Type = super::ConstellationNodeButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for ConstellationNodeButton {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.set_label(&self.number.get().to_string());
    }
}

// Trait shared by all widgets
impl WidgetImpl for ConstellationNodeButton {}

// Trait shared by all buttons
impl ButtonImpl for ConstellationNodeButton {
    fn clicked(&self, button: &Self::Type) {
        self.number.set(self.number.get() + 1);
        button.set_label(&self.number.get().to_string())
    }
}
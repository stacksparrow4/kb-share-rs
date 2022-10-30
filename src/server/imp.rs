use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/stacksparrow4/KBShareRS/server.ui")]
pub struct Server {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Server {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "StartServer";
    type Type = super::Server;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Server {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Server {}

// Trait shared by all windows
impl WindowImpl for Server {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Server {}

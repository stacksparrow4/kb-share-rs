use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Entry};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/stacksparrow4/KBShareRS/client.ui")]
pub struct Client {
    #[template_child]
    pub ip_entry: TemplateChild<Entry>,
    #[template_child]
    pub port_entry: TemplateChild<Entry>,
    #[template_child]
    pub connect_button: TemplateChild<Button>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Client {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "StartClient";
    type Type = super::Client;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Client {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Client {}

// Trait shared by all windows
impl WindowImpl for Client {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Client {}

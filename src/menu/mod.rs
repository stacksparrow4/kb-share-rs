mod imp;

use glib::Object;
use gtk::{gio, glib, subclass::prelude::ObjectSubclassIsExt, Application};

glib::wrapper! {
    pub struct Menu(ObjectSubclass<imp::Menu>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Menu {
    pub fn new(app: &Application) -> Self {
        let window: Self = Object::builder().property("application", app).build();

        window.imp().create_windows(app);

        window
    }
}

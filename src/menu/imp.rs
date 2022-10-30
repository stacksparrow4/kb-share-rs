use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Application, Button, CompositeTemplate};

use crate::server::Server;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/stacksparrow4/KBShareRS/menu.ui")]
pub struct Menu {
    #[template_child]
    pub start_server: TemplateChild<Button>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Menu {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MainMenu";
    type Type = super::Menu;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Menu {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl Menu {
    pub fn create_windows(&self, app: &Application) {
        let obj2 = self.obj().clone();
        let app2 = app.clone();
        self.start_server.connect_clicked(move |_| {
            Server::new(&app2).present();
            obj2.close();
        });
    }
}

// Trait shared by all widgets
impl WidgetImpl for Menu {}

// Trait shared by all windows
impl WindowImpl for Menu {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Menu {}

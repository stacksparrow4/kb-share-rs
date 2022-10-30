use glib::subclass::InitializingObject;
use gtk::glib::clone;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Entry};
use gtk::{prelude::*, TextView};

use crate::keycodenames::KEYCODE_NAMES;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/stacksparrow4/KBShareRS/server.ui")]
pub struct Server {
    #[template_child]
    pub client_binding_entry: TemplateChild<Entry>,
    #[template_child]
    pub server_binding_entry: TemplateChild<Entry>,
    #[template_child]
    pub binding_textview: TemplateChild<TextView>,
    #[template_child]
    pub add_binding_button: TemplateChild<Button>,
}

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

fn create_key_completion() -> gtk::EntryCompletion {
    let autocomplete = gtk::ListStore::new(&[String::static_type()]);

    for entry in KEYCODE_NAMES.keys() {
        autocomplete.set(&autocomplete.append(), &[(0, entry)]);
    }

    let key_completion = gtk::EntryCompletion::builder()
        .popup_completion(true)
        .model(&autocomplete)
        .build();
    key_completion.set_text_column(0);

    key_completion
}

// Trait shared by all GObjects
impl ObjectImpl for Server {
    fn constructed(&self) {
        self.parent_constructed();

        let binding_textview = self.binding_textview.clone();
        let client_binding_entry = self.client_binding_entry.clone();
        let server_binding_entry = self.server_binding_entry.clone();

        client_binding_entry.set_completion(Some(&create_key_completion()));
        server_binding_entry.set_completion(Some(&create_key_completion()));

        self.add_binding_button.connect_clicked(clone!(@weak binding_textview, @weak client_binding_entry, @weak server_binding_entry => move |_| {
            let buf = binding_textview.buffer();
            let mut curr_text = String::from(buf.text(&buf.start_iter(), &buf.end_iter(), true));

            if curr_text.len() > 0 {
                curr_text.push_str("\n");
            }
            curr_text.push_str(client_binding_entry.text().as_str());
            curr_text.push_str(" => ");
            curr_text.push_str(server_binding_entry.text().as_str());
            buf.set_text(curr_text.as_str());

            client_binding_entry.set_text("");
            server_binding_entry.set_text("");
        }));
    }
}

// Trait shared by all widgets
impl WidgetImpl for Server {}

// Trait shared by all windows
impl WindowImpl for Server {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Server {}

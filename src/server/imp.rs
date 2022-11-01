use std::collections::HashMap;

use glib::subclass::InitializingObject;
use gtk::glib::clone;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Entry};
use gtk::{prelude::*, TextView};

use crate::keycodenames::KEYCODE_NAMES;
use crate::net_server::start_server_thread;

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
    #[template_child]
    pub port_entry: TemplateChild<Entry>,
    #[template_child]
    pub start_server_button: TemplateChild<Button>,
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
        let add_binding_button = self.add_binding_button.clone();
        let port_entry = self.port_entry.clone();

        client_binding_entry.set_completion(Some(&create_key_completion()));
        server_binding_entry.set_completion(Some(&create_key_completion()));

        add_binding_button.connect_clicked(clone!(@weak binding_textview, @weak client_binding_entry, @weak server_binding_entry => move |_| {
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

        self.start_server_button
            .connect_clicked(clone!(@weak binding_textview, @weak client_binding_entry, @weak server_binding_entry, @weak add_binding_button, @weak port_entry => move |btn| {
                let buf = binding_textview.buffer();
                let curr_text = String::from(buf.text(&buf.start_iter(), &buf.end_iter(), true));

                let mut mappings: HashMap<&str, &str> = HashMap::new();

                for l in curr_text.lines() {
                    if l.trim().len() == 0 {
                        continue;
                    }

                    let parts: Vec<&str> = l.split("=>").collect();

                    if parts.len() != 2 {
                        btn.set_label("Invalid bindings syntax");
                        return;
                    }

                    let binding_client = parts[0].trim();
                    let binding_server = parts[1].trim();

                    if !KEYCODE_NAMES.contains_key(binding_client) {
                        btn.set_label(format!("Invalid binding {}", binding_client).as_str());
                        return;
                    }
                    if !KEYCODE_NAMES.contains_key(binding_server) {
                        btn.set_label(format!("Invalid binding {}", binding_server).as_str());
                        return;
                    }

                    mappings.insert(KEYCODE_NAMES.get_key(binding_client).unwrap(), KEYCODE_NAMES.get_key(binding_server).unwrap());
                }

                let port = u16::from_str_radix(&port_entry.text(), 10);

                if let Err(_) = port {
                    btn.set_label("Invalid port");
                    return;
                }

                let recv_err = start_server_thread(mappings, port.unwrap());

                btn.set_label("Server started!");
                btn.set_sensitive(false);
                add_binding_button.set_sensitive(false);
                server_binding_entry.set_sensitive(false);
                client_binding_entry.set_sensitive(false);
                binding_textview.set_sensitive(false);
                port_entry.set_sensitive(false);

                recv_err.attach(None, clone!(@weak btn => @default-return Continue(false),
                    move |msg| {
                        btn.add_css_class("error");
                        btn.set_label(msg.as_str());
                        Continue(true)
                }));
            }));
    }
}

// Trait shared by all widgets
impl WidgetImpl for Server {}

// Trait shared by all windows
impl WindowImpl for Server {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Server {}

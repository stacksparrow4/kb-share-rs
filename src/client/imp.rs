use glib::subclass::InitializingObject;
use gtk::glib::clone;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Entry};
use gtk::{prelude::*, Label};

use crate::net_client::start_client_thread;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/stacksparrow4/KBShareRS/client.ui")]
pub struct Client {
    #[template_child]
    pub connection_status: TemplateChild<Label>,
    #[template_child]
    pub ip_entry: TemplateChild<Entry>,
    #[template_child]
    pub port_entry: TemplateChild<Entry>,
    #[template_child]
    pub src_port_entry: TemplateChild<Entry>,
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

        let connection_status = self.connection_status.clone();
        let ip_entry = self.ip_entry.clone();
        let port_entry = self.port_entry.clone();
        let src_port_entry = self.src_port_entry.clone();

        self.connect_button.connect_clicked(
            clone!(@weak connection_status, @weak ip_entry, @weak port_entry, @weak src_port_entry => move |btn| {
                let dst_port = u16::from_str_radix(&port_entry.text(), 10);
                if let Err(_) = dst_port {
                    btn.set_label("Invalid destination port");
                    return;
                }

                let src_port = u16::from_str_radix(&src_port_entry.text(), 10);
                if let Err(_) = src_port {
                    btn.set_label("Invalid source port");
                    return;
                }

                let (recv_err, recv_msg) = start_client_thread(&ip_entry.text(), dst_port.unwrap(), src_port.unwrap());

                recv_err.attach(
                    None,
                    clone!(@weak btn => @default-return Continue(false),
                        move |msg| {
                            btn.add_css_class("error");
                            btn.set_label(msg.as_str());
                            Continue(true)
                    }),
                );

                recv_msg.attach(
                    None,
                    clone!(@weak btn => @default-return Continue(false),
                        move |msg| {
                            connection_status.set_text(&msg);
                            Continue(true)
                    }),
                );

                btn.set_label("Connected");
                btn.set_sensitive(false);
            }),
        );
    }
}

// Trait shared by all widgets
impl WidgetImpl for Client {}

// Trait shared by all windows
impl WindowImpl for Client {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Client {}

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

use glib::subclass::InitializingObject;
use gtk::glib::clone;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Entry, EventControllerKey};
use gtk::{prelude::*, Label};

use crate::net_client::{start_client_thread, KeyPressMsg};

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

    pub msg_sender: Rc<RefCell<Option<mpsc::Sender<KeyPressMsg>>>>,
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

        let key_sender = self.msg_sender.clone();

        self.connect_button.connect_clicked(
            clone!(@weak connection_status, @weak ip_entry, @weak port_entry, @weak src_port_entry, @weak key_sender => move |btn| {
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

                let (recv_err, recv_msg, send_key) = start_client_thread(&ip_entry.text(), dst_port.unwrap(), src_port.unwrap());

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

                // Attach key sender to window state, to be utilised by event
                *key_sender.borrow_mut() = Some(send_key);

                btn.set_label("Connected");
                btn.set_sensitive(false);
                ip_entry.set_sensitive(false);
                port_entry.set_sensitive(false);
                src_port_entry.set_sensitive(false);
            }),
        );

        let e_control = EventControllerKey::new();

        e_control.connect_key_pressed(
            clone!(@weak key_sender => @default-return gtk::Inhibit(false),
            move |_, _, k, _| {
                let sender = key_sender.borrow();

                if sender.is_none() {
                    println!("Sender doesn't exist yet!");
                    return gtk::Inhibit(false);
                }

                if sender.as_ref()
                .unwrap()
                .send(KeyPressMsg {
                    keycode: k as u16,
                    is_pressed: true,
                })
                .is_err() {
                    println!("Reciever is closed");
                }

                gtk::Inhibit(false)
            }),
        );

        e_control.connect_key_released(clone!(@weak key_sender =>
        move |_, _, k, _| {
            let sender = key_sender.borrow();

            if sender.is_none() {
                println!("Sender doesn't exist yet!");
                return;
            }

            if sender.as_ref()
            .unwrap()
            .send(KeyPressMsg {
                keycode: k as u16,
                is_pressed: false,
            })
            .is_err() {
                println!("Reciever is closed");
            }
        }));

        self.obj().add_controller(&e_control);
    }
}

// Trait shared by all widgets
impl WidgetImpl for Client {}

// Trait shared by all windows
impl WindowImpl for Client {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Client {}

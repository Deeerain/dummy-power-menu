use std::rc::Rc;

use gtk::gdk::keys::constants::Escape;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Button, Label, Window};
use gtk_layer_shell::LayerShell;

pub struct DialogBuilder {
    parent: ApplicationWindow,
    message: String,
    yes_button: Option<Button>,
    no_button: Option<Button>,
    on_confrim: Rc<dyn Fn()>,
}

impl DialogBuilder {
    pub fn new<F: Fn() + 'static>(
        parent: &ApplicationWindow,
        message: &str,
        on_confrim: F,
    ) -> Self {
        Self {
            parent: parent.clone(),
            message: message.to_string(),
            yes_button: Option::None,
            no_button: Option::None,
            on_confrim: Rc::new(on_confrim),
        }
    }

    pub fn build(mut self) -> Window {
        self.parent.hide();

        let dialog = self.create_dialog_window();
        self.setup_layer_shell(&dialog);
        self.setup_ui(&dialog);
        self.setup_events(&dialog);

        dialog
    }

    fn create_dialog_window(&self) -> Window {
        Window::builder()
            .destroy_with_parent(true)
            .decorated(false)
            .modal(true)
            .type_hint(gtk::gdk::WindowTypeHint::Dialog)
            .transient_for(&self.parent)
            .build()
    }

    fn setup_layer_shell(&self, dialog: &Window) {
        dialog.init_layer_shell();
        dialog.set_keyboard_interactivity(true);
        dialog.set_layer(gtk_layer_shell::Layer::Top);
        dialog.auto_exclusive_zone_enable();
    }

    fn setup_ui(&mut self, dialog: &Window) {
        let vbox = gtk::Box::builder()
            .spacing(15)
            .margin(25)
            .orientation(gtk::Orientation::Vertical)
            .build();

        let hbox = gtk::Box::builder()
            .spacing(15)
            .halign(gtk::Align::Center)
            .build();

        let label = Label::builder()
            .label(&self.message)
            .wrap(true)
            .halign(gtk::Align::Center)
            .build();

        self.no_button = Some(
            Button::builder()
                .label("Нет")
                .width_request(90)
                .height_request(35)
                .build(),
        );

        self.yes_button = Some(
            Button::builder()
                .label("Да")
                .width_request(90)
                .height_request(35)
                .build(),
        );

        hbox.pack_start(&self.yes_button.clone().unwrap(), false, false, 0);
        hbox.pack_start(&self.no_button.clone().unwrap(), false, false, 0);

        vbox.pack_start(&label, true, true, 0);
        vbox.pack_start(&hbox, false, false, 0);
        dialog.add(&vbox);
    }

    fn setup_events(&self, dialog: &Window) {
        let parent = self.parent.clone();
        let on_confirm = self.on_confrim.clone();

        dialog.connect_key_press_event(clone!(@strong dialog, @strong parent => move |_, event| {
            if event.keyval() == Escape {
                dialog.close();
                return gtk::glib::Propagation::Stop;
            }
            gtk::glib::Propagation::Proceed
        }));

        dialog.connect_destroy(clone!(@strong parent, @strong on_confirm => move |_| {
            parent.show_all();
        }));

        if let Some(yes_button) = self.yes_button.clone() {
            yes_button.connect_clicked(clone!(@strong on_confirm, @strong dialog => move |_| {
                dialog.close();
                println!("yes");
                (on_confirm)();
            }));
        }

        if let Some(no_button) = self.no_button.clone() {
            no_button.connect_clicked(clone!(@strong dialog => move |_| {
                println!("No");
                dialog.close();
            }));
        }
    }
}

use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Box, Button};

use crate::action::{Action, execute_system_action};

use super::DialogBuilder;

pub struct ActionBarBuilder {
    window: ApplicationWindow,
    actions: Vec<Action>,
}

impl ActionBarBuilder {
    pub fn new(window: &ApplicationWindow, actions: Vec<Action>) -> Self {
        Self {
            window: window.clone(),
            actions,
        }
    }

    pub fn build(self) -> Box {
        let hbox = Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .halign(gtk::Align::Center)
            .build();

        for action in self.actions.clone() {
            let button = self.create_action_button(&action);
            hbox.pack_start(&button, false, false, 0);
        }

        hbox
    }

    fn create_action_button(&self, action: &Action) -> Button {
        let button = Button::builder()
            .width_request(70)
            .height_request(70)
            .label(action.icon)
            .tooltip_text(action.description)
            .build();

        let window = self.window.clone();
        let action = action.clone();

        button.connect_clicked(clone!(@strong window, @strong action => move |_| {
            if action.requires_confirmation {
                let dialog_message = format!("{}?", action.description);
                let dialog = DialogBuilder::new(&window, &dialog_message, clone!(@strong window, @strong action => move || {
                    if let Err(e) = execute_system_action(&action.command) {
                        eprintln!("{}", e);
                    }
                    window.close();
                })).build();

                dialog.show_all();
            } else {
                if let Err(e) = execute_system_action(&action.command) {
                    eprintln!("{}", e);
                }
                window.close();
            }
        }));

        button
    }
}

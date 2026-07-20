use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box, Button};

use crate::action::{Action};

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
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(5)
            .halign(gtk4::Align::Center)
            .build();

        for action in self.actions.clone() {
            let button = self.create_action_button(&action);
            hbox.append(&button);
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

        button
    }
}

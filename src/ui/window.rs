use gtk::gdk::keys::constants::Escape;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box};
use gtk_layer_shell::LayerShell;

use super::ActionBarBuilder;
use crate::action::Action;

pub struct MainWindowBuilder {
    app: Application,
}

impl MainWindowBuilder {
    pub fn new(app: &Application) -> Self {
        Self { app: app.clone() }
    }

    pub fn build(self) -> ApplicationWindow {
        let window = ApplicationWindow::builder()
            .application(&self.app)
            .name("main-window")
            .build();

        self.setup_layer_shell(&window);
        self.setup_keyboard_events(&window);
        self.setup_ui(&window);

        window.show_all();
        window
    }

    fn setup_layer_shell(&self, window: &ApplicationWindow) {
        window.init_layer_shell();
        window.set_layer(gtk_layer_shell::Layer::Top);
        window.set_keyboard_interactivity(true);
        window.auto_exclusive_zone_enable();
    }

    fn setup_keyboard_events(&self, window: &ApplicationWindow) {
        window.connect_key_press_event(|window, event| {
            if event.keyval() == Escape {
                window.close();
                return gtk::glib::Propagation::Stop;
            }

            gtk::glib::Propagation::Proceed
        });
    }

    fn setup_ui(&self, window: &ApplicationWindow) {
        let vbox = Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin(10)
            .build();
        let actions = Action::get_all();
        let bar = ActionBarBuilder::new(window, actions).build();

        vbox.pack_start(&bar, true, true, 0);
        window.add(&vbox);
        window.show_all();
    }
}

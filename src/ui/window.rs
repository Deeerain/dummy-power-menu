use gtk4::{prelude::*};
use gtk4::{Application, ApplicationWindow, Box};

use gtk4_layer_shell::LayerShell;
use gtk4_layer_shell::{Layer, KeyboardMode};

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
        self.setup_ui(&window);

        window.present();
        window
    }

    fn setup_layer_shell(&self, window: &ApplicationWindow) {
        window.init_layer_shell();
        window.set_layer(Layer::Top);
        window.set_keyboard_mode(KeyboardMode::Exclusive);
        window.auto_exclusive_zone_enable();
    }

    fn setup_ui(&self, window: &ApplicationWindow) {
        let vbox = Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(10)
            .build();
        let actions = Action::get_all();
        let bar = ActionBarBuilder::new(window, actions).build();

        vbox.append(&bar);
        window.set_child(Some(&vbox));
    }
}

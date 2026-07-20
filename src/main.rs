use gtk4::Application;
use gtk4::glib::ExitCode;
use gtk4::prelude::*;

use crate::ui::MainWindowBuilder;

mod action;
mod ui;

const APP_ID: &str = "com.deerains.dummy-power-menu";


fn main() -> ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        MainWindowBuilder::new(app).build();
    });
    app.run()
}

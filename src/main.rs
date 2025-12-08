use gtk::gdk::keys::constants::Escape;
use gtk::glib::Propagation;
use gtk::glib::clone;
use gtk::{Application, Label, Window, glib};
use gtk::{ApplicationWindow, Box, Button, prelude::*};
use gtk_layer_shell::LayerShell;
use std::process::Command;

const APP_ID: &str = "com.deerains.dummy-power-menu";

#[derive(Clone)]
struct Action {
    icon: &'static str,
    command: &'static str,
    description: &'static str,
    requires_confirmation: bool,
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(on_app_activated);
    app.run()
}

fn on_app_activated(app: &Application) {
    build_ui(app);
}

fn build_ui(app: &Application) {
    let actions = vec![
        Action {
            icon: "󰐥",
            command: "shutdown",
            description: "Выключение системы",
            requires_confirmation: true,
        },
        Action {
            icon: "󰑓",
            command: "soft-reboot",
            description: "Перезагрузка",
            requires_confirmation: true,
        },
        Action {
            icon: "󰒲",
            command: "suspend",
            description: "Сон",
            requires_confirmation: false,
        },
        Action {
            icon: "󰢠",
            command: "hibernate",
            description: "Гибернация",
            requires_confirmation: false,
        },
        Action {
            icon: "󰍃",
            command: "logout",
            description: "Выход из системы",
            requires_confirmation: true,
        },
        Action {
            icon: "",
            command: "lock",
            description: "Блокировка экрана",
            requires_confirmation: false,
        },
    ];

    let window = create_window(app);
    let vbox = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin(5)
        .build();

    let bar = create_bar(&window, actions);
    vbox.pack_start(&bar, true, true, 0);
    window.add(&vbox);
    window.show_all();
}

fn create_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .name("main-window")
        .build();

    window.init_layer_shell();
    window.set_layer(gtk_layer_shell::Layer::Top);
    window.set_keyboard_interactivity(true);

    window.connect_key_press_event(|window, event| {
        if event.keyval() == Escape {
            window.close();
            Propagation::Stop
        } else {
            Propagation::Proceed
        }
    });

    window
}

fn create_dialog_window<F: Fn() + 'static>(
    parent: &ApplicationWindow,
    message: &str,
    on_confirm: F,
) -> Window {
    parent.hide();

    let dialog = Window::builder()
        .destroy_with_parent(true)
        .decorated(false)
        .width_request(300)
        .height_request(200)
        .modal(true)
        .type_hint(gtk::gdk::WindowTypeHint::Dialog)
        .transient_for(parent)
        .build();

    let vbox = Box::builder()
        .spacing(10)
        .margin(20)
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    let hbox = Box::builder()
        .spacing(10)
        .halign(gtk::Align::Center)
        .build();

    let label = Label::builder()
        .label(message)
        .wrap(true)
        .halign(gtk::Align::Center)
        .build();

    let yes_button = Button::builder().label("Да").width_request(80).build();

    let no_button = Button::builder().label("Нет").width_request(80).build();

    dialog.connect_key_press_event(clone!(@strong dialog => move |_, event| {
        if event.keyval() == Escape {
            dialog.close();
            Propagation::Stop
        } else {
            Propagation::Proceed
        }
    }));

    dialog.connect_delete_event(clone!(@strong dialog, @strong parent => move |_,_|{
        parent.show_all();

        Propagation::Proceed
    }));

    yes_button.connect_clicked(clone!(@strong dialog => move |_| {
        dialog.close();
        on_confirm();
    }));

    no_button.connect_clicked(clone!(@strong dialog => move |_| {
        dialog.close();
    }));

    hbox.pack_start(&yes_button, false, false, 0);
    hbox.pack_start(&no_button, false, false, 0);

    vbox.pack_start(&label, true, true, 0);
    vbox.pack_start(&hbox, false, false, 0);
    dialog.add(&vbox);

    dialog.init_layer_shell();
    dialog.set_keyboard_interactivity(true);
    dialog.set_layer(gtk_layer_shell::Layer::Top);

    dialog
}

fn create_bar(window: &ApplicationWindow, actions: Vec<Action>) -> Box {
    let hbox = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .halign(gtk::Align::Center)
        .build();

    for action in actions {
        let button = Button::builder()
            .width_request(60)
            .height_request(60)
            .label(action.icon)
            .tooltip_text(action.description)
            .build();

        let window_clone = window.clone();
        let action_clone = action.clone();

        button.connect_clicked(move |_| {
            if action_clone.requires_confirmation {
                let dialog_message = format!("{}?", action_clone.description);
                let dialog = create_dialog_window(&window_clone, &dialog_message, move || {
                    execute_system_action(&action_clone.command);
                });
                dialog.show_all();
            } else {
                execute_system_action(&action_clone.command);
            }

            window_clone.close();
        });

        hbox.pack_start(&button, false, false, 0);
    }

    hbox
}

fn execute_system_action(action: &str) {
    let result = match action {
        "shutdown" => Command::new("systemctl").arg("poweroff").spawn(),
        "soft-reboot" => Command::new("systemctl").arg("reboot").spawn(),
        "suspend" => Command::new("systemctl").arg("suspend").spawn(),
        "hibernate" => Command::new("systemctl").arg("hibernate").spawn(),
        "logout" => Command::new("hyprctl").args(["dispatch", "exit"]).spawn(),
        "lock" => Command::new("hyprctl")
            .args(["dispatch", "exec", "hyprlock"])
            .spawn(),
        _ => {
            eprintln!("Unknown action: {}", action);
            return;
        }
    };

    match result {
        Ok(_) => println!("Action '{}' executed", action),
        Err(e) => eprintln!("Failed to execute '{}': {}", action, e),
    }
}

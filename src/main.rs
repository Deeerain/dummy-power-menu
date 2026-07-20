use std::process::Command;

use async_channel::Sender;
use gtk4::gdk::Key;
use gtk4::glib::{ExitCode, Propagation};
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, EventControllerKey, Orientation, glib,
};

const APP_ID: &str = "com.deerains.dummy-power-menu";

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum PowerCommand {
    Shutdown,
    Reboot,
    Suspend,
    Exit,
    CloseMenu,
}

#[derive(Clone, Copy)]
struct CommandSpec {
    program: &'static str,
    args: &'static [&'static str],
}

#[derive(Clone, Copy)]
struct PowerAction {
    command: PowerCommand,
    label: &'static str,
    spec: Option<CommandSpec>,
}

impl CommandSpec {
    const fn new(program: &'static str, args: &'static [&'static str]) -> Self {
        Self { program, args }
    }
}

fn main() -> ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let (sender, receiver) = async_channel::unbounded::<PowerCommand>();
    let actions = action_definitions();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Power Menu")
        .build();

    setup_layer_shell(&window);
    setup_controller(&window, &sender);

    let box_container = Box::new(Orientation::Vertical, 12);
    box_container.set_margin_top(5);
    box_container.set_margin_bottom(5);
    box_container.set_margin_start(5);
    box_container.set_margin_end(5);
    box_container.set_halign(gtk4::Align::Center);
    box_container.set_valign(gtk4::Align::Center);

    for btn in build_buttons(&sender, &actions) {
        box_container.append(&btn);
    }

    window.set_child(Some(&box_container));
    window.set_resizable(false);
    window.present();

    let window_clone = window.clone();
    let actions_for_loop = actions;
    glib::MainContext::default().spawn_local(async move {
        while let Ok(command) = receiver.recv().await {
            if let Some(action) = actions_for_loop.iter().find(|entry| entry.command == command) {
                if let Some(spec) = action.spec {
                    match Command::new(spec.program).args(spec.args).spawn() {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("Failed to run {:?}: {err}", action.command);
                        }
                    }
                }
            }

            window_clone.close();
            break;
        }
    });
}

fn action_definitions() -> Vec<PowerAction> {
    vec![
        PowerAction {
            command: PowerCommand::Shutdown,
            label: "Shutdown",
            spec: Some(CommandSpec::new("systemctl", &["poweroff"])),
        },
        PowerAction {
            command: PowerCommand::Reboot,
            label: "Reboot",
            spec: Some(CommandSpec::new("systemctl", &["reboot"])),
        },
        PowerAction {
            command: PowerCommand::Suspend,
            label: "Suspend",
            spec: Some(CommandSpec::new("systemctl", &["suspend"])),
        },
        PowerAction {
            command: PowerCommand::Exit,
            label: "Exit session",
            spec: Some(CommandSpec::new("hyprctl", &["dispatch", "exit"])),
        },
        PowerAction {
            command: PowerCommand::CloseMenu,
            label: "Close",
            spec: None,
        },
    ]
}

fn setup_layer_shell(window: &ApplicationWindow) {
    use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);
    window.set_anchor(Edge::Left, false);
    window.set_anchor(Edge::Right, false);
    window.set_anchor(Edge::Top, false);
    window.set_anchor(Edge::Bottom, false);
}

fn setup_controller(window: &ApplicationWindow, sender: &Sender<PowerCommand>) {
    let key_controller = EventControllerKey::new();
    let tx = sender.clone();
    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        let command = match keyval {
            Key::Escape => Some(PowerCommand::CloseMenu),
            _ => None,
        };

        if let Some(cmd) = command {
            let _ = tx.send_blocking(cmd);
            Propagation::Proceed
        } else {
            Propagation::Stop
        }
    });
    window.add_controller(key_controller);
}

fn build_buttons(sender: &Sender<PowerCommand>, actions: &[PowerAction]) -> Vec<Button> {
    let mut result = Vec::<Button>::new();

    for action in actions {
        let tx = sender.clone();
        let button = Button::builder().label(action.label).build();
        let command = action.command;
        button.connect_clicked(move |_| {
            let _ = tx.send_blocking(command);
        });
        result.push(button);
    }

    result
}
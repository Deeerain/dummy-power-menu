use std::process::Command;

#[derive(Clone)]
pub struct Action {
    pub icon: &'static str,
    pub command: &'static str,
    pub description: &'static str,
    pub requires_confirmation: bool,
}

impl Action {
    pub fn get_all() -> Vec<Self> {
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
        actions
    }
}

pub fn execute_system_action(action: &str) {
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

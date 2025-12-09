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

#[cfg(not(debug_assertions))]
pub fn execute_system_action(action: &str) -> Result<std::process::Child, std::io::Error> {
    let result = match action {
        "shutdown" => Command::new("systemctl").arg("poweroff").spawn(),
        "soft-reboot" => Command::new("systemctl").arg("reboot").spawn(),
        "suspend" => Command::new("systemctl").arg("suspend").spawn(),
        "hibernate" => Command::new("systemctl").arg("hibernate").spawn(),
        "logout" => Command::new("hyprctl").args(["dispatch", "exit"]).spawn(),
        "lock" => Command::new("hyprctl")
            .args(["dispatch", "exec", "hyprlock"])
            .spawn(),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Command not found",
        )),
    };

    result
}

#[cfg(debug_assertions)]
pub fn execute_system_action(action: &str) -> Result<std::process::Child, std::io::Error> {
    let result = Command::new("echo").arg(action).spawn();
    result
}

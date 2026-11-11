//! Windows registry read helpers for Procreate Pocket Companion.

pub const APP_KEY: &str = "Software\\Procreate\\Procreate Pocket Companion";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Procreate Pocket Companion";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Procreate Pocket Companion") },
        RegValue { name: "Version",     value: String::from("v4.2.9") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Procreate Pocket Companion") },
        RegValue { name: "Publisher",       value: String::from("Procreate") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Procreate Pocket Companion\\uninstall.exe\"") },
    ]
}

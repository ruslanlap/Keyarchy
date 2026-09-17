use std::fmt;

use iced::keyboard::{key::Named, Key, Modifiers};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hotkey {
    pub super_key: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub key: String,
}

impl Hotkey {
    pub fn parse(modifiers: &str, key: &str) -> Self {
        let modifiers = modifiers.to_ascii_uppercase();
        Self {
            super_key: modifiers.contains("SUPER")
                || modifiers.contains("META")
                || modifiers.contains("MAINMOD")
                || modifiers.contains("MOD4"),
            ctrl: modifiers.contains("CTRL") || modifiers.contains("CONTROL"),
            alt: modifiers.contains("ALT") || modifiers.contains("MOD1"),
            shift: modifiers.contains("SHIFT"),
            key: normalize_key(key),
        }
    }

    pub fn from_iced(key: Key, modifiers: Modifiers) -> Option<Self> {
        let key = key_label(&key)?;
        if matches!(key.as_str(), "Super" | "Ctrl" | "Alt" | "Shift") {
            return None;
        }
        Some(Self {
            super_key: modifiers.logo(),
            ctrl: modifiers.control(),
            alt: modifiers.alt(),
            shift: modifiers.shift(),
            key,
        })
    }

    pub fn parts(&self) -> Vec<String> {
        let mut parts = Vec::new();
        if self.super_key {
            parts.push("Super".into());
        }
        if self.ctrl {
            parts.push("Ctrl".into());
        }
        if self.alt {
            parts.push("Alt".into());
        }
        if self.shift {
            parts.push("Shift".into());
        }
        parts.push(self.key.clone());
        parts
    }
}

impl fmt::Display for Hotkey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.parts().join(" + "))
    }
}

pub fn key_label(key: &Key) -> Option<String> {
    Some(match key.as_ref() {
        Key::Character(value) => normalize_key(value),
        Key::Named(named) => match format!("{named:?}").as_str() {
            "Super" => "Super".into(),
            "Control" => "Ctrl".into(),
            "Alt" => "Alt".into(),
            "Shift" => "Shift".into(),
            _ => named_key(named),
        },
        Key::Unidentified => return None,
    })
}

fn normalize_key(key: &str) -> String {
    match key.trim().trim_matches('$').to_ascii_lowercase().as_str() {
        "return" => "Enter".into(),
        "escape" => "Esc".into(),
        "space" => "Space".into(),
        value => value.to_ascii_uppercase(),
    }
}

fn named_key(key: Named) -> String {
    match key {
        Named::Enter => "Enter".into(),
        Named::Escape => "Esc".into(),
        Named::Space => "Space".into(),
        Named::Tab => "Tab".into(),
        Named::ArrowUp => "UP".into(),
        Named::ArrowDown => "DOWN".into(),
        Named::ArrowLeft => "LEFT".into(),
        Named::ArrowRight => "RIGHT".into(),
        Named::Backspace => "Backspace".into(),
        other => format!("{other:?}").to_ascii_uppercase(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hyprland_modifiers() {
        let hotkey = Hotkey::parse("$mainMod SHIFT", "3");
        assert!(hotkey.super_key);
        assert!(hotkey.shift);
        assert_eq!(hotkey.to_string(), "Super + Shift + 3");
    }
}

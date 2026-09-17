use std::{path::PathBuf, time::Instant};

use iced::Task;

use crate::{
    learning::{select_next, SessionFeedback},
    omarchy::{load_bindings, Binding},
    storage::{Progress, ProgressStore},
};

use super::Page;

#[derive(Debug, Clone)]
pub struct PressedKey {
    pub label: String,
    pub held: bool,
}

#[derive(Debug)]
pub struct State {
    pub page: Page,
    pub bindings: Vec<Binding>,
    pub current: Option<usize>,
    pub feedback: Option<SessionFeedback>,
    pub progress: Progress,
    pub config_path: PathBuf,
    pub store: Option<ProgressStore>,
    pub error: Option<String>,
    pub challenge_started: Instant,
    pub pressed_keys: Vec<PressedKey>,
    pub last_attempt: Vec<String>,
    pub onboarding_step: usize,
    pub show_hint: bool,
}

impl State {
    pub fn new(config_path: PathBuf) -> (Self, Task<super::Message>) {
        let store = ProgressStore::discover().ok();
        let progress = store
            .as_ref()
            .and_then(|store| store.load().ok())
            .unwrap_or_default();
        let (bindings, error) = match load_bindings(&config_path) {
            Ok(bindings) if !bindings.is_empty() => (bindings, None),
            Ok(_) => (Vec::new(), Some("No Hyprland bindings were found".into())),
            Err(error) => (Vec::new(), Some(error.to_string())),
        };
        let page = if progress.onboarding_complete || bindings.is_empty() {
            Page::Practice
        } else {
            Page::Learn
        };
        let current = if page == Page::Learn {
            Some(0)
        } else {
            select_next(&bindings, &progress, None)
        };

        (
            Self {
                page,
                bindings,
                current,
                feedback: None,
                progress,
                config_path,
                store,
                error,
                challenge_started: Instant::now(),
                pressed_keys: Vec::new(),
                last_attempt: Vec::new(),
                onboarding_step: 0,
                show_hint: false,
            },
            Task::none(),
        )
    }

    pub fn current_binding(&self) -> Option<&Binding> {
        self.current.and_then(|index| self.bindings.get(index))
    }

    pub fn save(&mut self) {
        if let Some(store) = &self.store {
            if let Err(error) = store.save(&self.progress) {
                self.error = Some(format!("Could not save progress: {error}"));
            }
        }
    }

    pub fn record_key_press(&mut self, label: String) {
        if self
            .pressed_keys
            .iter()
            .any(|key| key.label == label && key.held)
        {
            return;
        }
        self.pressed_keys.push(PressedKey { label, held: true });
    }

    pub fn record_key_release(&mut self, label: &str) {
        if let Some(key) = self
            .pressed_keys
            .iter_mut()
            .rev()
            .find(|key| key.label == label && key.held)
        {
            key.held = false;
        }
    }

    pub fn clear_attempt(&mut self) {
        self.pressed_keys.clear();
        self.feedback = None;
        self.show_hint = false;
        self.challenge_started = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use super::PressedKey;

    #[test]
    fn key_trace_keeps_each_press_and_release_state() {
        let mut keys = vec![PressedKey {
            label: "Super".into(),
            held: true,
        }];
        keys[0].held = false;
        keys.push(PressedKey {
            label: "Super".into(),
            held: true,
        });
        assert_eq!(keys.len(), 2);
        assert!(!keys[0].held);
        assert!(keys[1].held);
    }
}

use iced::Task;

use crate::{
    input::{key_label, Hotkey},
    learning::{score_attempt, select_next, SessionFeedback},
    omarchy::load_bindings,
};

use super::{Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Navigate(page) => {
            state.page = page;
            state.clear_attempt();
            if page == super::Page::Learn && !state.progress.onboarding_complete {
                state.current = Some(state.onboarding_step);
            } else if page == super::Page::Practice {
                state.current = select_next(&state.bindings, &state.progress, state.current);
            }
        }
        Message::KeyPressed(key, modifiers) => {
            let is_training = state.page == super::Page::Practice
                || (state.page == super::Page::Learn && !state.progress.onboarding_complete);
            if !is_training || state.feedback.is_some() || state.current_binding().is_none() {
                return Task::none();
            }
            if let Some(label) = key_label(&key) {
                state.record_key_press(label);
            }
            let Some(pressed) = Hotkey::from_iced(key, modifiers) else {
                return Task::none();
            };
            let expected = state
                .current_binding()
                .map(|binding| binding.hotkey.clone());
            if let Some(expected) = expected {
                let result = score_attempt(
                    &expected,
                    &pressed,
                    state.challenge_started.elapsed(),
                    state.progress.streak,
                );
                state.progress.record(&expected, &result);
                state.feedback = Some(SessionFeedback::from(result));
                state.last_attempt = state
                    .pressed_keys
                    .iter()
                    .map(|key| key.label.clone())
                    .collect();
                state.save();
            }
        }
        Message::KeyReleased(key) => {
            if let Some(label) = key_label(&key) {
                state.record_key_release(&label);
            }
        }
        Message::NextChallenge | Message::SkipChallenge => {
            if state.page == super::Page::Learn && !state.progress.onboarding_complete {
                let lesson_count = state.bindings.len().min(5);
                state.onboarding_step += 1;
                if state.onboarding_step >= lesson_count {
                    state.progress.onboarding_complete = true;
                    state.page = super::Page::Practice;
                    state.current = select_next(&state.bindings, &state.progress, state.current);
                    state.save();
                } else {
                    state.current = Some(state.onboarding_step);
                }
            } else {
                state.current = select_next(&state.bindings, &state.progress, state.current);
            }
            state.clear_attempt();
        }
        Message::ShowHint => state.show_hint = true,
        Message::ReloadBindings => match load_bindings(&state.config_path) {
            Ok(bindings) if !bindings.is_empty() => {
                state.bindings = bindings;
                state.current =
                    if state.page == super::Page::Learn && !state.progress.onboarding_complete {
                        Some(state.onboarding_step.min(state.bindings.len() - 1))
                    } else {
                        select_next(&state.bindings, &state.progress, None)
                    };
                state.clear_attempt();
                state.error = None;
            }
            Ok(_) => state.error = Some("No Hyprland bindings were found".into()),
            Err(error) => state.error = Some(error.to_string()),
        },
        Message::ResetProgress => {
            state.progress = Default::default();
            state.page = super::Page::Learn;
            state.onboarding_step = 0;
            state.current = (!state.bindings.is_empty()).then_some(0);
            state.clear_attempt();
            state.save();
        }
    }
    Task::none()
}

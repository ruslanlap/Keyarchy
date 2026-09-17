use iced::{
    alignment,
    widget::{button, column, container, horizontal_rule, row, text},
    Element, Fill,
};

use crate::app::{Message, State};

pub fn view(state: &State) -> Element<'_, Message> {
    let content: Element<'_, Message> = if let Some(binding) = state.current_binding() {
        let pressed = state
            .pressed_keys
            .iter()
            .map(|key| (key.label.clone(), key.held))
            .collect();
        let status: Element<'_, Message> = if let Some(feedback) = &state.feedback {
            let marker = if feedback.correct { "✓" } else { "×" };
            column![
                text(format!("{marker} {}", feedback.title)).size(30),
                text(&feedback.detail).size(16),
                button("Next challenge →")
                    .padding([12, 22])
                    .on_press(Message::NextChallenge),
            ]
            .align_x(alignment::Horizontal::Center)
            .spacing(12)
            .into()
        } else {
            column![
                text("Live input").size(15),
                super::keycaps(pressed),
                if state.show_hint {
                    text(format!("Hint: {}", binding.hotkey)).size(16)
                } else {
                    text("Every pressed key will appear here.").size(14)
                },
                row![
                    button("Show keys").on_press(Message::ShowHint),
                    button("Skip").on_press(Message::SkipChallenge),
                ]
                .spacing(10),
            ]
            .align_x(alignment::Horizontal::Center)
            .spacing(8)
            .into()
        };
        column![
            text("PRACTICE").size(14),
            text(&binding.action).size(34),
            text("Press the matching hotkey").size(17),
            horizontal_rule(1),
            status,
            text("Input is captured only while this window is focused.").size(12),
        ]
        .align_x(alignment::Horizontal::Center)
        .spacing(24)
        .into()
    } else {
        column![
            text("No shortcuts available").size(30),
            text(
                state
                    .error
                    .as_deref()
                    .unwrap_or("Choose a Hyprland configuration in Settings.")
            ),
            button("Reload configuration").on_press(Message::ReloadBindings),
        ]
        .align_x(alignment::Horizontal::Center)
        .spacing(16)
        .into()
    };

    container(content).center_x(Fill).center_y(Fill).into()
}

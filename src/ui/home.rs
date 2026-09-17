use iced::{
    alignment,
    widget::{button, column, container, horizontal_rule, row, text, vertical_space},
    Element, Fill,
};

use crate::app::{Message, State};

pub fn view(state: &State) -> Element<'_, Message> {
    if !state.progress.onboarding_complete && !state.bindings.is_empty() {
        return onboarding(state);
    }

    let list = state.bindings.iter().take(14).fold(
        column![
            text("Shortcut library").size(32),
            text(format!("{} shortcuts loaded", state.bindings.len())).size(16)
        ]
        .spacing(14),
        |column, binding| {
            column.push(
                container(
                    column![
                        text(&binding.action).size(17),
                        text(binding.hotkey.to_string()).size(14),
                        text(format!("from {}", binding.source)).size(11)
                    ]
                    .spacing(4),
                )
                .padding(12)
                .width(Fill)
                .style(container::bordered_box),
            )
        },
    );
    list.into()
}

fn onboarding(state: &State) -> Element<'_, Message> {
    let lesson_count = state.bindings.len().min(5);
    let step = state.onboarding_step.min(lesson_count.saturating_sub(1));
    let binding = &state.bindings[step];
    let pressed = state
        .pressed_keys
        .iter()
        .map(|key| (key.label.clone(), key.held))
        .collect();
    let events = if state.pressed_keys.is_empty() {
        "Waiting for your first key".into()
    } else {
        state
            .pressed_keys
            .iter()
            .map(|key| {
                format!(
                    "{} {}",
                    key.label,
                    if key.held { "down" } else { "released" }
                )
            })
            .collect::<Vec<_>>()
            .join("  ·  ")
    };
    let last_attempt: Element<'_, Message> = if state.last_attempt.is_empty() {
        vertical_space().height(0).into()
    } else {
        column![
            text("Last attempt").size(14),
            super::keycaps(
                state
                    .last_attempt
                    .iter()
                    .cloned()
                    .map(|label| (label, false))
                    .collect(),
            ),
        ]
        .spacing(8)
        .into()
    };
    let result: Element<'_, Message> = if let Some(feedback) = &state.feedback {
        column![
            text(if feedback.correct {
                "✓ Correct"
            } else {
                "× Try the next one"
            })
            .size(22),
            text(&feedback.detail).size(14),
            button(if step + 1 == lesson_count {
                "Start practice →"
            } else {
                "Next shortcut →"
            })
            .padding([11, 18])
            .on_press(Message::NextChallenge),
        ]
        .spacing(9)
        .into()
    } else {
        button("Skip for now")
            .on_press(Message::SkipChallenge)
            .into()
    };

    let rail = container(
        column![
            text("Welcome").size(30),
            text("Build visual memory with your real shortcuts.").size(15),
            vertical_space().height(18),
            text("✓  Config found").size(17),
            text(format!("   {} shortcuts loaded", state.bindings.len())).size(14),
            vertical_space().height(16),
            text(format!("{}  Learn 5 essentials", step + 1)).size(17),
            text("   See every key as you press it.").size(14),
            vertical_space().height(16),
            text("○  Ready to practice").size(17),
        ]
        .spacing(8),
    )
    .width(245)
    .height(Fill)
    .padding([14, 24])
    .style(super::dark_panel);

    let main = column![
        row![
            text("YOUR FIRST SHORTCUT").size(13),
            iced::widget::horizontal_space(),
            text(format!("{} of {}", step + 1, lesson_count)).size(16),
        ],
        text(format!("Press {}", binding.hotkey)).size(32),
        text("Watch every key appear as you press it.").size(17),
        vertical_space().height(8),
        text("Live input").size(14),
        super::keycaps(pressed),
        text(events).size(13),
        last_attempt,
        horizontal_rule(1),
        result,
        vertical_space(),
        text(format!("Loaded from {}", state.config_path.display())).size(12),
        text("Input stays local and is read only while this window is focused.").size(12),
    ]
    .spacing(13)
    .align_x(alignment::Horizontal::Left);

    row![rail, container(main).padding([10, 28]).width(Fill)]
        .height(Fill)
        .into()
}

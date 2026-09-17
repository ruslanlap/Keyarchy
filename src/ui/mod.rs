mod home;
mod practice;
mod progress;
mod settings;

use iced::{
    widget::{button, column, container, row, text, vertical_space, Row},
    Border, Color, Element, Fill, Theme,
};

use crate::app::{Message, Page, State};

pub fn view(state: &State) -> Element<'_, Message> {
    let navigation = [
        (Page::Learn, "Learn"),
        (Page::Practice, "Practice"),
        (Page::WeakKeys, "Weak keys"),
        (Page::Progress, "Progress"),
        (Page::Settings, "Settings"),
    ]
    .into_iter()
    .fold(
        column![text("KEYARCHY").size(24)].spacing(12),
        |menu, (page, label)| {
            let active = state.page == page;
            let mut item = button(text(label).size(16))
                .width(Fill)
                .padding(12)
                .style(if active {
                    button::secondary
                } else {
                    button::text
                });
            if !active {
                item = item.on_press(Message::Navigate(page));
            }
            menu.push(item)
        },
    );

    let sidebar = container(
        column![
            navigation,
            vertical_space(),
            text(format!("{} day streak", state.progress.streak)),
            text(format!("{} XP", state.progress.xp)).size(14),
        ]
        .spacing(8),
    )
    .width(220)
    .height(Fill)
    .padding(24);

    let content: Element<'_, Message> = match state.page {
        Page::Learn => home::view(state),
        Page::Practice => practice::view(state),
        Page::WeakKeys => progress::weak_keys(state),
        Page::Progress => progress::view(state),
        Page::Settings => settings::view(state),
    };

    row![
        sidebar,
        container(content).padding(40).width(Fill).height(Fill)
    ]
    .into()
}

pub(crate) fn keycaps(keys: Vec<(String, bool)>) -> Element<'static, Message> {
    if keys.is_empty() {
        return container(text("Press a key…").size(16))
            .padding([14, 18])
            .style(container::bordered_box)
            .into();
    }

    keys.into_iter()
        .enumerate()
        .fold(
            Row::new().spacing(10).align_y(iced::Center),
            |row, (index, (label, held))| {
                let row = if index > 0 {
                    row.push(text("+").size(24))
                } else {
                    row
                };
                row.push(container(text(label).size(18)).padding([13, 18]).style(
                    move |_theme: &Theme| {
                        container::Style::default()
                            .color(Color::from_rgb8(220, 225, 255))
                            .background(if held {
                                Color::from_rgb8(65, 72, 155)
                            } else {
                                Color::from_rgb8(30, 36, 58)
                            })
                            .border(
                                Border::default()
                                    .color(if held {
                                        Color::from_rgb8(122, 130, 255)
                                    } else {
                                        Color::from_rgb8(70, 78, 112)
                                    })
                                    .width(1)
                                    .rounded(9),
                            )
                    },
                ))
            },
        )
        .into()
}

pub(crate) fn dark_panel(_theme: &Theme) -> container::Style {
    container::Style::default()
        .background(Color::from_rgb8(24, 28, 46))
        .border(
            Border::default()
                .color(Color::from_rgb8(45, 53, 82))
                .width(1),
        )
}

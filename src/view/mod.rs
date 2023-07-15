use std::path::PathBuf;

use crate::model::Model;
use crate::model::Report;
use iced::widget::column;
use iced::widget::*;
use iced::Element;
use iced::Renderer;
use iced_aw::Spinner;
use iced_native::widget::Column;
use iced_native::{alignment::Horizontal, Alignment, Color, Length};

mod style;

#[derive(Debug, Clone)]
pub enum Message {
    RemoveDropped(usize),
    Clear,
    Save,
}

pub fn view(model: &Model) -> Element<Message> {
    let title = text(if !model.valid() {
        "Uno o piu' file hanno dei problemi"
    } else if model.saved {
        "Elenco salvato!"
    } else {
        "Trascina uno o piu' file .csv da convertire"
    })
    .width(Length::Fill)
    .horizontal_alignment(Horizontal::Center);

    let save_button = button("Salva");

    column![
        title,
        container(scrollable(column![
            column![Column::with_children(
                model
                    .dropped_files
                    .iter()
                    .enumerate()
                    .map(|(i, (p, r))| report_view(p, r, Message::RemoveDropped(i)))
                    .collect()
            )],
            vertical_space(32),
            column![Column::with_children(
                model.hovering_files.iter().map(|p| path_view(p)).collect()
            )],
        ]))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(if model.is_hovering() {
            style::bordered_container()
        } else {
            style::normal_container()
        }),
        row![
            container(button("Azzera").on_press(Message::Clear))
                .align_x(Horizontal::Center)
                .width(Length::Fill),
            container(if model.valid() {
                save_button.on_press(Message::Save)
            } else {
                save_button
            })
            .align_x(Horizontal::Center)
            .width(Length::Fill)
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
    ]
    .padding(16)
    .into()
}

fn path_view<'a>(path: &PathBuf) -> Element<'a, Message> {
    column![
        text(path.file_name().and_then(|f| f.to_str()).unwrap_or("ERROR")),
        horizontal_rule(2),
        //container(horizontal_space(320)).style(style::line_container())
    ]
    .align_items(Alignment::Center)
    .spacing(4)
    .padding(4)
    .into()
}

fn report_view<'a>(path: &PathBuf, report: &Report, msg: Message) -> Element<'a, Message> {
    let name: Element<'a, Message> =
        text(path.file_name().and_then(|f| f.to_str()).unwrap_or("ERROR")).into();
    column![
        row![
            name,
            container(
                container(match report {
                    Report::Waiting => Spinner::new().into(),
                    Report::Valid { .. } => <iced_native::widget::Text<'_, Renderer> as Into<
                        Element<'a, Message>,
                    >>::into(text("OK")),
                    Report::Error(string) =>
                        text(string).style(Color::from([0.8, 0.0, 0.0])).into(),
                })
                .center_x()
            )
            .width(Length::Fill),
            button("X").on_press(msg)
        ]
        .align_items(Alignment::Center)
        .spacing(32),
        horizontal_rule(2),
        //container(horizontal_space(320)).style(style::line_container())
    ]
    .align_items(Alignment::Center)
    .spacing(4)
    .padding(16)
    .into()
}

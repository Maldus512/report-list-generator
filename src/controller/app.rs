use std::path::PathBuf;

use crate::controller::report::{read_report, save_report_list};
use crate::model::{Model, Report};
use crate::view;
use iced::{self, subscription, Application, Command, Element, Subscription};

#[derive(Debug)]
pub enum Event {
    View(view::Message),
    System(iced::Event),
    Loaded(PathBuf, Report),
    Saved,
}

pub struct App {
    model: Model,
}

impl Application for App {
    type Message = Event;
    type Theme = iced::theme::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new((): Self::Flags) -> (App, Command<Event>) {
        (
            App {
                model: Model::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Generatore elenco prodotti")
    }

    fn update(&mut self, event: Event) -> Command<Event> {
        use Event::*;
        match event {
            System(iced::Event::Window(iced::window::Event::FileHovered(path))) => {
                if path.extension().map(|e| e == "csv").unwrap_or(false) {
                    self.model.hovering_files.push(path);
                }
                Command::none()
            }
            System(iced::Event::Window(iced::window::Event::FileDropped(path))) => {
                if path.extension().map(|e| e == "csv").unwrap_or(false) {
                    if let Some(index) = self.model.hovering_files.iter().position(|x| *x == path) {
                        self.model.hovering_files.remove(index);
                    }
                    self.model
                        .dropped_files
                        .push((path.clone(), Report::default()));
                    Command::perform(read_report(path.clone()), |r| Event::Loaded(path, r))
                } else {
                    Command::none()
                }
            }
            System(iced::Event::Window(iced::window::Event::FilesHoveredLeft)) => {
                self.model.hovering_files = vec![];
                Command::none()
            }
            System(_) => Command::none(),

            View(view::Message::RemoveDropped(index)) => {
                self.model.dropped_files.remove(index);
                Command::none()
            }
            View(view::Message::Clear) => {
                self.model.dropped_files = vec![];
                Command::none()
            }
            View(view::Message::Save) => Command::perform(
                save_report_list(
                    self.model
                        .dropped_files
                        .iter()
                        .map(|(_, r)| r.clone())
                        .collect(),
                ),
                |_| Event::Saved,
            ),

            Saved => {
                self.model.saved = true;
                Command::none()
            }
            Loaded(path, report) => {
                if let Some(reference) = self
                    .model
                    .dropped_files
                    .iter()
                    .position(|(p, _)| *p == path)
                    .and_then(|i| self.model.dropped_files.get_mut(i))
                {
                    *reference = (path, report);
                }
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Event> {
        subscription::events().map(Event::System)
    }

    fn view(&self) -> Element<Event> {
        view::view(&self.model).map(Event::View)
    }

    fn theme(&self) -> iced::theme::Theme {
        iced::theme::Theme::Dark
    }
}

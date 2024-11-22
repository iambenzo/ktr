#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_input, toggler, Space,
};
use iced::widget::{Button, Column};
use iced::{Center, Element, Fill, Task, Theme};
use kindle_clippings::model::Book;
use kindle_clippings::output::render_output;
use kindle_clippings::{parse_clippings, read_file_string};
use rfd::{AsyncFileDialog, FileHandle};

use self::book_toggle::BookToggler;
mod book_toggle;

fn main() -> iced::Result {
    iced::application(Ktr::title, Ktr::update, Ktr::view)
        .centered()
        .theme(Ktr::theme)
        .run()
}

#[derive(Debug)]
pub struct Ktr {
    screen: Screen,
    input: Option<FileHandle>,
    use_template: bool,
    template: Option<FileHandle>,
    output_dir: Option<FileHandle>,
    library: HashMap<String, Book>,
    filter_text: String,
    filtered_library: HashMap<String, Book>,
    selected_library: HashMap<String, Book>,
    output_created: bool,
}

#[derive(Debug, Clone)]
enum Message {
    BackPressed,
    NextPressed,
    OpenClippings,
    InputChanged(Option<FileHandle>),
    LibraryChanged(HashMap<String, Book>),
    OpenTemplate,
    UseTemplate(bool),
    TemplateChanged(Option<FileHandle>),
    BookToggled((bool, String)),
    FilterTextChanged(String),
    SelectAllBooks,
    SelectNoBooks,
    ChooseOutputDir,
    OutputDirChanged(Option<FileHandle>),
    OutputCreated(bool),
    Exit,
}

impl Ktr {
    fn title(&self) -> String {
        "Kindle to References".to_string()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNightStorm
    }

    fn update(&mut self, event: Message) -> Task<Message> {
        match event {
            Message::BackPressed => {
                if let Some(screen) = self.screen.previous() {
                    self.screen = screen;
                }
            }
            Message::NextPressed => {
                if let Some(screen) = self.screen.next() {
                    self.screen = screen;
                    if screen == Screen::End {
                        return Task::perform(
                            create_reference_files(
                                self.selected_library.clone(),
                                self.template.clone(),
                                self.output_dir.clone(),
                            ),
                            Message::OutputCreated,
                        );
                    }
                }
            }
            Message::OpenClippings => {
                return Task::perform(open_clippings(), Message::InputChanged)
            }
            Message::InputChanged(i) => {
                self.input = i;
                if self.input.is_some() {
                    return Task::perform(
                        parse_library(self.input.clone().unwrap()),
                        Message::LibraryChanged,
                    );
                }
            }
            Message::LibraryChanged(l) => {
                self.library = l;
                self.filtered_library = self.library.clone();
            }
            Message::OpenTemplate => {
                return Task::perform(open_template(), Message::TemplateChanged)
            }
            Message::UseTemplate(u) => {
                self.use_template = u;
            }
            Message::TemplateChanged(t) => {
                self.template = t;
            }
            Message::BookToggled((t, b)) => {
                if t {
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        self.selected_library.entry(b.clone())
                    {
                        e.insert(self.library.get(&b).unwrap().clone());
                    }
                } else {
                    self.selected_library.remove(&b);
                }
                // dbg!(&self.selected_library);
            }
            Message::FilterTextChanged(s) => {
                self.filter_text = s;
                if self.filter_text.is_empty() {
                    self.filtered_library = self.library.clone();
                } else {
                    self.filtered_library = self
                        .library
                        .iter()
                        .filter(|(k, _v)| {
                            k.to_lowercase().contains(&self.filter_text.to_lowercase())
                        })
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();
                }
                // dbg!(&self.filtered_library);
            }
            Message::SelectAllBooks => {
                self.selected_library = self.library.clone();
            }
            Message::SelectNoBooks => {
                self.selected_library.clear();
            }
            Message::ChooseOutputDir => {
                return Task::perform(open_target_dir(), Message::OutputDirChanged)
            }
            Message::OutputDirChanged(o) => {
                self.output_dir = o;
            }
            Message::OutputCreated(b) => {
                self.output_created = b;
            }
            Message::Exit => {
                ::std::process::exit(0);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let pager_controls = row![]
            .push_maybe(self.screen.previous().is_some().then(|| {
                padded_button("Back")
                    .on_press(Message::BackPressed)
                    .style(button::secondary)
            }))
            .push(horizontal_space())
            .push_maybe(
                self.can_continue()
                    .then(|| padded_button("Next").on_press(Message::NextPressed)),
            );

        let end_controls = row![]
            .push(horizontal_space())
            .push(padded_button("Exit").on_press(Message::Exit));

        let screen = match self.screen {
            Screen::SelectInput => self.select_input(),
            Screen::BookSelection => self.book_selection(),
            Screen::SelectOutput => self.select_output(),
            Screen::End => self.end(),
        };

        let content: Element<_> = match self.screen {
            Screen::End => column![screen, end_controls]
                .max_width(540)
                .spacing(20)
                .padding(20)
                .into(),
            _ => column![screen, pager_controls,]
                .max_width(540)
                .spacing(20)
                .padding(20)
                .into(),
        };

        let scrollable = scrollable(container(content).center_x(Fill));

        container(scrollable).center_y(Fill).into()
    }

    fn select_input(&self) -> Column<Message> {
        let selected_file = match &self.input {
            Some(f) => f.path().to_str().unwrap(),
            None => "None",
        };

        let file_input = text_input("Clippings file...", selected_file)
            .padding(10)
            .size(20);

        let clippings_btn = button("Open").padding(10).on_press(Message::OpenClippings);

        let selected_template = match &self.template {
            Some(f) => f.path().to_str().unwrap(),
            None => "None",
        };

        let template_toggle = toggler(self.use_template)
            .label("Use a custom output template?")
            .on_toggle(Message::UseTemplate);

        let template_input = text_input("Template file...", selected_template)
            .padding(10)
            .size(20);

        let template_btn = button("Open").padding(10).on_press(Message::OpenTemplate);

        if self.use_template {
            Self::container("Input Selection")
                .push("Open your 'My Clippings.txt' file")
                .push(row![file_input, clippings_btn].spacing(10).align_y(Center))
                .push(Space::new(0, 20))
                .push(row![template_toggle])
                .push(
                    row![template_input, template_btn]
                        .spacing(10)
                        .align_y(Center),
                )
                .push(Space::new(0, 20))
        } else {
            Self::container("Input Selection")
                .push("Open your 'My Clippings.txt' file")
                .push(row![file_input, clippings_btn].spacing(10).align_y(Center))
                .push(Space::new(0, 20))
                .push(row![template_toggle])
                .push(Space::new(0, 20))
        }
    }

    fn book_selection(&self) -> Column<Message> {
        let mut out = Self::container("Book Selection");

        if self.library.is_empty() {
            out = out.push("No books found in your clippings file")
        } else {
            let filter_input = text_input("Search", &self.filter_text)
                .on_input(Message::FilterTextChanged)
                .padding(10)
                .size(20);

            let select_none_btn = button("Select None")
                .padding(10)
                .style(button::secondary)
                .on_press(Message::SelectNoBooks);

            let select_all_btn = button("Select All")
                .padding(10)
                .on_press(Message::SelectAllBooks);

            out = out.push(row![filter_input]);
            out = out
                .push(row![select_none_btn, horizontal_space(), select_all_btn])
                .push(Space::new(0, 20));

            for (title, _book) in self.filtered_library.iter() {
                out = out.push(
                    BookToggler::new(self.selected_library.contains_key(title))
                        .label(title)
                        .on_toggle(Message::BookToggled),
                );
            }
        }

        out
    }

    fn select_output(&self) -> Column<Message> {
        let selected_dir = match &self.output_dir {
            Some(f) => f.path().to_str().unwrap(),
            None => "None",
        };

        let output_dir = text_input("Output directory...", selected_dir)
            .padding(10)
            .size(20);

        let output_btn = button("Open")
            .padding(10)
            .on_press(Message::ChooseOutputDir);

        Self::container("Selecting output")
            .push("Choose your output directory")
            .push(row![output_dir, output_btn].spacing(10).align_y(Center))
            .push(Space::new(0, 20))
    }

    fn end(&self) -> Column<Message> {
        if self.output_created {
            Self::container("Done!").push(Space::new(0, 20))
        } else {
            Self::container("Working...")
        }
    }

    fn can_continue(&self) -> bool {
        match self.screen {
            Screen::SelectInput => self.input.is_some(),
            Screen::BookSelection => !self.selected_library.is_empty(),
            Screen::SelectOutput => self.output_dir.is_some(),
            Screen::End => false,
        }
    }

    fn container(title: &str) -> Column<'_, Message> {
        column![text(title).size(50)].spacing(20)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    SelectInput,
    BookSelection,
    SelectOutput,
    End,
}

impl Screen {
    const ALL: &'static [Self] = &[
        Self::SelectInput,
        Self::BookSelection,
        Self::SelectOutput,
        Self::End,
    ];

    pub fn next(self) -> Option<Screen> {
        Self::ALL
            .get(
                Self::ALL
                    .iter()
                    .copied()
                    .position(|screen| screen == self)
                    .expect("Screen must exist")
                    + 1,
            )
            .copied()
    }

    pub fn previous(self) -> Option<Screen> {
        let position = Self::ALL
            .iter()
            .copied()
            .position(|screen| screen == self)
            .expect("Screen must exist");

        if position > 0 {
            Some(Self::ALL[position - 1])
        } else {
            None
        }
    }
}

fn padded_button<Message: Clone>(label: &str) -> Button<'_, Message> {
    button(text(label)).padding([12, 24])
}

async fn open_clippings() -> Option<FileHandle> {
    AsyncFileDialog::new()
        .add_filter("text", &["txt"])
        .pick_file()
        .await
}

async fn open_template() -> Option<FileHandle> {
    AsyncFileDialog::new()
        .add_filter("markdown", &["md"])
        .pick_file()
        .await
}

async fn open_target_dir() -> Option<FileHandle> {
    AsyncFileDialog::new().pick_folder().await
}

async fn parse_library(clippings: FileHandle) -> HashMap<String, Book> {
    let mut books: HashMap<String, Book> = HashMap::new();
    if let Ok(s) = read_file_string(clippings.path()) {
        books = parse_clippings(s);
    }

    books
}

async fn create_reference_files(
    lib: HashMap<String, Book>,
    template: Option<FileHandle>,
    output_dir: Option<FileHandle>,
) -> bool {
    let template = template.map(|t| t.path().to_path_buf());

    for (_, book) in lib.iter() {
        if let Err(e) = render_output(book, &template, output_dir.clone().unwrap().path()) {
            eprintln!("{}", e);
        }
    }

    true
}

impl Default for Ktr {
    fn default() -> Self {
        Self {
            screen: Screen::SelectInput,
            input: None,
            template: None,
            output_dir: None,
            use_template: false,
            library: HashMap::new(),
            filter_text: "".to_string(),
            filtered_library: HashMap::new(),
            selected_library: HashMap::new(),
            output_created: false,
        }
    }
}

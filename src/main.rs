use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::env::args;

use git2::{BranchType, Repository};
use iced::executor::Null;
use iced::{
    scrollable, Application, Checkbox, Column, Command, Container, Element, Length, Scrollable,
    Settings, Text,
};

pub fn main() {
    App::run(Settings::default())
}

struct App {
    repo: git2::Repository,
    selected_branches: HashSet<String>,
}

#[derive(Debug, Clone)]
enum Message {
    BranchSelected(String, bool),
}

impl Application for App {
    type Executor = Null;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let path = args().nth(1).unwrap_or(".".to_string());
        let repo = Repository::open(path).expect("Failed to open repository");
        let selected_branches = repo.branches(Some(BranchType::Local)).unwrap().fold(
            HashSet::new(),
            |mut aggr, branch| match branch {
                Ok((branch, _type)) => match branch.name() {
                    Ok(Some(name)) => {
                        aggr.insert(name.to_string());
                        aggr
                    }
                    _ => aggr,
                },
                Err(_) => aggr,
            },
        );

        (
            Self {
                repo,
                selected_branches,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let gitegylet = "Gitegylet".to_string();

        match self.repo.workdir() {
            Some(pwd) => match pwd.file_name() {
                Some(file) => match file.to_str() {
                    Some(name) => format!("{} | {}", name, gitegylet),
                    None => gitegylet,
                },
                None => gitegylet,
            },
            None => gitegylet,
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::BranchSelected(name, selected) => {
                let name = name.clone();
                if selected {
                    self.selected_branches.insert(name);
                } else {
                    self.selected_branches.remove(&name);
                }
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let branches = self.repo.branches(Some(BranchType::Local)).unwrap();
        let column = branches.fold(Column::new(), |col, branch| match branch {
            Ok((branch, _type)) => match branch.name() {
                Ok(Some(name)) => {
                    let selected = self.selected_branches.contains(name);
                    let line = Checkbox::new(selected, "name", move |message| {
                        Message::BranchSelected("foo".to_string(), message) // can't send branch name
                    })
                    .width(Length::Fill);
                    col.push(line)
                }
                Ok(_) => col,
                Err(_) => col,
            },
            Err(_) => col,
        });

        Container::new(column)
            // .style(style::Container)
            .width(Length::Fill)
            .into()
    }
}

// struct Branch<'a> {
//     branch: git2::Branch<'a>,
//     selected: bool,
// }
//
// impl Branch<'_> {
//     pub fn current(&self) -> bool {
//         self.branch.is_head()
//     }
//
//     pub fn local(&self) -> bool {
//         !self.branch.get().is_remote()
//     }
// }

struct Commit<'a> {
    commit: git2::Commit<'a>,
    selected: bool,
}

impl Ord for Commit<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.commit.time().cmp(&other.commit.time())
    }
}

impl PartialOrd for Commit<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Commit<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.commit.id() == other.commit.id()
    }
}

impl Eq for Commit<'_> {}

// impl Commit {
//   fn view(&mut self) -> Element<'_, Gitegylet::Message> {
//
//   }
// }

struct Gitegylet {
    repo: Repository,
    scroll: scrollable::State,
}

impl Gitegylet {
    fn commits(&self) -> Vec<Commit> {
        let branches = self.repo.branches(Some(BranchType::Local)).unwrap();
        let mut ids = HashSet::new();
        let mut heap = BinaryHeap::new();
        branches.for_each(|b| match b {
            Ok((branch, _bt)) => match branch.get().peel_to_commit() {
                Ok(commit) => {
                    ids.insert(commit.id());
                    heap.push(Commit {
                        commit,
                        selected: false,
                    });
                }
                Err(_) => {}
            },
            Err(_) => {}
        });
        if heap.is_empty() {
            return vec![];
        }

        let mut vector: Vec<Commit> = vec![];
        while vector.len() < 60 {
            match heap.pop() {
                Some(commit) => {
                    commit.commit.parents().for_each(|parent| {
                        if !ids.contains(&parent.id()) {
                            ids.insert(parent.id());
                            heap.push(Commit {
                                commit: parent,
                                selected: false,
                            });
                        }
                    });
                    vector.push(commit);
                }
                None => break,
            }
        }
        return vector;
    }
}

pub enum CommitMessage {
    Selected(bool),
}

impl Application for Gitegylet {
    type Executor = Null;
    type Message = ();
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let path = args().nth(1).unwrap_or(".".to_string());
        let repo = Repository::open(path).expect("Failed to open repository");

        (
            Self {
                repo,
                scroll: scrollable::State::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let gitegylet = "Gitegylet".to_string();

        match self.repo.workdir() {
            Some(pwd) => match pwd.file_name() {
                Some(file) => match file.to_str() {
                    Some(name) => format!("{} | {}", name, gitegylet),
                    None => gitegylet,
                },
                None => gitegylet,
            },
            None => gitegylet,
        }
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let commits = self.commits();
        let column = commits.iter().fold(Column::new(), |col, commit| {
            let message = match commit.commit.summary() {
                Some(msg) => msg.to_string(),
                None => commit.commit.id().to_string(),
            };

            let element = Container::new(Text::new(message))
                .style(style::Commit)
                .width(Length::Fill);

            col.push(element)
        });

        drop(commits);

        let container = Container::new(column)
            .style(style::Container)
            .width(Length::Fill);

        Scrollable::new(&mut self.scroll)
            .push(container)
            .height(Length::Fill)
            .into()
    }
}

mod style;

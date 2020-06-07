use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::env::args;

use git2::{BranchType, Repository};
use iced::executor::Null;
use iced::{
    scrollable, Application, Checkbox, Column, Command, Container, Element, Length, Row,
    Scrollable, Settings, Text,
};

pub fn main() {
    Repo::run(Settings::default())
}

struct Repo {
    repo: git2::Repository,
    branches: Vec<Branch>,
}

#[derive(Debug, Clone)]
enum Message {
    BranchMessage(usize, BranchMessage),
}

impl Application for Repo {
    type Executor = Null;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let path = args().nth(1).unwrap_or(".".to_string());
        let repo = Repository::open(path).expect("Failed to open repository");
        let branches =
            repo.branches(Some(BranchType::Local))
                .unwrap()
                .fold(vec![], |mut aggr, branch| match branch {
                    Ok((branch, _type)) => match branch.name() {
                        Ok(Some(name)) => {
                            aggr.push(Branch::new(name.to_string()));
                            aggr
                        }
                        _ => aggr,
                    },
                    Err(_) => aggr,
                });

        (Self { repo, branches }, Command::none())
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
            Message::BranchMessage(i, message) => {
                if let Some(branch) = self.branches.get_mut(i) {
                    branch.update(message);
                }
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let branches =
            self.branches
                .iter_mut()
                .enumerate()
                .fold(Column::new(), |col, (i, branch)| {
                    col.push(
                        branch
                            .view()
                            .map(move |message| Message::BranchMessage(i, message)),
                    )
                });

        let row = Row::new().push(branches);

        Container::new(row)
            .style(style::Container)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

#[derive(Debug, Clone)]
enum BranchMessage {
    Selected(bool),
}

struct Branch {
    name: String,
    selected: bool,
}

impl Branch {
    fn new(name: String) -> Self {
        Self {
            name,
            selected: true,
        }
    }

    fn update(&mut self, message: BranchMessage) {
        match message {
            BranchMessage::Selected(selected) => self.selected = selected,
        }
    }

    fn view(&mut self) -> Element<BranchMessage> {
        let checkbox = Checkbox::new(self.selected, &self.name, BranchMessage::Selected)
            // .width(Length::Fill)
            .style(style::Branch);

        Row::new().padding(2).push(checkbox).into()
    }
}

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
                // .style(style::Commit)
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

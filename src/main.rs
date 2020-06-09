use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::env::args;

use git2::{BranchType, Repository};
use iced::executor::Null;
use iced::{
    scrollable, Application, Checkbox, Column, Command, Container, Element, Length, Row,
    Scrollable, Settings, Text, VerticalAlignment,
};

pub fn main() {
    Repo::run(Settings::default())
}

struct Repo {
    repo: git2::Repository,
    branches: Vec<Branch>,
    commits: Commits,
    // selected_commit: Commit,
}

#[derive(Debug, Clone)]
enum Message {
    BranchMessage(usize, BranchMessage),
    CommitMessage(usize, CommitMessage),
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
                            aggr.push(Branch {
                                name: name.to_string(),
                                selected: true,
                                head: branch.is_head(),
                            });
                            aggr
                        }
                        _ => aggr,
                    },
                    Err(_) => aggr,
                });
        let commits = Commits::new(&repo, &branches, 50);

        (
            Self {
                repo,
                branches,
                commits,
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
            Message::BranchMessage(i, message) => {
                if let Some(branch) = self.branches.get_mut(i) {
                    branch.update(message);
                }
                self.commits = Commits::new(&self.repo, &self.branches, 50);
                Command::none()
            }
            Message::CommitMessage(_i, _message) => Command::none(),
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let branches = self.branches.iter_mut().enumerate().fold(
            Column::new().width(Length::Units(260)),
            |col, (i, branch)| {
                col.push(
                    branch
                        .view()
                        .map(move |message| Message::BranchMessage(i, message)),
                )
            },
        );

        Container::new(Row::new().push(branches).push(self.commits.view()))
            .style(style::Window)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

#[derive(Debug, Clone)]
enum BranchMessage {
    Selected(bool),
}

#[derive(Clone)]
struct Branch {
    name: String,
    head: bool,
    selected: bool,
}

impl Branch {
    fn new(name: String) -> Self {
        Self {
            name,
            head: false,
            selected: true,
        }
    }

    fn update(&mut self, message: BranchMessage) {
        match message {
            BranchMessage::Selected(selected) => self.selected = selected,
        }
    }

    fn view(&mut self) -> Element<BranchMessage> {
        let checkbox = Checkbox::new(self.selected, "", BranchMessage::Selected)
            .style(style::BranchCheckbox)
            .spacing(6);
        let left_pad = Column::new().width(Length::Units(16));
        let text = Text::new(&self.name)
            .size(15)
            .height(Length::Units(28))
            .vertical_alignment(VerticalAlignment::Center);
        let right_pad = Column::new().width(Length::Units(16));

        let row = Row::new()
            .height(Length::Units(28))
            .padding(2)
            .push(left_pad)
            .push(checkbox)
            .push(text)
            .push(right_pad);

        Container::new(row)
            .style(if self.head {
                if self.selected {
                    style::Branch::Head
                } else {
                    style::Branch::UnselectedHead
                }
            } else {
                style::Branch::Normal
            })
            .width(Length::Fill)
            .into()
    }
}

struct Commits {
    commits: Vec<Commit>,
    scroll: scrollable::State,
}

impl Commits {
    fn default() -> Self {
        Self {
            commits: vec![],
            scroll: scrollable::State::default(),
        }
    }

    fn new(repo: &git2::Repository, branches: &[Branch], count: usize) -> Self {
        let mut ids = HashSet::new();
        let mut heap = BinaryHeap::new();
        branches
            .iter()
            .filter(|Branch { selected, .. }| *selected)
            .for_each(
                |Branch { name, .. }| match repo.find_branch(name, BranchType::Local) {
                    Ok(branch) => match branch.get().peel_to_commit() {
                        Ok(commit) => {
                            ids.insert(commit.id());
                            heap.push(Commit::from_git2(commit));
                        }
                        Err(_) => {}
                    },
                    Err(_) => {}
                },
            );

        if heap.is_empty() {
            return Self::default();
        }

        let mut commits: Vec<Commit> = vec![];
        while commits.len() < count {
            match heap.pop() {
                Some(commit) => {
                    repo.find_commit(commit.id)
                        .unwrap()
                        .parents()
                        .for_each(|parent| {
                            if !ids.contains(&parent.id()) {
                                ids.insert(parent.id());
                                heap.push(Commit::from_git2(parent));
                            }
                        });
                    commits.push(commit);
                }
                None => break,
            }
        }
        Self {
            commits,
            scroll: scrollable::State::default(),
        }
    }

    fn update(&mut self, _message: CommitMessage) {}

    fn view(&mut self) -> Element<Message> {
        let commits =
            self.commits
                .iter_mut()
                .enumerate()
                .fold(Column::new(), |col, (i, commit)| {
                    col.push(
                        commit
                            .view()
                            .map(move |message| Message::CommitMessage(i, message)),
                    )
                });

        Scrollable::new(&mut self.scroll)
            .push(commits)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[derive(Clone)]
struct Commit {
    id: git2::Oid,
    time: git2::Time,
    summary: String,
    message: String,
    author: git2::Signature<'static>,
    selected: bool,
}

#[derive(Debug, Clone)]
enum CommitMessage {
    Selected(bool),
}

impl Commit {
    fn from_git2(commit: git2::Commit) -> Self {
        Self {
            id: commit.id(),
            time: commit.time(),
            summary: match commit.summary() {
                Some(summary) => summary.to_string(),
                None => commit.id().to_string(),
            },
            message: match commit.message() {
                Some(msg) => msg.to_string(),
                None => commit.id().to_string(),
            },
            author: commit.author().to_owned(),
            selected: false,
        }
    }

    fn update(&mut self, message: CommitMessage) {
        match message {
            CommitMessage::Selected(selected) => self.selected = selected,
        }
    }

    fn view(&mut self) -> Element<CommitMessage> {
        let text = Text::new(self.summary.clone());

        Row::new().padding(2).push(text).into()
    }
}

impl Ord for Commit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Commit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Commit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Commit {}

mod style;

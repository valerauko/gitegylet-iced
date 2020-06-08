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

impl Repo {
    fn commits(&self) -> Vec<Commit> {
        let mut ids = HashSet::new();
        let mut heap = BinaryHeap::new();
        self.branches
            .iter()
            .filter(|Branch { selected, .. }| *selected)
            .for_each(
                |Branch { name, .. }| match self.repo.find_branch(name, BranchType::Local) {
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
            return vec![];
        }

        let mut vector: Vec<Commit> = vec![];
        while vector.len() < 60 {
            match heap.pop() {
                Some(commit) => {
                    self.repo
                        .find_commit(commit.id)
                        .unwrap()
                        .parents()
                        .for_each(|parent| {
                            if !ids.contains(&parent.id()) {
                                ids.insert(parent.id());
                                heap.push(Commit::from_git2(parent));
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
            Message::CommitMessage(i, message) => Command::none(),
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mut commit_vec = self.commits().to_owned();
        let commits = commit_vec
            .iter_mut()
            .enumerate()
            .fold(Column::new(), |col, (i, commit)| {
                col.push(
                    commit
                        .view()
                        .map(move |message| Message::CommitMessage(i, message)),
                )
            });

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

        let row = Row::new().push(branches).push(commits);

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
    id: git2::Oid,
    time: git2::Time,
    summary: String,
    message: String,
    author: git2::Signature<'a>,
    selected: bool,
}

#[derive(Debug, Clone)]
enum CommitMessage {
    Selected(bool),
}

impl Commit<'_> {
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

impl Ord for Commit<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Commit<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Commit<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Commit<'_> {}

// Scrollable::new(&mut self.scroll)
// .push(container)
// .height(Length::Fill)
// .into()

mod style;

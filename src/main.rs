use std::env::args;
use git2::{Repository, BranchType, Error, Oid, Branch};
use iced::{Application, Command, Element, Settings, Column, Scrollable, Length, Color};
use iced::executor::Null;
use std::collections::{BTreeMap, BinaryHeap, HashSet};
use std::cmp::{Ordering, max, min};
use iced_native::{Container, Text};

struct Commit<'a> {
  commit: git2::Commit<'a>
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

struct Iny {
  repo: Repository
}

impl Iny {
  fn commits(&self) -> Vec<Commit> {
    let branches = self.repo.branches(
      Some(BranchType::Local)
    ).unwrap();
    let mut ids = HashSet::new();
    let mut heap = BinaryHeap::new();
    branches.for_each(|b| {
      match b {
        Ok((branch, _bt)) => {
          match branch.get().peel_to_commit() {
            Ok(commit) => {
              ids.insert(commit.id());
              heap.push(Commit { commit });
            },
            Err(_) => {}
          }
        },
        Err(_) => {}
      }
    });
    if heap.is_empty() { return vec![] }

    let mut vector: Vec<Commit> = vec![];
    while vector.len() < 50 {
      match heap.pop() {
        Some(commit) => {
          commit.commit.parents().for_each(|parent| {
            if !ids.contains(&parent.id()) {
              ids.insert(parent.id());
              heap.push(Commit { commit: parent });
            }
          });
          vector.push(commit);
        },
        None => break
      }
    }
    return vector
  }
}

impl Application for Iny {
  type Executor = Null;
  type Message = ();
  type Flags = ();

  fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
    let path = args().nth(1).unwrap_or(".".to_string());
    let repo = Repository::open(path)
      .expect("Failed to open repository");

    (Self { repo }, Command::none())
  }

  fn title(&self) -> String {
    let iny = "Iny".to_string();

    match self.repo.workdir() {
      Some(pwd) => match pwd.file_name() {
        Some(file) => match file.to_str() {
          Some(name) => format!("{} | {}", name, iny),
          None => iny
        },
        None => iny
      },
      None => iny
    }
  }

  fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&mut self) -> Element<'_, Self::Message> {
    let commits = self.commits()
      .iter()
      .fold(
          Column::new().spacing(10),
          |col, commit| {
              let message = match commit.commit.message() {
                  Some(msg) => msg.to_string(),
                  None => commit.commit.id().to_string()
              };
              col.push(Text::new(message).size(16))
          }
      );

    Container::new(commits)
      .width(Length::Fill)
      .height(Length::Fill)
      .padding(10)
      .into()
  }
}

pub fn main() {
  Iny::run(Settings::default())
}

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::env::args;

use git2::{BranchType, Repository};
use iced::{Application, Color, Column, Command, Element, Length, Settings, Background};
use iced::executor::Null;
use iced_native::{Container, Text};
use iced::widget::container::Style;

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

// impl Commit {
//   fn view(&mut self) -> Element<'_, Gitegylet::Message> {
//
//   }
// }

struct Gitegylet {
  repo: Repository
}

impl Gitegylet {
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
    while vector.len() < 60 {
      match heap.pop() {
        Some(commit) => {
          println!("{}", commit.commit.summary().unwrap_or("msg missing"));
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

impl Application for Gitegylet {
  type Executor = Null;
  type Message = ();
  type Flags = ();

  fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    let path = args().nth(1).unwrap_or(".".to_string());
    let repo = Repository::open(path)
      .expect("Failed to open repository");

    (Self { repo }, Command::none())
  }

  fn title(&self) -> String {
    let gitegylet = "Gitegylet".to_string();

    match self.repo.workdir() {
      Some(pwd) => match pwd.file_name() {
        Some(file) => match file.to_str() {
          Some(name) => format!("{} | {}", name, gitegylet),
          None => gitegylet
        },
        None => gitegylet
      },
      None => gitegylet
    }
  }

  fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&mut self) -> Element<'_, Self::Message> {
    let commits = self.commits();
    println!("{}", commits.len());
    let column = commits
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

    Container::new(column)
      .style(style::Container)
      .width(Length::Fill)
      .height(Length::Fill)
      .padding(10)
      .into()
  }
}

mod style {
  use iced::container::{Style, StyleSheet};
  use iced::{Background, Color};

  pub struct Container;

  impl StyleSheet for Container {
    fn style(&self) -> Style {
      Style {
        background: Some(Background::Color(Color::from_rgb8(0x0, 0x0, 0x11))),
        text_color: Some(Color::from_rgb8(0xee, 0xee, 0xee)),
        ..Style::default()
      }
    }
  }
}

pub fn main() {
  Gitegylet::run(Settings::default())
}

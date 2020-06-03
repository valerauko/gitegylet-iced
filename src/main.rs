extern crate git2;

use git2::{Repository, Commit, Oid, BranchType};
use std::collections::{BinaryHeap, HashSet};
use chrono::{DateTime, NaiveDateTime, TimeZone, Local};
use std::cmp::{Ordering};

use iced::{
    widget::canvas::{self, Canvas, LineJoin, Path, Stroke},
    executor, Application, Color, Command, Container, Element, Length,
    Point, Settings,
};

struct ListedCommit<'a> {
    commit: Commit<'a>
}

impl Ord for ListedCommit<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.commit.time().cmp(&other.commit.time())
    }
}

impl PartialOrd for ListedCommit<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ListedCommit<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.commit.id() == other.commit.id()
    }
}

impl Eq for ListedCommit<'_> {}

struct CommitLog<'repo> {
    repo_path: String,
    tree: BinaryHeap<ListedCommit<'repo>>,
    ids: HashSet<Oid>
}

impl CommitLog<'_> {
    pub fn from_args<'repo>() -> CommitLog<'repo> {
        CommitLog {
            repo_path: std::env::args().nth(1).unwrap_or(".".to_string()),
            tree: BinaryHeap::new(),
            ids: HashSet::new()
        }
    }

    pub fn init(&mut self) {
        let repo = Repository::open(self.repo_path).expect("failed to open repo");

        let branches = repo.branches(Some(BranchType::Local)).unwrap();
        branches.map(|branch| branch.unwrap().0.get().peel_to_commit().unwrap())
                .for_each(|commit| {
                    self.ids.insert(commit.id());
                    self.tree.push(ListedCommit { commit: commit });
                });
    }
}

impl <'repo>Iterator for CommitLog<'repo> {
    type Item = ListedCommit<'repo>;

    fn next(&mut self) -> Option<ListedCommit<'repo>> {
        if self.tree.is_empty() {
            return None
        }
        let first = self.tree.pop().unwrap();
        first.commit.parents().for_each(|parent| {
            if !self.ids.contains(&parent.id()) {
                self.ids.insert(parent.id());
                self.tree.push(ListedCommit { commit: parent });
            }
        });
        Some(first)
    }
}

fn arg_to_path() -> String {
    std::env::args().nth(1).unwrap_or(".".to_string())
}

fn commit_time(commit: &Commit) -> DateTime<Local> {
    let timestamp = commit.time().seconds();
    Local.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp, 0))
}

pub fn main() {
    CommitLog::run(
        Settings {
            antialiasing: true,
            ..Settings::default()
        }
    )
}

impl Application for CommitLog<'_> {
    type Executor = executor::Null;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let mut log = CommitLog::from_args();
        log.init();
        (log, Command::none())
    }

    fn title(&self) -> String {
        String::from("Iny")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let canvas = Canvas::new()
            .width(Length::Units(400))
            .height(Length::Units(700));

        Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}

fn branch(from: Point, to: Point, r: f32) -> Path {
    Path::new(|p| {
        let pre_curve = Point::new(
            to.x - r, // x next to to-branch
            from.y // y from from-branch
        );
        let post_curve = Point::new(
            to.x, // x from to-branch
            from.y - r // y next to from-branch
        );
        p.move_to(from);
        p.arc_to(pre_curve, post_curve, r);
        p.line_to(to);
    })
}

fn merge(from: Point, to: Point, r: f32) -> Path {
    Path::new(|p| {
        let pre_curve = Point::new(
            from.x, // x from from-branch
            to.y + r // y next to to-branch
        );
        let post_curve = Point::new(
            from.x - r, // x next to from-branch
            to.y // y from to-branch
        );
        p.move_to(from);
        p.arc_to(pre_curve, post_curve, r);
        p.line_to(to);
    })
}

impl canvas::Drawable for CommitLog<'_> {
    fn draw(&self, frame: &mut canvas::Frame) {
        frame.with_save(|frame| {
            let r = 10.0;
            let b1_c1_point = Point::new(r, (1.0 + 3.0 * 0.0) * r);
            let b1_c1 = Path::circle(b1_c1_point, r);
            let b1_c2_point = Point::new(r, (1.0 + 3.0 * 2.0) * r);
            let b1_c2 = Path::circle(b1_c2_point, r);
            let b1_c1_c2 = Path::line(b1_c1_point, b1_c2_point);
            let b1_color = Color::from_rgb8(0x97, 0x97, 0x97);
            let b1_stroke = Stroke {
                width: 3.0,
                color: b1_color,
                ..Stroke::default()
            };

            let b2_c1_point = Point::new(4.0 * r, (1.0 + 3.0 * 1.0) * r);
            let b2_c1 = Path::circle(b2_c1_point, r);
            let b2_color = Color::from_rgb8(0x0, 0x90, 0xb5);
            let b2_stroke = Stroke {
                width: 3.0,
                color: b2_color,
                line_join: LineJoin::Round,
                ..Stroke::default()
            };
            let b2_branch = branch(b1_c2_point, b2_c1_point, r);
            let b2_merge = merge(b2_c1_point, b1_c1_point, r);

            frame.fill(&b2_c1, b2_color);
            frame.stroke(&b2_branch, b2_stroke);
            frame.stroke(&b2_merge, b2_stroke);

            frame.fill(&b1_c1, b1_color);
            frame.fill(&b1_c2, b1_color);
            frame.stroke(&b1_c1_c2, b1_stroke);
        })
    }
}

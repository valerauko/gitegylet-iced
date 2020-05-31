extern crate git2;

use git2::{Repository, Commit, Oid, BranchType};
use std::collections::{BinaryHeap, HashSet};
use chrono::{DateTime, NaiveDateTime, TimeZone, Local};
use std::cmp::{Ordering};

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

struct CommitList<'repo> {
    tree: BinaryHeap<ListedCommit<'repo>>,
    ids: HashSet<Oid>
}

impl CommitList<'_> {
    pub fn new<'repo>(repo: &'repo Repository) -> CommitList<'repo> {
        let mut tree = BinaryHeap::new();
        let mut ids = HashSet::new();
        let branches = repo.branches(Some(BranchType::Local)).unwrap();
        branches.map(|branch| branch.unwrap().0.get().peel_to_commit().unwrap())
                .for_each(|commit| {
                    ids.insert(commit.id());
                    tree.push(ListedCommit { commit: commit });
                });

        CommitList {
            tree: tree,
            ids: ids
        }
    }
}

impl <'repo>Iterator for CommitList<'repo> {
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

fn main() {
    let repo = Repository::open(arg_to_path()).expect("failed to open repo");
    CommitList::new(&repo).for_each(|w| println!("{}", w.commit.message().unwrap_or("")))
}

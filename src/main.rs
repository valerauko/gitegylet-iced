extern crate git2;

use git2::{Repository, Commit};
use std::collections::BinaryHeap;
use chrono::{DateTime, NaiveDateTime, TimeZone, Local};
use std::cmp::{Ordering};

struct WrappedCommit<'a> {
    commit: Commit<'a>
}

impl WrappedCommit<'_> {
    pub fn new(c: Commit) -> WrappedCommit {
        WrappedCommit {
            commit: c
        }
    }
}

impl Ord for WrappedCommit<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.commit.time().cmp(&other.commit.time())
    }
}

impl PartialOrd for WrappedCommit<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for WrappedCommit<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.commit.id() == other.commit.id()
    }
}

impl Eq for WrappedCommit<'_> {}

fn arg_to_path() -> String {
    std::env::args().nth(1).unwrap_or(".".to_string())
}

fn head_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    repo.head()?.resolve()?.peel_to_commit()
}

fn commit_time(commit: &Commit) -> DateTime<Local> {
    let timestamp = commit.time().seconds();

    Local.from_utc_datetime(&NaiveDateTime::from_timestamp(timestamp, 0))
}

fn main() {
    let repo = Repository::open(arg_to_path()).expect("failed to open repo");
    let branches = repo.branches(Some(BranchType::Local)).expect("no references?");
    let mut heap = BinaryHeap::new();

    branches.map(|branch| branch.unwrap().0.get().peel_to_commit().unwrap())
            .map(|one_ref| WrappedCommit::new(one_ref))
            .for_each(|wc| heap.push(wc));

    heap.into_sorted_vec().into_iter().for_each(|wc|
        println!("{} {}", wc.commit.message().unwrap_or("no message"), commit_time(&wc.commit))
    )
}

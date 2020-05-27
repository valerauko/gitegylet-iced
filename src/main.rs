extern crate git2;

use git2::{Repository, Commit};

fn arg_to_path() -> String {
    std::env::args().nth(1).unwrap_or(".".to_string())
}

fn head_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    repo.head()?.resolve()?.peel_to_commit()
}

fn main() {
    let repo = Repository::open(arg_to_path()).expect("failed to open repo");
    let mut refs = repo.references().expect("no references?");
    refs.names().for_each(|name| println!("{}", name.unwrap_or("no ref name?")))
}

extern crate git2;

use git2::Repository;

fn repo_root() -> String {
    std::env::args().nth(1).unwrap_or(".".to_string())
}

fn main() {
    let repo = Repository::open(repo_root()).expect("failed to open repo");
    println!("{} is {:?}", repo.path().display(), repo.state());
}

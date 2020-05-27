extern crate git2;

use git2::{Repository, Commit, ObjectType};

fn arg_to_path() -> String {
    std::env::args().nth(1).unwrap_or(".".to_string())
}

fn head_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(
        |_| git2::Error::from_str("no commit behind head")
    )
}

fn main() {
    let repo = Repository::open(arg_to_path()).expect("failed to open repo");
    let head = head_commit(&repo);
    match head {
        Ok(commit) => println!("{}", commit.message().unwrap_or("no message")),
        Err(e) => println!("{}", e),
    }
}

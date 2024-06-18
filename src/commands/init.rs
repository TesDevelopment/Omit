use std::path::PathBuf;


pub fn run_init() {
    let git_dir = PathBuf::from(".git");

    if !git_dir.exists() {
        eprintln!("No git directory found, omit is meant to be used in a git repository");
        return;
    }

    
}
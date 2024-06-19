use std::fs;



pub fn run_desync() {
    if fs::remove_dir_all(".omit").is_err() {
        println!("No omit directory found");
    }

    if fs::remove_file(".git/hooks/post-merge").is_err() {
        println!("No post-merge hook found");
    }

    if fs::remove_file(".git/hooks/pre-commit").is_err() {
        println!("No pre-commit hook found");
    }

    println!("Omit has been removed from this repository");
}
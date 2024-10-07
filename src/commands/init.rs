use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
pub fn run_init() {
    let git_dir = PathBuf::from(".git");

    if !git_dir.exists() {
        eprintln!("No git directory found, omit is meant to be used in a git repository");
        return;
    }

    let pull_hook = PathBuf::from(".git/hooks/post-merge");

    std::fs::write(&pull_hook, "#!/bin/sh\n\nomit pull").expect("Unable to write pull hook");

    let commit_hook = PathBuf::from(".git/hooks/pre-commit");

    std::fs::write(&commit_hook, "#!/bin/sh\n\nomit commit").expect("Unable to write commit hook");

    std::fs::set_permissions(&pull_hook, std::fs::Permissions::from_mode(0o755)).expect("Unable to set pull hook permissions");
    std::fs::set_permissions(&commit_hook, std::fs::Permissions::from_mode(0o755)).expect("Unable to set commit hook permissions");
}

// #[cfg(windows)]
// pub fn run_init() {
//     let git_dir = PathBuf::from(".git");

//     if !git_dir.exists() {
//         eprintln!("No git directory found, omit is meant to be used in a git repository");
//         return;
//     }

//     let pull_hook = PathBuf::from(".git/hooks/post-merge");

//     std::fs::write(&pull_hook, "#!/bin/sh\n\nomit pull").expect("Unable to write pull hook");

//     let commit_hook = PathBuf::from(".git/hooks/pre-commit");

//     std::fs::write(&commit_hook, "#!/bin/sh\n\nomit commit").expect("Unable to write commit hook");
// }
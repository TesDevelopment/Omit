use std::{fs::{read_to_string, OpenOptions}, io::Write, path::PathBuf};

use crate::components::linker::read_linker;

pub fn run_ensure() {
    if !PathBuf::from(".gitignore").exists() {
        std::fs::write(".gitignore", ".omit/.omit_key").expect("Unable to write to .gitignore");
    }

    let linker = read_linker().expect("Unable to read linker file");
    let mut gitignore = OpenOptions::new()
        .append(true)
        .open(".gitignore")
        .unwrap();
    
    // Compile a buffer to append later so we avoid weird blocking errors
    let mut write_buffer = "".to_owned();

    let ignore_contents = read_to_string(".gitignore").expect("Unable to read .gitignore");
    let to_strip = std::env::current_dir().unwrap();

    if !ignore_contents.contains(".omit/.omit_key") {
        write_buffer.push_str("\n.omit/.omit_key");
    }

    for linked_object in linker.linked_objects {
        if ignore_contents.contains(&linked_object.path) {
            continue;
        }

        write_buffer.push_str("\n");

        let mut fnl = linked_object.path;

        if fnl.contains(&to_strip.to_str().unwrap().to_string()) {
            fnl = fnl.strip_prefix(to_strip.to_str().unwrap()).unwrap().to_string();
        }

        write_buffer.push_str(fnl.as_str());
    }

    println!("Writing to secrets .gitignore");
    gitignore.write_all(write_buffer.as_bytes()).expect("Unable to write to .gitignore");
}
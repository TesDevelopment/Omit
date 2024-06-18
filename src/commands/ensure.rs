use std::{fs::OpenOptions, io::Write, path::PathBuf};

use crate::components::linker::{read_linker, read_linker_raw};

pub fn run_ensure() {
    if !PathBuf::from(".gitignore").exists() {
        std::fs::write(".gitignore", ".omit/.omit_key").expect("Unable to write to .gitignore");
    }

    let linker = read_linker().expect("Unable to read linker file");

    let linker_contents = read_linker_raw();

    let mut gitignore = OpenOptions::new()
        .append(true)
        .open(".gitignore")
        .unwrap();
    
    // Compile a buffer to append later so we avoid weird blocking errors
    let mut write_buffer = "".to_owned();

    for linked_object in linker.linked_objects {
        if linker_contents.contains(&linked_object.path) {
            continue;
        }

        write_buffer.push_str(&linked_object.path);
    }

    gitignore.write_all(write_buffer.as_bytes()).expect("Unable to write to .gitignore");
}
use serde::{Deserialize, Serialize};

// Technically you can parse json without a struct but a struct makes validation a lot more streamlined

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkerPage {
    pub linked_objects: Vec<LinkedObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkedObject {
    pub identifier: String,
    pub path: String,
    pub signiture: String,
}

impl LinkedObject {
    pub fn new(identifier: String, path: String,signiture: String) -> LinkedObject {
        LinkedObject {
            identifier,
            path,
            signiture
        }
    }

}
pub fn write_linker(linker: LinkerPage) -> Result<(), std::io::Error> {
    let linker_file = serde_json::to_string(&linker)?;
    std::fs::write(".omit/linker.json", linker_file)
}

pub fn read_linker() -> Result<LinkerPage, std::io::Error> {
    let linker_file = std::fs::read_to_string(".omit/linker.json");

    match linker_file {
        Err(_) => {
            let linker = LinkerPage {
                linked_objects: Vec::new(),
            };
            write_linker(linker.clone())?;
            return Ok(linker);
        }
        file => {
            let linker: LinkerPage = serde_json::from_str(&file.unwrap())?;
            Ok(linker)
        }
    }

    
}

pub fn read_linker_raw() -> String {
    std::fs::read_to_string(".omit/linker.json").unwrap()
}

#[cfg(tests)]
mod tests {

    #[test]
    pub fn test_read_linker() {
        let linker = read_linker().unwrap();
        assert_eq!(linker.linked_objects.len(), 1);
    }
}
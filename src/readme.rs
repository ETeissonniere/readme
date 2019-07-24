use std::env;

#[derive(Debug)]
pub struct Readme {
    name: String,
}

impl Readme {
    pub fn new() -> Self {
        let current_dir = env::current_dir().unwrap();
        let name = current_dir.file_name().unwrap();
        let name_str = name.to_str().unwrap();

        Readme {
            name: name_str.to_string(),
        }
    }

    pub fn survey(&mut self) {
        // Name
        // Description
        // Documentation
        // Image
        // Dependencies
        // Build
        // Test
        // Install
        // Usage example
    }

    pub fn save(&self) {
        // At the moment only print it, later save to README.md
        println!("{:#?}", self);
    }
}

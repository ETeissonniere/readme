extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Checkboxes, Confirmation, Editor, Input};
use std::env;

#[derive(Debug)]
struct Link {
    text: String,
    url: String,
}

#[derive(Debug)]
struct Commands {
    deps: String,
    build: String,
    test: String,
    install: String,
}

#[derive(Debug)]
pub struct Readme {
    name: String,
    description: String,
    image: Link,
    commands: Commands,
    usage: String,
}

impl Readme {
    pub fn new() -> Self {
        let current_dir = env::current_dir().unwrap();
        let name = current_dir.file_name().unwrap();
        let name_str = name.to_str().unwrap();

        Readme {
            name: name_str.to_string(),
            description: "".to_string(),
            image: Link {
                text: "screenshot".to_string(),
                url: "".to_string(),
            },
            commands: Commands {
                deps: "".to_string(),
                build: "".to_string(),
                test: "".to_string(),
                install: "".to_string(),
            },
            usage: "".to_string(),
        }
    }

    pub fn survey(&mut self) {
        self.name = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project's name")
            .default(self.name.clone())
            .interact()
            .unwrap();

        if Confirmation::new()
            .with_text("Would you like to add a description?")
            .interact()
            .unwrap()
        {
            if let Some(desc) = Editor::new().edit("").unwrap() {
                self.description = desc;
            }
        }

        if Confirmation::new()
            .with_text("Would you like to add an image?")
            .interact()
            .unwrap()
        {
            self.image.text = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Image's alt text")
                .default(self.image.text.clone())
                .interact()
                .unwrap();

            self.image.url = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Image's URL")
                .default(self.image.url.clone())
                .interact()
                .unwrap();
        }

        let readme_commands = &[
            "Command to install the dependencies",
            "Command to build the project",
            "Command to run the units tests",
            "Command to install the project",
        ];
        let commands_to_prompt_for = Checkboxes::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose the commands you want to add")
            .items(&readme_commands[..])
            .interact()
            .unwrap();

        if !commands_to_prompt_for.is_empty() {
            for command in commands_to_prompt_for {
                match command {
                    0 => {
                        self.commands.deps = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt(readme_commands[command])
                            .default(self.commands.deps.clone())
                            .interact()
                            .unwrap()
                    }
                    1 => {
                        self.commands.build = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt(readme_commands[command])
                            .default(self.commands.build.clone())
                            .interact()
                            .unwrap()
                    }
                    2 => {
                        self.commands.test = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt(readme_commands[command])
                            .default(self.commands.test.clone())
                            .interact()
                            .unwrap()
                    }
                    3 => {
                        self.commands.install = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt(readme_commands[command])
                            .default(self.commands.install.clone())
                            .interact()
                            .unwrap()
                    }
                    _ => panic!("unknown selection"),
                }
            }
        }

        if Confirmation::new()
            .with_text("Would you like to add a usage example?")
            .interact()
            .unwrap()
        {
            if let Some(us) = Editor::new().edit("").unwrap() {
                self.usage = us;
            }
        }
    }

    pub fn save(&self) {
        // At the moment only print it, later save to README.md
        println!("{:#?}", self);
    }
}

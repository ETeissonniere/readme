extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Checkboxes, Confirmation, Editor, Input};
use std::{env, fmt};

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

impl Commands {
    fn is_empty(&self) -> bool {
        return self.deps.is_empty()
            && self.build.is_empty()
            && self.test.is_empty()
            && self.install.is_empty();
    }
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
            name: String::from(name_str),
            description: String::from(""),
            image: Link {
                text: String::from("screenshot"),
                url: String::from(""),
            },
            commands: Commands {
                deps: String::from(""),
                build: String::from(""),
                test: String::from(""),
                install: String::from(""),
            },
            usage: String::from(""),
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
        let mut readme = String::new();
        readme.push_str(&fmt::format(format_args!("# {}\n", self.name)));

        if !self.description.is_empty() {
            readme.push_str(&self.description);
            readme.push_str("\n")
        }

        if !self.image.url.is_empty() {
            readme.push_str(&fmt::format(format_args!(
                "\n![{}]({})\n",
                self.image.text, self.image.url
            )));
        }

        if !self.commands.is_empty() {
            readme.push_str("\n# Development\n\n");

            if !self.commands.deps.is_empty() {
                readme.push_str("## Dependencies\n");
                readme.push_str(&fmt::format(format_args!(
                    "```\n{}\n```\n\n",
                    self.commands.deps
                )));
            }

            if !self.commands.build.is_empty() {
                readme.push_str("## Building\n");
                readme.push_str(&fmt::format(format_args!(
                    "```\n{}\n```\n\n",
                    self.commands.build
                )));
            }

            if !self.commands.test.is_empty() {
                readme.push_str("## Testing\n");
                readme.push_str(&fmt::format(format_args!(
                    "```\n{}\n```\n\n",
                    self.commands.test
                )));
            }

            if !self.commands.install.is_empty() {
                readme.push_str("## Installing\n");
                readme.push_str(&fmt::format(format_args!(
                    "```\n{}\n```\n\n",
                    self.commands.install
                )));
            }
        }

        if !self.usage.is_empty() {
            readme.push_str("# Usage\n");
            readme.push_str(&fmt::format(format_args!("```\n{}\n```\n", self.usage)));
        }

        // At the moment only print it, later save to README.md
        println!("{}", readme);
    }
}

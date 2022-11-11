use std::error::Error;
use crate::ExecutableCommand;
use clap::Parser;
use inquire::{Editor, Select, Text};

#[derive(Parser)]
#[clap(name = "reportbug", about = "An interactive prompt to automatically report bugs to the Mangrove team, to help them get fixed faster", version, author)]
pub struct ReportBugCommand {}

impl ExecutableCommand for ReportBugCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        println!("Welcome to the Mangrove bug reporting tool.");
        println!("This is a tool designed to accelerate the process of reporting a bug by partially completing the triage process right now.");

        let bug_types = vec!["Command-line interface bug", "libmangrove issue", "Critical security issue", "Generic bug report (I don't know)"];
        let bug_type = Select::new("What type of bug would you like to report?", bug_types).prompt();

        let reportee_name = Text::new("What is your name? Pseudonyms are okay.").prompt();
        let reportee_email = Text::new("What is your email? Updates on your report will be sent here.").prompt();

        let priority_levels = vec!["0 - Affects every single user of mangrove.", "1 - Affects the large majority of the mangrove userbase.", "2 - Affects a high quantity of mangrove users.", "3 - Affects a specific group of users and needs to be fixed", "4 - Low-priority issue that only affects a couple of users", "5 - Has no effect on any users"];
        let priority = Select::new("What is the priority of this issue? How many users are affected?", priority_levels).prompt();

        let subject = Text::new("Write a short, one-sentence description of the bug, like an email subject.").prompt();
        let mut message;
        loop {
            message = Editor::new("Write a longer description of the bug including as much detail as possible, like an email message:")
                .with_formatter(&|submission| {
                    let char_count = submission.chars().count();
                    if char_count <= 20 {
                        submission.into()
                    } else {
                        let mut substr: String = submission.chars().take(17).collect();
                        substr.push_str("...");
                        substr
                    }
                })
                .prompt()?;
            if message != "" {
                break;
            }
            println!("A description is required");
        }

        println!("Hang on while we figure out where to send your report to...");
        Ok(())
    }
}
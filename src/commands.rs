use std::{
    error::Error,
    io::{stdout, Write},
    process::Command,
};

use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CoprItem {
    description: String,
    full_name: String,
    contact: Option<String>,
    homepage: Option<String>,
    instructions: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CoprResponses {
    items: Vec<CoprItem>,
}

pub(crate) fn install(project: &str) -> Result<(), Box<dyn Error>> {
    let name = project
        .split('/')
        .nth(1)
        .ok_or("use format `copr_username/copr_projectname` to reference copr project")?;

    Command::new("sudo")
        .args(["dnf", "copr", "enable", project])
        .status()?;

    Command::new("sudo")
        .args(["dnf", "install", name])
        .status()?;

    Ok(())
}

pub(crate) fn search(query: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://copr.fedorainfracloud.org/api_3/project/search?query={query}");

    let projects = ureq::get(&url).call()?.into_json::<CoprResponses>()?;

    if projects.items.is_empty() {
        return Err("No matches found.".into());
    }

    let na = "N/A";

    let search_results = projects
        .items
        .iter()
        .fold(String::new(), |mut init, project| {
            let CoprItem {
                description,
                full_name,
                contact,
                homepage,
                instructions,
            } = project;

            let title = format!(
                "{} {}",
                "(copr)".bold().bright_blue(),
                full_name.trim().bold().bright_green()
            );

            let description = {
                let header = "\n\nDescription:".bold().underline();

                let desc = description.trim();

                if desc.is_empty() {
                    format!("{header} {na}")
                } else {
                    let instructions = instructions.trim();

                    if instructions.is_empty() {
                        format!("{header}\n{desc}")
                    } else {
                        format!(
                            "{header}\n{desc}\n\n{}\n{instructions}",
                            "Instruction:".bold().underline()
                        )
                    }
                }
            };

            let support = {
                // Support
                //     Contact: {contact}
                //     Homepage: {homepage}
                let header = "\n\nSupport:".bold().underline();

                match (contact, homepage) {
                    (None, None) => format!("{header} {na}\n"),
                    (None, Some(h)) => format!("{header}\n  Homepage - {h}\n"),
                    (Some(c), None) => format!("{header}\n  Contact - {c}\n"),
                    (Some(c), Some(h)) => format!("{header}\n  Homepage - {h}\n  Contact - {c}\n"),
                }
            };

            let temp = format!("{title}{description}{support}\n");

            init.push_str(&temp);

            init
        });

    writeln!(&mut stdout(), "{}", search_results)?;

    Ok(())
}

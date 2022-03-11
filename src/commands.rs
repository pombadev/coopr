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
                let desc = description.trim();
                let instructions = instructions.trim();

                format!(
                    "{} {}{}",
                    "\n\nDescription".bold().underline(),
                    if desc.is_empty() {
                        na.to_string()
                    } else {
                        "\n".to_string() + desc
                    },
                    if instructions.is_empty() {
                        "".to_string()
                    } else {
                        format!("\n\n{}\n{instructions}", "Instruction".bold().underline())
                    }
                )
            };

            let support = {
                // Support
                //     Contact: {contact}
                //     Homepage: {homepage}
                //     Project: https://copr.fedorainfracloud.org/coprs/<>
                let header_with_default_link = format!(
                    "\n\n{}\n  Project - https://copr.fedorainfracloud.org/coprs/{full_name}\n",
                    "Support".bold().underline()
                );

                match (contact, homepage) {
                    (None, None) => header_with_default_link,
                    (None, Some(h)) => format!("{header_with_default_link}  Homepage - {h}\n"),
                    (Some(c), None) => format!("{header_with_default_link}  Contact - {c}\n"),
                    (Some(c), Some(h)) => {
                        format!("{header_with_default_link}  Homepage - {h}\n  Contact - {c}\n")
                    }
                }
            };

            let temp = format!("{title}{description}{support}\n");

            init.push_str(&temp);

            init
        });

    writeln!(&mut stdout(), "{}", search_results)?;

    Ok(())
}

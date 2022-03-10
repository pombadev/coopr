use std::{env, error::Error};

#[path = "commands.rs"]
mod commands;

macro_rules! usage {
    () => {
        "{app} - {description}

USAGE:
    {app} [FLAG]
    {app} [OPTION] <QUERY>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --install <QUERY>   Install a package on your system
    -s, --search <QUERY>    Search package details for the given string"
    };
}

enum App {
    Search(String),
    Install(String),
    Required(String),
    Help,
    Version,
    Invalid,
}

impl App {
    fn parse() -> Result<Self, Box<dyn Error>> {
        let app = match env::args().nth(1) {
            Some(cmd) => match &cmd[..] {
                "--search" | "-s" => {
                    if let Some(query) = env::args().nth(2) {
                        Self::Search(query)
                    } else {
                        Self::Required("search".into())
                    }
                }

                "--install" | "-i" => {
                    if let Some(query) = env::args().nth(2) {
                        Self::Install(query)
                    } else {
                        Self::Required("install".into())
                    }
                }

                "--help" | "-h" => Self::Help,

                "--version" | "-v" => Self::Version,

                _ => Self::Invalid,
            },
            None => Self::Invalid,
        };

        Ok(app)
    }
}

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    match App::parse()? {
        App::Search(query) => commands::search(&query)?,
        App::Install(query) => commands::install(&query)?,
        App::Required(action) => return Err(format!("`--{action}` requires a parameter").into()),
        App::Help => {
            println!(
                usage!(),
                app = env!("CARGO_PKG_NAME"),
                description = env!("CARGO_PKG_DESCRIPTION")
            )
        }
        App::Version => println!("v{}", env!("CARGO_PKG_VERSION")),
        App::Invalid => {
            return Err("invalid option, pass `--help` to see all available commands".into())
        }
    }

    Ok(())
}

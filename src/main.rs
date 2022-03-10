#![cfg(target_os = "linux")]

use std::{error::Error, process::exit};

use colored::Colorize;

mod app;

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = app::run() {
        eprintln!("{}: {err}", "Error".bold().red());
        exit(1);
    }

    Ok(())
}

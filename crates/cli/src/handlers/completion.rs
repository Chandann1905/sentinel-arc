use crate::cli::Cli;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;

pub fn handle(shell: Shell) {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "sentinel", &mut io::stdout());
}

mod cli;
mod handlers;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => handlers::init::handle().await,
        Commands::Doctor => handlers::doctor::handle().await,
        Commands::Scan { path } => handlers::scan::handle(path).await,
        Commands::Search {
            query,
            node_type,
            limit,
            json,
        } => handlers::search::handle(query, node_type.clone(), *limit, *json).await,
        Commands::Graph { query } => handlers::graph::handle(query).await,
        Commands::Context { intent, json } => handlers::context::handle(intent, *json).await,
        Commands::Validate => {
            let code = handlers::validate::handle().await?;
            std::process::exit(code);
        }
        Commands::RebuildIndex => handlers::rebuild_index::handle().await,
        Commands::Stats => handlers::stats::handle().await,
        Commands::Version => {
            handlers::version::handle();
            Ok(())
        }
        Commands::Completion { shell } => {
            handlers::completion::handle(*shell);
            Ok(())
        }
    }
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sentinel")]
#[command(version, about = "Sentinel Arc CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Sentinel Arc workspace
    Init,

    /// Verify environment and setup
    Doctor,

    /// Scan directory and populate knowledge graph
    Scan {
        /// The path to scan
        #[arg(default_value = ".")]
        path: String,
    },

    /// Search the knowledge graph
    Search {
        /// The query string
        query: String,

        /// Filter by node type (string)
        #[arg(short, long)]
        node_type: Option<String>,

        /// Maximum number of results
        #[arg(short, long, default_value_t = 10)]
        limit: usize,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Visualize graph relationships
    Graph {
        /// The Node ID or Title to center the graph around
        query: String,
    },

    /// Generate an LLM context package
    Context {
        /// The natural language intent
        intent: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Validate the project graph for drift and consistency
    Validate,

    /// Generate a chronological timeline
    Timeline {
        /// Optional Node ID to filter by feature/module/roadmap
        #[arg(short, long)]
        node_id: Option<String>,

        /// Show only Architecture Decision Records
        #[arg(long, conflicts_with = "node_id")]
        decisions: bool,
    },

    /// Run the Model Context Protocol (MCP) JSON-RPC server
    Mcp,

    /// Rebuild the full text search index from the current database
    RebuildIndex,

    /// Display workspace statistics
    Stats,

    /// Display version and build information
    Version,

    /// Generate shell completions
    Completion {
        /// The shell to generate the script for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

use anyhow::{Context, Result};
use console::style;
use petgraph::Direction;
use sentinel_arc_core::types::ids::NodeId;
use sentinel_arc_graph::engine::GraphEngine;
use sentinel_arc_graph::projection::GraphProjection;
use sentinel_arc_knowledge::database::Database;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use std::collections::HashSet;
use std::path::Path;


static TREE_EDGE: &str = "├── ";
static TREE_CORNER: &str = "└── ";
static TREE_PIPE: &str = "│   ";
static TREE_BLANK: &str = "    ";

pub async fn handle(query: &str) -> Result<()> {
    let db_path = Path::new(".sentinel/knowledge.db");
    let db = Database::init(db_path)
        .await
        .context("Failed to initialize database")?;
    let knowledge = KnowledgeEngine::new(&db);

    let proj = GraphEngine::build_projection(&knowledge).await?;

    let node_id = match knowledge.get_node(&NodeId::from_string(query)).await {
        Ok(n) => n.id,
        Err(_) => {
            // Search for title
            let results = knowledge.search(query)?;
            if let Some(hit) = results.hits.into_iter().next() {
                NodeId::from_string(hit.entity_id)
            } else {
                println!("Node not found for query: '{}'", query);
                return Ok(());
            }
        }
    };

    let node = knowledge.get_node(&node_id).await?;

    println!("\n{}", style(format!("{} ({:?})", node.title, node.node_type)).bold().cyan());
    println!("{}", style(node.id.to_string()).dim());

    println!("\n{}", style("Dependencies (Downstream)").yellow().bold());
    let mut visited = HashSet::new();
    print_tree(&node_id, &knowledge, &proj, 0, String::new(), true, Direction::Outgoing, &mut visited).await?;

    println!("\n{}", style("Impact (Upstream)").magenta().bold());
    let mut visited2 = HashSet::new();
    print_tree(&node_id, &knowledge, &proj, 0, String::new(), true, Direction::Incoming, &mut visited2).await?;

    Ok(())
}

use async_recursion::async_recursion;

#[allow(clippy::too_many_arguments)]
#[async_recursion]
async fn print_tree(
    node_id: &NodeId,
    knowledge: &KnowledgeEngine,
    proj: &GraphProjection,
    depth: usize,
    prefix: String,
    is_last: bool,
    direction: Direction,
    visited: &mut HashSet<NodeId>,
) -> Result<()> {
    if depth > 0 {
        if !visited.insert(node_id.clone()) {
            println!("{}{} {}", prefix, if is_last { TREE_CORNER } else { TREE_EDGE }, style("[Circular Reference]").red());
            return Ok(());
        }

        let node = knowledge.get_node(node_id).await?;
        let connector = if is_last { TREE_CORNER } else { TREE_EDGE };
        println!("{}{} {} {}", prefix, connector, style(node.title).green(), style(format!("({:?})", node.node_type)).dim());
    } else {
        visited.insert(node_id.clone());
    }

    let children = match direction {
        Direction::Outgoing => proj.get_descendants(node_id),
        Direction::Incoming => proj.get_ancestors(node_id),
    };

    for (i, child_id) in children.iter().enumerate() {
        let is_last_child = i == children.len() - 1;
        let new_prefix = if depth == 0 {
            String::new()
        } else {
            format!("{}{}", prefix, if is_last { TREE_BLANK } else { TREE_PIPE })
        };
        
        print_tree(child_id, knowledge, proj, depth + 1, new_prefix, is_last_child, direction, visited).await?;
    }

    if depth > 0 {
        visited.remove(node_id);
    }

    Ok(())
}

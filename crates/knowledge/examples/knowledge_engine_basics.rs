use sentinel_arc_core::domain::node::Node;
use sentinel_arc_core::domain::relationship::Relationship;
use sentinel_arc_core::types::node_type::NodeType;
use sentinel_arc_core::types::relationship_type::RelationshipType;
use sentinel_arc_knowledge::database::Database;
use sentinel_arc_knowledge::engine::knowledge_engine::KnowledgeEngine;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize an in-memory database and run migrations
    let db = Database::init_with_path(Path::new(":memory:")).await?;

    // 2. Initialize the Knowledge Engine Facade
    let engine = KnowledgeEngine::new(&db);

    // 3. Create a Source Node
    println!("Creating Source Node...");
    let mut source = Node::new(NodeType::Feature, "Wallet Module");
    source.description = "Core wallet logic.".to_string();
    source.set_tags(vec!["backend".to_string()]);

    let (source_node, _event) = engine.create_node(source).await?;
    println!("Created Node: {} ({})", source_node.title, source_node.id);

    // 4. Create a Target Node
    println!("\nCreating Target Node...");
    let mut target = Node::new(NodeType::Task, "Implement Wallet API");
    target.description = "Build the REST API.".to_string();

    let (target_node, _event) = engine.create_node(target).await?;
    println!("Created Node: {} ({})", target_node.title, target_node.id);

    // 5. Create a Relationship between them
    println!("\nCreating Relationship...");
    let rel = Relationship::new(
        source_node.id.clone(),
        target_node.id.clone(),
        RelationshipType::DependsOn,
    );
    let (relationship, _event) = engine.create_relationship(rel).await?;
    println!(
        "Created Relationship: {} --[{:?}]--> {}",
        relationship.source_node, relationship.relationship_type, relationship.target_node
    );

    // 6. Fetch the Event History
    println!("\nFetching Temporal Audit Trail for Source Node...");
    let history = engine.get_history(source_node.id.as_str()).await?;
    for event in history {
        println!(
            "Event Logged: {:?} at {}",
            event.event_type, event.timestamp
        );
    }

    Ok(())
}

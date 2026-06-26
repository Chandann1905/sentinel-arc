use sentinel_arc_core::types::ids::NodeId;
use sentinel_arc_graph::projection::GraphProjection;
use std::collections::HashSet;

/// Expands the graph topology up to a given depth starting from the given root nodes.
/// Returns a set of related Node IDs (excluding the roots).
pub fn expand_topology(
    projection: &GraphProjection,
    roots: &[NodeId],
    max_depth: u32,
) -> HashSet<NodeId> {
    let mut related = HashSet::new();

    for root in roots {
        // Collect dependencies (things the root depends on)
        let deps = projection.find_dependencies(root, Some(max_depth as usize));
        for id in deps {
            related.insert(id);
        }

        // Collect impact (things that depend on the root)
        let impact = projection.find_impact(root, Some(max_depth as usize));
        for id in impact {
            related.insert(id);
        }
    }

    // Exclude the roots themselves from the related set (in case of cycles)
    for root in roots {
        related.remove(root);
    }

    related
}

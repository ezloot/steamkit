use std::collections::HashMap;

use crate::{Node, NodeEdge};
use petgraph::prelude::*;

fn get_children_by<P: FnMut(&NodeIndex) -> bool>(
    graph: &Graph<Node, NodeEdge>,
    parent_idx: NodeIndex,
    predicate: P,
) -> Vec<NodeIndex> {
    graph
        .neighbors_directed(parent_idx, Direction::Outgoing)
        .filter(predicate)
        .collect()
}

// fn get_module_types(
//     graph: &Graph<Node, NodeEdge>,
//     node_idx: NodeIndex,
// ) -> HashMap<String, NodeIndex> {
//     let mut m = HashMap::new();

//     let imports = get_children(graph, node_idx, |idx| graph[*idx].is_import());
//     for import in imports {
//         m.extend(get_children_by(graph, import))
//     }
// }

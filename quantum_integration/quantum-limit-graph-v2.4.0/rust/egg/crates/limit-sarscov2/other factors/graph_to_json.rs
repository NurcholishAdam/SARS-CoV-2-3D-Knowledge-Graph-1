use petgraph::graph::DiGraph;
use crate::core::*;
use crate::serial::*;
use std::collections::HashSet;

pub fn export_graph_to_json(graph: &DiGraph<Node, Edge>) -> String {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut stages = HashSet::new();
    let mut total_quantum_correlation = 0.0;
    let mut edge_count = 0;

    // Convert nodes
    for node_idx in graph.node_indices() {
        let node = &graph[node_idx];
        nodes.push(SerializableNode::from(node));
        stages.insert(format!("{:?}", node.stage));
    }

    // Convert edges
    for edge_idx in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge_idx).unwrap();
        let edge = &graph[edge_idx];
        edges.push(SerializableEdge::from(edge));
        total_quantum_correlation += edge.quantum_entanglement;
        edge_count += 1;
    }

    let quantum_avg = if edge_count > 0 {
        total_quantum_correlation / edge_count as f32
    } else {
        0.0
    };

    let serializable_graph = SerializableGraph {
        nodes,
        edges,
        metadata: GraphMetadata {
            total_nodes: graph.node_count(),
            total_edges: graph.edge_count(),
            stages: stages.into_iter().collect(),
            quantum_correlation_average: quantum_avg,
            generated_at: chrono::Utc::now().to_rfc3339(),
        },
    };

    serde_json::to_string_pretty(&serializable_graph).unwrap()
}

pub fn export_graph_to_json_compact(graph: &DiGraph<Node, Edge>) -> String {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for node_idx in graph.node_indices() {
        let node = &graph[node_idx];
        nodes.push(SerializableNode::from(node));
    }

    for edge_idx in graph.edge_indices() {
        let edge = &graph[edge_idx];
        edges.push(SerializableEdge::from(edge));
    }

    serde_json::to_string(&serde_json::json!({
        "nodes": nodes,
        "edges": edges
    })).unwrap()
}

pub fn export_stage_filtered_json(graph: &DiGraph<Node, Edge>, stage: CorrelationStage) -> String {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Filter nodes by stage
    for node_idx in graph.node_indices() {
        let node = &graph[node_idx];
        if format!("{:?}", node.stage) == format!("{:?}", stage) {
            nodes.push(SerializableNode::from(node));
        }
    }

    // Filter edges by stage
    for edge_idx in graph.edge_indices() {
        let edge = &graph[edge_idx];
        if format!("{:?}", edge.stage) == format!("{:?}", stage) {
            edges.push(SerializableEdge::from(edge));
        }
    }

    serde_json::to_string_pretty(&serde_json::json!({
        "stage": format!("{:?}", stage),
        "nodes": nodes,
        "edges": edges
    })).unwrap()
}

//! SARS-CoV-2 Quantum Limit Graph - Enhanced Multi-Factor Correlation Library
//! 
//! This library provides a comprehensive framework for modeling SARS-CoV-2 correlations
//! across multiple stages with quantum entanglement factors.

pub mod core;
pub mod graph_cons;
pub mod serial;
pub mod graph_to_json;

pub use core::{
    Node, Edge, NodeType, CorrelationType, CorrelationStage,
    NodeMetadata,
};
pub use graph_cons::{build_quantum_limit_graph, QuantumLimitGraphBuilder};
pub use serial::{
    SerializableNode, SerializableEdge, SerializableGraph,
    SerializableMetadata, GraphMetadata,
};
pub use graph_to_json::{
    export_graph_to_json, export_graph_to_json_compact,
    export_stage_filtered_json,
};

use petgraph::graph::DiGraph;

/// Build and export the complete SARS-CoV-2 quantum graph to JSON
pub fn build_and_export() -> String {
    let graph = build_quantum_limit_graph();
    export_graph_to_json(&graph)
}

/// Build and export a stage-filtered graph
pub fn build_and_export_stage(stage: CorrelationStage) -> String {
    let graph = build_quantum_limit_graph();
    export_stage_filtered_json(&graph, stage)
}

/// Get graph statistics
pub fn get_graph_stats(graph: &DiGraph<Node, Edge>) -> GraphStats {
    let total_nodes = graph.node_count();
    let total_edges = graph.edge_count();
    
    let mut quantum_sum = 0.0;
    let mut correlation_sum = 0.0;
    let mut entanglement_sum = 0.0;
    
    for node_idx in graph.node_indices() {
        quantum_sum += graph[node_idx].quantum_weight;
    }
    
    for edge_idx in graph.edge_indices() {
        let edge = &graph[edge_idx];
        correlation_sum += edge.correlation_strength;
        entanglement_sum += edge.quantum_entanglement;
    }
    
    GraphStats {
        total_nodes,
        total_edges,
        avg_quantum_weight: if total_nodes > 0 { quantum_sum / total_nodes as f32 } else { 0.0 },
        avg_correlation: if total_edges > 0 { correlation_sum / total_edges as f32 } else { 0.0 },
        avg_entanglement: if total_edges > 0 { entanglement_sum / total_edges as f32 } else { 0.0 },
    }
}

#[derive(Debug, Clone)]
pub struct GraphStats {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub avg_quantum_weight: f32,
    pub avg_correlation: f32,
    pub avg_entanglement: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_construction() {
        let graph = build_quantum_limit_graph();
        assert!(graph.node_count() > 0, "Graph should have nodes");
        assert!(graph.edge_count() > 0, "Graph should have edges");
    }

    #[test]
    fn test_json_export() {
        let json = build_and_export();
        assert!(json.contains("nodes"), "JSON should contain nodes");
        assert!(json.contains("edges"), "JSON should contain edges");
        assert!(json.contains("metadata"), "JSON should contain metadata");
    }

    #[test]
    fn test_stage_filtering() {
        let json = build_and_export_stage(CorrelationStage::Stage1Direct);
        assert!(json.contains("Stage1Direct"), "Should filter by stage");
    }

    #[test]
    fn test_graph_stats() {
        let graph = build_quantum_limit_graph();
        let stats = get_graph_stats(&graph);
        assert!(stats.total_nodes > 0);
        assert!(stats.total_edges > 0);
        assert!(stats.avg_quantum_weight > 0.0);
        assert!(stats.avg_correlation > 0.0);
    }
}

use serde::{Serialize, Deserialize};
use crate::core::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableNode {
    pub id: String,
    pub numeric_id: usize,
    pub label: String,
    pub node_type: String,
    pub description: Option<String>,
    pub quantum_weight: f32,
    pub stage: String,
    pub metadata: SerializableMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableMetadata {
    pub evidence_count: usize,
    pub confidence_score: f32,
    pub last_updated: String,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableEdge {
    pub id: String,
    pub source: usize,
    pub target: usize,
    pub correlation_strength: f32,
    pub correlation_type: String,
    pub description: Option<String>,
    pub quantum_entanglement: f32,
    pub stage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableGraph {
    pub nodes: Vec<SerializableNode>,
    pub edges: Vec<SerializableEdge>,
    pub metadata: GraphMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMetadata {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub stages: Vec<String>,
    pub quantum_correlation_average: f32,
    pub generated_at: String,
}

impl From<&Node> for SerializableNode {
    fn from(node: &Node) -> Self {
        Self {
            id: node.id.to_string(),
            numeric_id: node.numeric_id,
            label: node.label.clone(),
            node_type: format!("{:?}", node.node_type),
            description: node.description.clone(),
            quantum_weight: node.quantum_weight,
            stage: format!("{:?}", node.stage),
            metadata: SerializableMetadata {
                evidence_count: node.metadata.evidence_count,
                confidence_score: node.metadata.confidence_score,
                last_updated: node.metadata.last_updated.clone(),
                sources: node.metadata.sources.clone(),
            },
        }
    }
}

impl From<&Edge> for SerializableEdge {
    fn from(edge: &Edge) -> Self {
        Self {
            id: edge.id.to_string(),
            source: edge.from,
            target: edge.to,
            correlation_strength: edge.correlation_strength,
            correlation_type: format!("{:?}", edge.correlation_type),
            description: edge.description.clone(),
            quantum_entanglement: edge.quantum_entanglement,
            stage: format!("{:?}", edge.stage),
        }
    }
}

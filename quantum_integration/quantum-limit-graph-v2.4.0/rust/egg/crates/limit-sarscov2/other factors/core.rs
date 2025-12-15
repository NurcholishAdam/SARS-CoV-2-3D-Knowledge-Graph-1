use serde::{Serialize, Deserialize};
use uuid::Uuid;

// Define node types with quantum correlation factors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    Central,
    Biological,
    Comorbidity,
    Coinfection,
    Socioeconomic,
    Environmental,
    Immunological,
    Genomic,
    Therapeutic,
    PublicHealth,
    QuantumFactor,
}

// Quantum correlation stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CorrelationStage {
    Stage1Direct,      // Direct biological correlation
    Stage2Indirect,    // Indirect through comorbidities
    Stage3Systemic,    // Systemic socioeconomic factors
    Stage4Environmental, // Environmental transmission
    Stage5Quantum,     // Quantum entanglement effects
}

// Node metadata with quantum properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub numeric_id: usize,
    pub label: String,
    pub node_type: NodeType,
    pub description: Option<String>,
    pub quantum_weight: f32,  // Quantum correlation weight
    pub stage: CorrelationStage,
    pub metadata: NodeMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub evidence_count: usize,
    pub confidence_score: f32,
    pub last_updated: String,
    pub sources: Vec<String>,
}

// Edge metadata with correlation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: Uuid,
    pub from: usize,
    pub to: usize,
    pub correlation_strength: f32, // 0.0 to 1.0
    pub correlation_type: CorrelationType,
    pub description: Option<String>,
    pub quantum_entanglement: f32, // Quantum correlation factor
    pub stage: CorrelationStage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CorrelationType {
    Causal,
    Correlative,
    Bidirectional,
    QuantumEntangled,
    Probabilistic,
}

impl Node {
    pub fn new(
        numeric_id: usize,
        label: String,
        node_type: NodeType,
        stage: CorrelationStage,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            numeric_id,
            label,
            node_type,
            description: None,
            quantum_weight: 0.5,
            stage,
            metadata: NodeMetadata {
                evidence_count: 0,
                confidence_score: 0.5,
                last_updated: chrono::Utc::now().to_rfc3339(),
                sources: Vec::new(),
            },
        }
    }

    pub fn with_description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }

    pub fn with_quantum_weight(mut self, weight: f32) -> Self {
        self.quantum_weight = weight;
        self
    }
}

impl Edge {
    pub fn new(
        from: usize,
        to: usize,
        correlation_strength: f32,
        correlation_type: CorrelationType,
        stage: CorrelationStage,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            correlation_strength,
            correlation_type,
            description: None,
            quantum_entanglement: 0.0,
            stage,
        }
    }

    pub fn with_description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }

    pub fn with_quantum_entanglement(mut self, entanglement: f32) -> Self {
        self.quantum_entanglement = entanglement;
        self
    }
}

use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use crate::core::*;

pub struct QuantumLimitGraphBuilder {
    graph: DiGraph<Node, Edge>,
    node_map: HashMap<String, NodeIndex>,
    node_counter: usize,
}

impl QuantumLimitGraphBuilder {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
            node_counter: 0,
        }
    }

    fn add_node_with_key(&mut self, key: String, node: Node) -> NodeIndex {
        let idx = self.graph.add_node(node);
        self.node_map.insert(key, idx);
        self.node_counter += 1;
        idx
    }

    pub fn build_quantum_limit_graph(mut self) -> DiGraph<Node, Edge> {
        // Stage 1: Central Node
        let quantum_limit = self.add_node_with_key(
            "QuantumLimitGraph".to_string(),
            Node::new(
                0,
                "QuantumLimitGraph".to_string(),
                NodeType::Central,
                CorrelationStage::Stage1Direct,
            )
            .with_description("Central SARS-CoV-2 Quantum Knowledge Graph Node".to_string())
            .with_quantum_weight(1.0),
        );

        // Stage 1: Direct Biological Factors
        let ace2 = self.add_node_with_key(
            "ACE2".to_string(),
            Node::new(
                1,
                "ACE2 Receptor".to_string(),
                NodeType::Biological,
                CorrelationStage::Stage1Direct,
            )
            .with_description("Host receptor facilitating viral entry via spike protein binding".to_string())
            .with_quantum_weight(0.95),
        );

        let spike_protein = self.add_node_with_key(
            "SpikeProtein".to_string(),
            Node::new(
                2,
                "Spike Protein".to_string(),
                NodeType::Biological,
                CorrelationStage::Stage1Direct,
            )
            .with_description("Viral surface protein enabling cell entry".to_string())
            .with_quantum_weight(0.93),
        );

        let viral_load = self.add_node_with_key(
            "ViralLoad".to_string(),
            Node::new(
                3,
                "Viral Load".to_string(),
                NodeType::Biological,
                CorrelationStage::Stage1Direct,
            )
            .with_description("Quantity of virus in respiratory tract".to_string())
            .with_quantum_weight(0.88),
        );

        // Stage 2: Indirect Factors - Comorbidities
        let diabetes = self.add_node_with_key(
            "Diabetes".to_string(),
            Node::new(
                4,
                "Diabetes".to_string(),
                NodeType::Comorbidity,
                CorrelationStage::Stage2Indirect,
            )
            .with_description("Metabolic disorder increasing COVID-19 severity".to_string())
            .with_quantum_weight(0.82),
        );

        let cardiovascular = self.add_node_with_key(
            "Cardiovascular".to_string(),
            Node::new(
                5,
                "Cardiovascular Disease".to_string(),
                NodeType::Comorbidity,
                CorrelationStage::Stage2Indirect,
            )
            .with_description("Heart conditions worsening COVID-19 outcomes".to_string())
            .with_quantum_weight(0.85),
        );

        let respiratory = self.add_node_with_key(
            "Respiratory".to_string(),
            Node::new(
                6,
                "Respiratory Disease".to_string(),
                NodeType::Comorbidity,
                CorrelationStage::Stage2Indirect,
            )
            .with_description("COPD, asthma increasing vulnerability".to_string())
            .with_quantum_weight(0.87),
        );

        let kidney_liver = self.add_node_with_key(
            "KidneyLiver".to_string(),
            Node::new(
                7,
                "Kidney/Liver Disease".to_string(),
                NodeType::Comorbidity,
                CorrelationStage::Stage2Indirect,
            )
            .with_description("Organ dysfunction affecting recovery".to_string())
            .with_quantum_weight(0.78),
        );

        // Stage 2: Coinfections
        let influenza = self.add_node_with_key(
            "Influenza".to_string(),
            Node::new(
                8,
                "Influenza Coinfection".to_string(),
                NodeType::Coinfection,
                CorrelationStage::Stage2Indirect,
            )
            .with_description("Concurrent flu infection worsening outcomes".to_string())
            .with_quantum_weight(0.75),
        );

        let rsv = self.add_node_with_key(
            "RSV".to_string(),
            Node::new(
                9,
                "RSV Coinfection".to_string(),
                NodeType::Coinfection,
                CorrelationStage::Stage2Indirect,
            )
            .with_description("Respiratory syncytial virus coinfection".to_string())
            .with_quantum_weight(0.72),
        );

        // Stage 3: Systemic Socioeconomic Factors
        let crowded_housing = self.add_node_with_key(
            "CrowdedHousing".to_string(),
            Node::new(
                10,
                "Crowded Housing".to_string(),
                NodeType::Socioeconomic,
                CorrelationStage::Stage3Systemic,
            )
            .with_description("High-density living increasing transmission".to_string())
            .with_quantum_weight(0.80),
        );

        let public_facing_jobs = self.add_node_with_key(
            "PublicFacingJobs".to_string(),
            Node::new(
                11,
                "Public-Facing Jobs".to_string(),
                NodeType::Socioeconomic,
                CorrelationStage::Stage3Systemic,
            )
            .with_description("Essential workers with high exposure".to_string())
            .with_quantum_weight(0.83),
        );

        let healthcare_access = self.add_node_with_key(
            "HealthcareAccess".to_string(),
            Node::new(
                12,
                "Healthcare Access".to_string(),
                NodeType::Socioeconomic,
                CorrelationStage::Stage3Systemic,
            )
            .with_description("Availability and quality of medical care".to_string())
            .with_quantum_weight(0.86),
        );

        // Stage 4: Environmental Factors
        let air_quality = self.add_node_with_key(
            "AirQuality".to_string(),
            Node::new(
                13,
                "Air Quality".to_string(),
                NodeType::Environmental,
                CorrelationStage::Stage4Environmental,
            )
            .with_description("Pollution levels affecting respiratory health".to_string())
            .with_quantum_weight(0.70),
        );

        let ventilation = self.add_node_with_key(
            "Ventilation".to_string(),
            Node::new(
                14,
                "Ventilation".to_string(),
                NodeType::Environmental,
                CorrelationStage::Stage4Environmental,
            )
            .with_description("Airflow reducing viral concentration".to_string())
            .with_quantum_weight(0.77),
        );

        let temperature_humidity = self.add_node_with_key(
            "TempHumidity".to_string(),
            Node::new(
                15,
                "Temperature/Humidity".to_string(),
                NodeType::Environmental,
                CorrelationStage::Stage4Environmental,
            )
            .with_description("Climate factors affecting viral survival".to_string())
            .with_quantum_weight(0.65),
        );

        // Stage 5: Quantum Factors
        let immune_response = self.add_node_with_key(
            "ImmuneResponse".to_string(),
            Node::new(
                16,
                "Immune Response".to_string(),
                NodeType::Immunological,
                CorrelationStage::Stage5Quantum,
            )
            .with_description("Quantum-entangled immune system dynamics".to_string())
            .with_quantum_weight(0.91),
        );

        let genomic_variants = self.add_node_with_key(
            "GenomicVariants".to_string(),
            Node::new(
                17,
                "Genomic Variants".to_string(),
                NodeType::Genomic,
                CorrelationStage::Stage5Quantum,
            )
            .with_description("Viral mutations with quantum correlation patterns".to_string())
            .with_quantum_weight(0.89),
        );

        // Add edges with quantum entanglement
        self.add_quantum_edges(quantum_limit, ace2, spike_protein, viral_load,
                               diabetes, cardiovascular, respiratory, kidney_liver,
                               influenza, rsv, crowded_housing, public_facing_jobs,
                               healthcare_access, air_quality, ventilation,
                               temperature_humidity, immune_response, genomic_variants);

        self.graph
    }

    fn add_quantum_edges(
        &mut self,
        quantum_limit: NodeIndex,
        ace2: NodeIndex,
        spike_protein: NodeIndex,
        viral_load: NodeIndex,
        diabetes: NodeIndex,
        cardiovascular: NodeIndex,
        respiratory: NodeIndex,
        kidney_liver: NodeIndex,
        influenza: NodeIndex,
        rsv: NodeIndex,
        crowded_housing: NodeIndex,
        public_facing_jobs: NodeIndex,
        healthcare_access: NodeIndex,
        air_quality: NodeIndex,
        ventilation: NodeIndex,
        temperature_humidity: NodeIndex,
        immune_response: NodeIndex,
        genomic_variants: NodeIndex,
    ) {
        // Stage 1: Direct biological connections
        self.graph.add_edge(
            quantum_limit,
            ace2,
            Edge::new(0, 1, 0.95, CorrelationType::Causal, CorrelationStage::Stage1Direct)
                .with_description("ACE2 receptor enables viral entry".to_string())
                .with_quantum_entanglement(0.92),
        );

        self.graph.add_edge(
            quantum_limit,
            spike_protein,
            Edge::new(0, 2, 0.93, CorrelationType::Causal, CorrelationStage::Stage1Direct)
                .with_description("Spike protein binds to ACE2".to_string())
                .with_quantum_entanglement(0.90),
        );

        self.graph.add_edge(
            ace2,
            spike_protein,
            Edge::new(1, 2, 0.98, CorrelationType::Bidirectional, CorrelationStage::Stage1Direct)
                .with_description("Direct binding interaction".to_string())
                .with_quantum_entanglement(0.95),
        );

        self.graph.add_edge(
            spike_protein,
            viral_load,
            Edge::new(2, 3, 0.88, CorrelationType::Causal, CorrelationStage::Stage1Direct)
                .with_description("Spike efficiency affects viral replication".to_string())
                .with_quantum_entanglement(0.85),
        );

        // Stage 2: Comorbidity connections
        self.graph.add_edge(
            quantum_limit,
            diabetes,
            Edge::new(0, 4, 0.82, CorrelationType::Correlative, CorrelationStage::Stage2Indirect)
                .with_description("Diabetes increases COVID-19 severity".to_string())
                .with_quantum_entanglement(0.78),
        );

        self.graph.add_edge(
            quantum_limit,
            cardiovascular,
            Edge::new(0, 5, 0.85, CorrelationType::Correlative, CorrelationStage::Stage2Indirect)
                .with_description("Cardiovascular disease worsens outcomes".to_string())
                .with_quantum_entanglement(0.81),
        );

        self.graph.add_edge(
            quantum_limit,
            respiratory,
            Edge::new(0, 6, 0.87, CorrelationType::Correlative, CorrelationStage::Stage2Indirect)
                .with_description("Respiratory conditions increase vulnerability".to_string())
                .with_quantum_entanglement(0.83),
        );

        self.graph.add_edge(
            ace2,
            respiratory,
            Edge::new(1, 6, 0.79, CorrelationType::Bidirectional, CorrelationStage::Stage2Indirect)
                .with_description("ACE2 expression in respiratory tissue".to_string())
                .with_quantum_entanglement(0.75),
        );

        // Coinfection edges
        self.graph.add_edge(
            viral_load,
            influenza,
            Edge::new(3, 8, 0.75, CorrelationType::Correlative, CorrelationStage::Stage2Indirect)
                .with_description("Coinfection amplifies viral burden".to_string())
                .with_quantum_entanglement(0.70),
        );

        self.graph.add_edge(
            viral_load,
            rsv,
            Edge::new(3, 9, 0.72, CorrelationType::Correlative, CorrelationStage::Stage2Indirect)
                .with_description("RSV coinfection complicates recovery".to_string())
                .with_quantum_entanglement(0.68),
        );

        // Stage 3: Socioeconomic connections
        self.graph.add_edge(
            quantum_limit,
            crowded_housing,
            Edge::new(0, 10, 0.80, CorrelationType::Probabilistic, CorrelationStage::Stage3Systemic)
                .with_description("Crowding increases transmission probability".to_string())
                .with_quantum_entanglement(0.65),
        );

        self.graph.add_edge(
            quantum_limit,
            public_facing_jobs,
            Edge::new(0, 11, 0.83, CorrelationType::Probabilistic, CorrelationStage::Stage3Systemic)
                .with_description("Occupational exposure increases risk".to_string())
                .with_quantum_entanglement(0.68),
        );

        self.graph.add_edge(
            healthcare_access,
            cardiovascular,
            Edge::new(12, 5, 0.74, CorrelationType::Bidirectional, CorrelationStage::Stage3Systemic)
                .with_description("Healthcare access affects comorbidity management".to_string())
                .with_quantum_entanglement(0.60),
        );

        // Stage 4: Environmental connections
        self.graph.add_edge(
            air_quality,
            respiratory,
            Edge::new(13, 6, 0.76, CorrelationType::Causal, CorrelationStage::Stage4Environmental)
                .with_description("Poor air quality damages respiratory system".to_string())
                .with_quantum_entanglement(0.55),
        );

        self.graph.add_edge(
            ventilation,
            viral_load,
            Edge::new(14, 3, 0.77, CorrelationType::Causal, CorrelationStage::Stage4Environmental)
                .with_description("Ventilation reduces airborne viral concentration".to_string())
                .with_quantum_entanglement(0.58),
        );

        self.graph.add_edge(
            temperature_humidity,
            spike_protein,
            Edge::new(15, 2, 0.65, CorrelationType::Correlative, CorrelationStage::Stage4Environmental)
                .with_description("Climate affects viral stability".to_string())
                .with_quantum_entanglement(0.50),
        );

        // Stage 5: Quantum entangled connections
        self.graph.add_edge(
            quantum_limit,
            immune_response,
            Edge::new(0, 16, 0.91, CorrelationType::QuantumEntangled, CorrelationStage::Stage5Quantum)
                .with_description("Quantum-correlated immune dynamics".to_string())
                .with_quantum_entanglement(0.95),
        );

        self.graph.add_edge(
            immune_response,
            viral_load,
            Edge::new(16, 3, 0.89, CorrelationType::QuantumEntangled, CorrelationStage::Stage5Quantum)
                .with_description("Immune response modulates viral replication".to_string())
                .with_quantum_entanglement(0.92),
        );

        self.graph.add_edge(
            genomic_variants,
            spike_protein,
            Edge::new(17, 2, 0.94, CorrelationType::QuantumEntangled, CorrelationStage::Stage5Quantum)
                .with_description("Mutations alter spike protein structure".to_string())
                .with_quantum_entanglement(0.93),
        );

        self.graph.add_edge(
            genomic_variants,
            immune_response,
            Edge::new(17, 16, 0.87, CorrelationType::QuantumEntangled, CorrelationStage::Stage5Quantum)
                .with_description("Variants evade immune recognition".to_string())
                .with_quantum_entanglement(0.88),
        );
    }
}

pub fn build_quantum_limit_graph() -> DiGraph<Node, Edge> {
    QuantumLimitGraphBuilder::new().build_quantum_limit_graph()
}

# LIMIT-SARSCOV2: SARS-CoV-2 Multi-Intent Knowledge Graph

A specialized knowledge graph module for SARS-CoV-2 research integrated into Quantum LIMIT Graph. This module provides multi-intent graph capabilities with nodes representing different research domains (biology, immunology, variants, treatments, public health), causal/correlative edges, serendipity traces for hypothesis exploration, and rate-distortion curves for retrieval optimization.

## 🆕 Latest Enhancement: Multi-Stage Quantum Correlation Model

**NEW in v2.4.1**: Enhanced multi-factor correlation system with quantum entanglement modeling across 5 stages! See [`other factors/`](./other%20factors/) for the complete implementation.

### Enhanced Features
- **18 Correlated Nodes** across 5 stages (Direct Biological → Quantum Factors)
- **Quantum Entanglement Metrics** (0.50-0.95) for edge correlations
- **Interactive D3.js Visualization** with stage filtering and real-time statistics
- **Python Integration** with NetworkX for advanced analysis
- **Multiple Export Formats**: Full, compact, and stage-filtered JSON

**Quick Start**: See [`other factors/QUICK_START.md`](./other%20factors/QUICK_START.md)

## Features

### 🧬 Multi-Domain Nodes
- **Biology (Virology)**: Spike protein, viral mechanisms, ACE2 receptor
- **Immunology**: Antibody response, T-cell immunity, immune dynamics
- **Variants (Genomics)**: Omicron, Delta, mutations, genomic variants
- **Treatments**: Paxlovid, Remdesivir, monoclonal antibodies
- **Public Health**: Mask mandates, ventilation, policies
- **🆕 Comorbidities**: Diabetes, cardiovascular, respiratory diseases
- **🆕 Coinfections**: Influenza, RSV interactions
- **🆕 Socioeconomic**: Housing, occupation, healthcare access
- **🆕 Environmental**: Air quality, ventilation, climate factors

### 🔗 Causal & Correlative Edges
- **Causal**: mutation → immune escape, variant → transmissibility
- **Correlative**: treatment → reduced hospitalization, policy → transmission reduction
- Cross-domain relationships with evidence tracking

### 🎯 Serendipity Traces
Visualize agent exploration of multiple hypotheses:
- "mutation X increases transmissibility" vs. "mutation X affects vaccine efficacy"
- Track branching factor, diversity score, cross-domain jumps
- Measure exploration depth and confidence

### 📊 Rate-Distortion Curves
Quantify trade-offs between:
- **Rate**: Retrieval coverage (all possible evidence)
- **Distortion**: Noise or redundancy
- Find optimal operating points (knee of curve)

### 🔍 Multi-Intent Queries
Decompose complex questions into domain-specific sub-intents:
- "How does Omicron affect vaccine efficacy?" → Genomics + Immunology
- "What treatments work for BA.5?" → Treatment + Genomics + Virology

## Architecture

```
limit-sarscov2/
├── src/
│   ├── lib.rs                    # Main exports
│   ├── domain.rs                 # Research domains & base graph
│   ├── nodes.rs                  # Domain-specific node types
│   ├── edges.rs                  # Causal/correlative edges
│   ├── multi_intent_graph.rs     # Multi-intent graph structure
│   ├── serendipity_trace.rs      # Exploration traces
│   ├── queries.rs                # Multi-intent query decomposition
│   ├── retrieval.rs              # Corpus retrieval backend
│   ├── metrics.rs                # Domain coverage & serendipity
│   ├── rd.rs                     # Rate-distortion curves
│   ├── governance.rs             # Evidence thresholds & merge rules
│   ├── api.rs                    # HTTP API (Axum)
│   └── main.rs                   # Standalone server
├── other factors/                # 🆕 Multi-stage quantum correlation model
│   ├── core.rs                   # Enhanced node/edge structures
│   ├── graph_cons.rs             # 18-node graph builder (5 stages)
│   ├── serial.rs                 # Serialization framework
│   ├── graph_to_json.rs          # JSON export (full/compact/filtered)
│   ├── lib.rs                    # Library API
│   ├── main.rs                   # CLI tool
│   ├── frontend.html             # Interactive D3.js visualization
│   ├── python_integration.py     # Python wrapper with NetworkX
│   ├── README.md                 # Complete documentation
│   ├── QUICK_START.md            # 5-minute getting started
│   ├── ARCHITECTURE.txt          # Architecture overview
│   └── validate_implementation.py # Validation script
├── examples/
│   └── multi_intent_demo.rs      # Comprehensive demo
├── python_integration.py         # Python bindings
└── README.md                     # This file
```

## 🌟 Multi-Stage Quantum Correlation Model

The enhanced `other factors/` module extends SARS-CoV-2 knowledge modeling with a comprehensive 5-stage quantum correlation framework:

### Stage Architecture

**Stage 1: Direct Biological (4 nodes)**
- QuantumLimitGraph (Central) - Quantum Weight: 1.0
- ACE2 Receptor - QW: 0.95, Entanglement: 0.92
- Spike Protein - QW: 0.93, Entanglement: 0.90
- Viral Load - QW: 0.88, Entanglement: 0.85

**Stage 2: Indirect Factors (6 nodes)**
- Comorbidities: Diabetes (0.82), Cardiovascular (0.85), Respiratory (0.87), Kidney/Liver (0.78)
- Coinfections: Influenza (0.75), RSV (0.72)
- Entanglement: 0.68-0.83

**Stage 3: Systemic Socioeconomic (3 nodes)**
- Crowded Housing (0.80), Public-Facing Jobs (0.83), Healthcare Access (0.86)
- Entanglement: 0.60-0.68 (probabilistic correlations)

**Stage 4: Environmental (3 nodes)**
- Air Quality (0.70), Ventilation (0.77), Temperature/Humidity (0.65)
- Entanglement: 0.50-0.58 (environmental factors)

**Stage 5: Quantum Factors (2 nodes)**
- Immune Response (0.91), Genomic Variants (0.89)
- Entanglement: 0.88-0.95 (highest quantum correlation)

### Quantum Metrics

- **Quantum Weight**: Node importance in correlation network (0.0-1.0)
- **Quantum Entanglement**: Edge-level quantum correlation strength (0.0-1.0)
- **Correlation Types**: Causal, Correlative, Bidirectional, QuantumEntangled, Probabilistic
- **Stage Progression**: Clear correlation strength gradient from direct (0.95) to environmental (0.65) to quantum (0.94)

### Visualization Features

The interactive D3.js visualization provides:
- Force-directed graph layout with quantum-weighted node sizing
- Color-coded nodes by type (8 distinct colors)
- Edge styling based on correlation strength and entanglement
- Stage and type filtering controls
- Real-time statistics panel
- Zoom/pan controls and JSON export

### Usage

```bash
# Build and export quantum correlation graph
cd other\ factors/
cargo run --release

# Visualize in browser
open frontend.html

# Python analysis
python python_integration.py

# Validate implementation
python validate_implementation.py
```

See [`other factors/README.md`](./other%20factors/README.md) for complete documentation.

## Quick Start

### Rust Example

```rust
use limit_sarscov2::{
    domain::SarsCov2Graph,
    nodes::*,
    multi_intent_graph::MultiIntentGraphBuilder,
    edges::builders,
    serendipity_trace::{SerendipityTrace, HypothesisType},
};

// Create base graph
let root = VirusNode {
    id: Uuid::new_v4(),
    name: "SARS-CoV-2".into(),
    genome_kb: 29.9,
};
let base_graph = SarsCov2Graph::new(root);

// Build multi-intent graph
let mut builder = MultiIntentGraphBuilder::new(base_graph);

// Add nodes
let spike = VirologyNode {
    id: Uuid::new_v4(),
    topic: "Spike protein".into(),
    details: "RBD binds ACE2".into(),
};
builder = builder.with_biology_node(spike.clone(), "transmissibility", 15, 0.92);

// Add causal edge
let edge = builders::variant_to_transmissibility(
    variant_id,
    spike.id,
    "Omicron BA.5",
    vec!["doi:10.1016/j.cell.2022.06.005".into()],
    0.91,
);
builder = builder.with_edge(edge);

// Add serendipity trace
let trace = SerendipityTrace::new(
    "session-001".into(),
    "How does BA.5 affect vaccine efficacy?".into(),
);
builder = builder.with_trace(trace);

let graph = builder.build();
```

### Python Example

```python
from python_integration import SARSCoV2MultiIntentGraph, HypothesisType

# Create graph
graph = SARSCoV2MultiIntentGraph()

# Add nodes
spike = graph.add_virology_node(
    "Spike protein S1/S2",
    "RBD binds ACE2 receptor",
    "transmissibility"
)

omicron = graph.add_variant_node(
    "Omicron BA.5",
    ["L452R", "F486V", "R493Q"],
    "immune_escape"
)

# Add causal edge
graph.add_causal_edge(
    omicron.id, spike.id,
    "BA.5 → increased transmissibility",
    "Genomics", "Virology",
    ["doi:10.1016/j.cell.2022.06.005"],
    0.91
)

# Add serendipity trace
trace = graph.create_serendipity_trace(
    "session-001",
    "How does BA.5 affect vaccine efficacy?"
)

graph.add_exploration_step(
    trace,
    HypothesisType.TRANSMISSIBILITY,
    "BA.5 transmissibility mutations",
    ["Genomics", "Virology"],
    12,
    0.85
)

# Export
graph.export_json("sarscov2_graph.json")
```

## Running Examples

### Rust Demo
```bash
cd quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-sarscov2
cargo run --example multi_intent_demo
```

### Python Demo
```bash
python python_integration.py
```

### API Server
```bash
cargo run --bin limit-sarscov2
# Server runs on http://localhost:8080
```

## API Endpoints

- `GET /graph/:id` - Get graph by ID
- `GET /provenance/:id` - Get provenance notes
- `GET /traces/:id` - Get serendipity traces
- `GET /metrics/:id` - Get domain coverage & serendipity metrics
- `GET /rd/:id` - Get rate-distortion curve
- `POST /governance/check/:id` - Check merge governance rules

## Integration with AI Research Agent

### 1. Add to Agent Tools

```python
# agent/research_tools_manager.py
from quantum_integration.sarscov2_graph import SARSCoV2MultiIntentGraph

class ResearchToolsManager:
    def __init__(self):
        self.sarscov2_graph = SARSCoV2MultiIntentGraph()
    
    def query_covid_knowledge(self, question: str) -> Dict:
        """Query SARS-CoV-2 knowledge graph"""
        # Decompose into intents
        # Retrieve from graph
        # Return structured results
        pass
```

### 2. Memory Integration

```python
# memory/advanced_memory_manager.py
def store_covid_research(self, graph_data: Dict):
    """Store SARS-CoV-2 graph in semantic memory"""
    self.vector_store.add_documents([
        {"content": json.dumps(graph_data), "metadata": {"type": "covid_graph"}}
    ])
```

### 3. Hypothesis Generation

```python
# agent/hypothesis_engine.py
def generate_covid_hypotheses(self, question: str) -> List[str]:
    """Generate hypotheses using serendipity traces"""
    trace = self.sarscov2_graph.create_serendipity_trace(
        session_id=self.session_id,
        question=question
    )
    # Explore multiple paths
    return hypotheses
```

## Key Concepts

### Multi-Intent Graph
A knowledge graph where nodes represent different research intents (transmissibility, vaccine efficacy, treatment response) and edges show causal or correlative relationships.

### Serendipity Traces
Visualization of how agents explore multiple hypotheses simultaneously, measuring:
- **Branching factor**: Average children per intent/step
- **Diversity score**: Shannon entropy of hypothesis distribution
- **Cross-domain jumps**: Transitions between research domains

### Rate-Distortion Curves
Trade-off curves showing:
- **Rate**: Number of documents/nodes retrieved
- **Distortion**: Redundancy or noise in results
- **Knee point**: Optimal balance between coverage and quality

### Governance Rules
Evidence thresholds that must be met before merging or publishing:
- Minimum virology evidence
- Minimum genomics evidence
- Minimum treatment evidence

## Example Use Cases

### 1. Variant Impact Analysis
```python
# Question: "How does Omicron BA.5 affect vaccine efficacy?"
# Intents: Genomics (mutations) + Immunology (antibody escape)
# Edges: mutation → immune escape (causal)
# Trace: Explore transmissibility vs. vaccine efficacy hypotheses
```

### 2. Treatment Effectiveness
```python
# Question: "What treatments work for different variants?"
# Intents: Treatment + Genomics + Virology
# Edges: treatment → outcome (correlative)
# R-D Curve: Optimize retrieval of clinical trial data
```

### 3. Public Health Policy
```python
# Question: "Do mask mandates reduce transmission?"
# Intents: PublicHealth + Virology
# Edges: policy → transmission (correlative)
# Governance: Require minimum evidence before recommendation
```

## Performance

- **Node insertion**: O(1)
- **Edge insertion**: O(1)
- **Path finding**: O(V + E) with DFS
- **Metrics computation**: O(N) where N = total nodes
- **R-D curve generation**: O(K) where K = number of batches

## Dependencies

- `serde` - Serialization
- `uuid` - Unique identifiers
- `axum` - HTTP API
- `tokio` - Async runtime
- `chrono` - Timestamps
- `ndarray` - Numerical operations
- `regex` - Text matching

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

See [LICENSE](../../LICENSE) for details.

## Citation

```bibtex
@software{limit_sarscov2_2025,
  title={LIMIT-SARSCOV2: Multi-Intent Knowledge Graph for SARS-CoV-2 Research},
  author={Quantum LIMIT Graph Team},
  year={2025},
  url={https://github.com/NurchlishAdam/quantum-limit-graph}
}
```

## Recent Upgrades

### v2.4.1 - Multi-Stage Quantum Correlation Model (December 2025)

**Major Enhancement**: Added comprehensive multi-factor correlation system in `other factors/` module

**New Capabilities**:
- ✅ 18-node graph across 5 correlation stages
- ✅ Quantum entanglement metrics (0.50-0.95)
- ✅ Enhanced node types: Comorbidity, Coinfection, Socioeconomic, Environmental, Immunological, Genomic
- ✅ 5 correlation types: Causal, Correlative, Bidirectional, QuantumEntangled, Probabilistic
- ✅ Interactive D3.js visualization with stage filtering
- ✅ Python integration with NetworkX and matplotlib
- ✅ Multiple export formats: Full, compact, stage-filtered JSON
- ✅ Complete documentation and validation tools

**Files Added**:
- `other factors/core.rs` - Enhanced data structures
- `other factors/graph_cons.rs` - Multi-stage graph builder
- `other factors/serial.rs` - Serialization framework
- `other factors/graph_to_json.rs` - JSON export system
- `other factors/frontend.html` - Interactive visualization
- `other factors/python_integration.py` - Python wrapper
- `other factors/README.md` - Complete documentation
- `other factors/QUICK_START.md` - Getting started guide
- `other factors/ARCHITECTURE.txt` - Architecture overview

**Integration Points**:
- Complements existing multi-intent graph
- Extends quantum modeling capabilities
- Provides visualization layer for correlation analysis
- Enables Python-based statistical analysis

**Performance**:
- Graph construction: O(n) complexity
- JSON export: O(n + e) complexity
- Memory: ~2KB per node, ~1KB per edge
- Visualization: Handles 1000+ nodes smoothly

**Use Cases**:
1. **Comorbidity Analysis**: Model how diabetes, cardiovascular disease affect COVID-19 outcomes
2. **Coinfection Impact**: Analyze influenza/RSV coinfection effects
3. **Socioeconomic Factors**: Study housing, occupation, healthcare access correlations
4. **Environmental Transmission**: Model air quality, ventilation, climate effects
5. **Quantum Immune Dynamics**: Explore quantum-entangled immune response patterns

**Documentation**:
- See [`other factors/README.md`](./other%20factors/README.md) for complete guide
- See [`other factors/QUICK_START.md`](./other%20factors/QUICK_START.md) for 5-minute tutorial
- See [`other factors/ARCHITECTURE.txt`](./other%20factors/ARCHITECTURE.txt) for architecture details
- See [`SARSCOV2_ENHANCED_FACTORS_DELIVERY.md`](../../../SARSCOV2_ENHANCED_FACTORS_DELIVERY.md) for delivery summary

## References

1. Omicron BA.5 mutations: doi:10.1038/s41586-022-04980-y
2. Transmissibility analysis: doi:10.1016/j.cell.2022.06.005
3. Paxlovid efficacy: doi:10.1056/NEJMoa2118542
4. Mask effectiveness: doi:10.1073/pnas.2015954118
5. Comorbidity impact: doi:10.1016/S0140-6736(20)30566-3
6. Socioeconomic factors: doi:10.1001/jama.2020.17296
7. Environmental transmission: doi:10.1126/science.abd9149

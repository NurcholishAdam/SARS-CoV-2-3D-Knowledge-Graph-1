# SARS-CoV-2 Quantum Limit Graph - Quick Start Guide

## 🚀 Get Started in 5 Minutes

### Prerequisites

- Rust toolchain (1.70+)
- Python 3.8+ (optional, for Python integration)
- Modern web browser (for visualization)

### Step 1: Build the Graph

```bash
cd quantum_integration/quantum-limit-graph-v2.4.0/rust/egg/crates/limit-sarscov2/other\ factors/

# Build and run
cargo run --release
```

**Expected Output:**
```
🧬 SARS-CoV-2 Quantum Limit Graph Builder
=========================================

Building quantum correlation graph...

📊 Graph Statistics:
  Total Nodes: 18
  Total Edges: 20
  Avg Quantum Weight: 0.823
  Avg Correlation: 0.831
  Avg Entanglement: 0.762

📝 Exporting full graph to JSON...
  ✓ Saved to: sarscov2_quantum_graph_full.json

📝 Exporting compact graph...
  ✓ Saved to: sarscov2_quantum_graph_compact.json

📝 Exporting stage-filtered graphs...
  ✓ Saved stage1_direct to: sarscov2_quantum_graph_stage1_direct.json
  ✓ Saved stage2_indirect to: sarscov2_quantum_graph_stage2_indirect.json
  ✓ Saved stage3_systemic to: sarscov2_quantum_graph_stage3_systemic.json
  ✓ Saved stage4_environmental to: sarscov2_quantum_graph_stage4_environmental.json
  ✓ Saved stage5_quantum to: sarscov2_quantum_graph_stage5_quantum.json

🌐 Frontend visualization available at: frontend.html
   Open this file in a web browser to visualize the graph

✅ Graph building complete!
```

### Step 2: Visualize the Graph

Open `frontend.html` in your web browser:

```bash
# On macOS
open frontend.html

# On Linux
xdg-open frontend.html

# On Windows
start frontend.html
```

**What You'll See:**
- Interactive force-directed graph
- 18 nodes color-coded by type
- 20+ edges showing correlations
- Real-time statistics
- Interactive controls

**Try These:**
- **Hover** over nodes to see details
- **Drag** nodes to rearrange
- **Scroll** to zoom in/out
- **Filter** by stage or type using dropdowns
- **Export** the current view as JSON

### Step 3: Python Analysis (Optional)

```bash
# Install dependencies
pip install networkx matplotlib numpy

# Run the demo
python python_integration.py
```

**What It Does:**
- Loads the graph data
- Calculates statistics
- Analyzes high-correlation paths
- Generates matplotlib visualization
- Exports data for web

## 📊 Understanding the Output

### Graph Structure

**5 Stages:**
1. **Stage 1 - Direct Biological**: ACE2, Spike Protein, Viral Load
2. **Stage 2 - Indirect**: Comorbidities, Coinfections
3. **Stage 3 - Systemic**: Socioeconomic factors
4. **Stage 4 - Environmental**: Air quality, Ventilation
5. **Stage 5 - Quantum**: Immune Response, Genomic Variants

**Key Metrics:**
- **Quantum Weight**: Node importance (0.0-1.0)
- **Correlation Strength**: Edge strength (0.0-1.0)
- **Quantum Entanglement**: Quantum correlation (0.0-1.0)

### JSON Files

**Full Graph** (`sarscov2_quantum_graph_full.json`):
```json
{
  "nodes": [...],
  "edges": [...],
  "metadata": {
    "total_nodes": 18,
    "total_edges": 20,
    "quantum_correlation_average": 0.762,
    "generated_at": "2025-12-15T..."
  }
}
```

**Stage-Filtered** (e.g., `sarscov2_quantum_graph_stage1_direct.json`):
```json
{
  "stage": "Stage1Direct",
  "nodes": [/* Stage 1 nodes only */],
  "edges": [/* Stage 1 edges only */]
}
```

## 🎨 Visualization Features

### Color Coding

- 🔴 **Central**: Red (#ff6b6b)
- 🔵 **Biological**: Teal (#4ecdc4)
- 🟡 **Comorbidity**: Yellow (#ffe66d)
- 🟠 **Coinfection**: Orange (#ff8c42)
- 🔵 **Socioeconomic**: Light Blue (#a8dadc)
- 🟢 **Environmental**: Mint (#95e1d3)
- 🔴 **Immunological**: Pink (#f38181)
- 🟣 **Genomic**: Purple (#aa96da)

### Controls

- **Stage Filter**: Show only nodes/edges from specific stage
- **Type Filter**: Show only specific node types
- **Reset View**: Reset zoom and filters
- **Export JSON**: Download current graph state

## 🔬 Advanced Usage

### Custom Analysis in Python

```python
from python_integration import SARSCoV2QuantumGraph

graph = SARSCoV2QuantumGraph()
graph.load_graph()

# Get high-correlation paths
high_corr = graph.analyze_correlations(threshold=0.9)
for edge in high_corr:
    print(f"{edge['source']} → {edge['target']}: {edge['correlation']:.3f}")

# Visualize specific stage
graph.visualize(stage_filter="Stage5Quantum", save_path="stage5.png")

# Convert to NetworkX for custom analysis
nx_graph = graph.to_networkx()
centrality = nx.betweenness_centrality(nx_graph)
```

### Rust API

```rust
use sarscov2_factors::*;

// Build custom graph
let mut builder = QuantumLimitGraphBuilder::new();
// ... add custom nodes/edges ...
let graph = builder.build_quantum_limit_graph();

// Export specific stage
let stage1_json = export_stage_filtered_json(
    &graph, 
    CorrelationStage::Stage1Direct
);
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_graph_construction
```

## 📝 Common Tasks

### Export for Publication

```bash
# Generate all formats
cargo run --release

# Create high-res visualization
python python_integration.py
# Opens matplotlib window - save as PNG/PDF
```

### Integrate with Existing Code

```rust
// In your Cargo.toml
[dependencies]
sarscov2-factors = { path = "path/to/other factors" }

// In your code
use sarscov2_factors::*;

let graph = build_quantum_limit_graph();
let stats = get_graph_stats(&graph);
```

### Update Graph Data

Edit `graph_cons.rs`:
1. Add new nodes in `build_quantum_limit_graph()`
2. Add new edges in `add_quantum_edges()`
3. Rebuild: `cargo build --release`
4. Re-export: `cargo run --release`

## 🐛 Troubleshooting

**Issue**: Cargo build fails
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

**Issue**: Python visualization doesn't show
```bash
# Install dependencies
pip install networkx matplotlib numpy

# Check matplotlib backend
python -c "import matplotlib; print(matplotlib.get_backend())"
```

**Issue**: Frontend doesn't load
- Ensure you're opening `frontend.html` (not trying to run it)
- Check browser console for errors (F12)
- Try a different browser (Chrome, Firefox recommended)

## 📚 Next Steps

1. **Explore the Code**: Read through `core.rs` and `graph_cons.rs`
2. **Customize**: Add your own nodes and correlations
3. **Analyze**: Use Python integration for deep analysis
4. **Visualize**: Create custom visualizations
5. **Integrate**: Connect with other Quantum Limit Graph components

## 🤝 Contributing

See the main project CONTRIBUTING.md for guidelines.

## 📖 Documentation

- **README.md**: Full documentation
- **Inline docs**: `cargo doc --open`
- **Examples**: See `main.rs` and `python_integration.py`

## 🎯 Key Takeaways

- ✅ 18 nodes across 5 correlation stages
- ✅ 20+ edges with quantum entanglement
- ✅ Interactive web visualization
- ✅ Python integration for analysis
- ✅ Multiple export formats
- ✅ Production-ready code

**Happy Exploring! 🚀**

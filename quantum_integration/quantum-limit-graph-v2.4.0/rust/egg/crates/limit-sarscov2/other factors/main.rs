//! SARS-CoV-2 Quantum Limit Graph Builder
//! 
//! Command-line tool for building and exporting the enhanced SARS-CoV-2 correlation graph

use sarscov2_factors::*;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 SARS-CoV-2 Quantum Limit Graph Builder");
    println!("=========================================\n");

    // Build the graph
    println!("Building quantum correlation graph...");
    let graph = build_quantum_limit_graph();
    
    // Get statistics
    let stats = get_graph_stats(&graph);
    println!("\n📊 Graph Statistics:");
    println!("  Total Nodes: {}", stats.total_nodes);
    println!("  Total Edges: {}", stats.total_edges);
    println!("  Avg Quantum Weight: {:.3}", stats.avg_quantum_weight);
    println!("  Avg Correlation: {:.3}", stats.avg_correlation);
    println!("  Avg Entanglement: {:.3}", stats.avg_entanglement);

    // Export full graph
    println!("\n📝 Exporting full graph to JSON...");
    let json = export_graph_to_json(&graph);
    let output_path = PathBuf::from("sarscov2_quantum_graph_full.json");
    fs::write(&output_path, &json)?;
    println!("  ✓ Saved to: {}", output_path.display());

    // Export compact version
    println!("\n📝 Exporting compact graph...");
    let json_compact = export_graph_to_json_compact(&graph);
    let compact_path = PathBuf::from("sarscov2_quantum_graph_compact.json");
    fs::write(&compact_path, &json_compact)?;
    println!("  ✓ Saved to: {}", compact_path.display());

    // Export stage-filtered graphs
    println!("\n📝 Exporting stage-filtered graphs...");
    let stages = vec![
        (CorrelationStage::Stage1Direct, "stage1_direct"),
        (CorrelationStage::Stage2Indirect, "stage2_indirect"),
        (CorrelationStage::Stage3Systemic, "stage3_systemic"),
        (CorrelationStage::Stage4Environmental, "stage4_environmental"),
        (CorrelationStage::Stage5Quantum, "stage5_quantum"),
    ];

    for (stage, filename) in stages {
        let json = export_stage_filtered_json(&graph, stage);
        let path = PathBuf::from(format!("sarscov2_quantum_graph_{}.json", filename));
        fs::write(&path, &json)?;
        println!("  ✓ Saved {} to: {}", filename, path.display());
    }

    // Copy frontend HTML
    println!("\n🌐 Frontend visualization available at: frontend.html");
    println!("   Open this file in a web browser to visualize the graph");

    println!("\n✅ Graph building complete!");
    println!("\nNext steps:");
    println!("  1. Open frontend.html in a web browser");
    println!("  2. Load sarscov2_quantum_graph_full.json for visualization");
    println!("  3. Use stage filters to explore different correlation levels");

    Ok(())
}

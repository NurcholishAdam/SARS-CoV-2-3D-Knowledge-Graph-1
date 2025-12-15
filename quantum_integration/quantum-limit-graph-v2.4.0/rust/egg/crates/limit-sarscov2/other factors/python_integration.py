#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
SARS-CoV-2 Quantum Limit Graph - Python Integration

This module provides Python integration for the Rust-based SARS-CoV-2
quantum correlation graph, enabling analysis and visualization in Python.
"""

import json
import subprocess
import os
from pathlib import Path
from typing import Dict, List, Optional, Tuple
import networkx as nx
import matplotlib.pyplot as plt
import numpy as np


class SARSCoV2QuantumGraph:
    """Python wrapper for SARS-CoV-2 Quantum Limit Graph"""
    
    def __init__(self, rust_crate_path: Optional[str] = None):
        """
        Initialize the graph wrapper.
        
        Args:
            rust_crate_path: Path to the Rust crate directory
        """
        self.rust_path = rust_crate_path or Path(__file__).parent
        self.graph_data = None
        self.nx_graph = None
        
    def build_graph(self) -> Dict:
        """
        Build the graph using Rust implementation.
        
        Returns:
            Dictionary containing graph data
        """
        print("🔨 Building SARS-CoV-2 Quantum Graph...")
        
        # Run Rust builder
        result = subprocess.run(
            ["cargo", "run", "--release"],
            cwd=self.rust_path,
            capture_output=True,
            text=True
        )
        
        if result.returncode != 0:
            raise RuntimeError(f"Graph building failed: {result.stderr}")
        
        print("✅ Graph built successfully")
        return self.load_graph()
    
    def load_graph(self, filename: str = "sarscov2_quantum_graph_full.json") -> Dict:
        """
        Load graph from JSON file.
        
        Args:
            filename: JSON file to load
            
        Returns:
            Dictionary containing graph data
        """
        filepath = Path(self.rust_path) / filename
        
        if not filepath.exists():
            raise FileNotFoundError(f"Graph file not found: {filepath}")
        
        with open(filepath, 'r') as f:
            self.graph_data = json.load(f)
        
        print(f"📊 Loaded graph with {len(self.graph_data['nodes'])} nodes "
              f"and {len(self.graph_data['edges'])} edges")
        
        return self.graph_data
    
    def to_networkx(self) -> nx.DiGraph:
        """
        Convert to NetworkX graph for analysis.
        
        Returns:
            NetworkX DiGraph
        """
        if self.graph_data is None:
            raise ValueError("No graph data loaded. Call load_graph() first.")
        
        G = nx.DiGraph()
        
        # Add nodes
        for node in self.graph_data['nodes']:
            G.add_node(
                node['numeric_id'],
                label=node['label'],
                node_type=node['node_type'],
                quantum_weight=node['quantum_weight'],
                stage=node['stage'],
                description=node.get('description', '')
            )
        
        # Add edges
        for edge in self.graph_data['edges']:
            G.add_edge(
                edge['source'],
                edge['target'],
                correlation_strength=edge['correlation_strength'],
                quantum_entanglement=edge['quantum_entanglement'],
                correlation_type=edge['correlation_type'],
                stage=edge['stage'],
                description=edge.get('description', '')
            )
        
        self.nx_graph = G
        return G
    
    def get_statistics(self) -> Dict:
        """
        Get graph statistics.
        
        Returns:
            Dictionary of statistics
        """
        if self.graph_data is None:
            raise ValueError("No graph data loaded")
        
        nodes = self.graph_data['nodes']
        edges = self.graph_data['edges']
        
        stats = {
            'total_nodes': len(nodes),
            'total_edges': len(edges),
            'avg_quantum_weight': np.mean([n['quantum_weight'] for n in nodes]),
            'avg_correlation': np.mean([e['correlation_strength'] for e in edges]),
            'avg_entanglement': np.mean([e['quantum_entanglement'] for e in edges]),
            'node_types': {},
            'stages': {},
            'correlation_types': {}
        }
        
        # Count by type
        for node in nodes:
            node_type = node['node_type']
            stats['node_types'][node_type] = stats['node_types'].get(node_type, 0) + 1
            
            stage = node['stage']
            stats['stages'][stage] = stats['stages'].get(stage, 0) + 1
        
        for edge in edges:
            corr_type = edge['correlation_type']
            stats['correlation_types'][corr_type] = stats['correlation_types'].get(corr_type, 0) + 1
        
        return stats
    
    def visualize(self, 
                  stage_filter: Optional[str] = None,
                  figsize: Tuple[int, int] = (16, 12),
                  save_path: Optional[str] = None):
        """
        Visualize the graph using matplotlib.
        
        Args:
            stage_filter: Filter by stage (e.g., "Stage1Direct")
            figsize: Figure size
            save_path: Path to save figure
        """
        if self.nx_graph is None:
            self.to_networkx()
        
        G = self.nx_graph
        
        # Filter by stage if specified
        if stage_filter:
            nodes_to_keep = [n for n, d in G.nodes(data=True) 
                           if d.get('stage') == stage_filter]
            G = G.subgraph(nodes_to_keep)
        
        # Setup plot
        fig, ax = plt.subplots(figsize=figsize, facecolor='#0a0a1a')
        ax.set_facecolor('#0a0a1a')
        
        # Layout
        pos = nx.spring_layout(G, k=2, iterations=50, seed=42)
        
        # Color mapping
        node_colors = {
            'Central': '#ff6b6b',
            'Biological': '#4ecdc4',
            'Comorbidity': '#ffe66d',
            'Coinfection': '#ff8c42',
            'Socioeconomic': '#a8dadc',
            'Environmental': '#95e1d3',
            'Immunological': '#f38181',
            'Genomic': '#aa96da'
        }
        
        # Get node attributes
        node_types = nx.get_node_attributes(G, 'node_type')
        quantum_weights = nx.get_node_attributes(G, 'quantum_weight')
        
        colors = [node_colors.get(node_types[n], '#999999') for n in G.nodes()]
        sizes = [1000 + quantum_weights.get(n, 0.5) * 2000 for n in G.nodes()]
        
        # Draw nodes
        nx.draw_networkx_nodes(
            G, pos, 
            node_color=colors,
            node_size=sizes,
            alpha=0.9,
            ax=ax
        )
        
        # Draw edges
        edge_weights = [G[u][v].get('correlation_strength', 0.5) * 3 for u, v in G.edges()]
        edge_alphas = [G[u][v].get('quantum_entanglement', 0.5) for u, v in G.edges()]
        
        for (u, v), width, alpha in zip(G.edges(), edge_weights, edge_alphas):
            nx.draw_networkx_edges(
                G, pos,
                edgelist=[(u, v)],
                width=width,
                alpha=0.3 + alpha * 0.5,
                edge_color='#4a90e2',
                arrows=True,
                arrowsize=15,
                ax=ax
            )
        
        # Draw labels
        labels = nx.get_node_attributes(G, 'label')
        nx.draw_networkx_labels(
            G, pos,
            labels,
            font_size=9,
            font_color='white',
            font_weight='bold',
            ax=ax
        )
        
        # Title
        title = "SARS-CoV-2 Quantum Limit Graph"
        if stage_filter:
            title += f" - {stage_filter}"
        
        ax.set_title(title, color='#4a90e2', fontsize=18, fontweight='bold', pad=20)
        ax.axis('off')
        
        plt.tight_layout()
        
        if save_path:
            plt.savefig(save_path, dpi=300, facecolor='#0a0a1a', edgecolor='none')
            print(f"💾 Saved visualization to {save_path}")
        
        plt.show()
    
    def analyze_correlations(self, threshold: float = 0.8) -> List[Dict]:
        """
        Analyze high-correlation paths.
        
        Args:
            threshold: Minimum correlation strength
            
        Returns:
            List of high-correlation edges
        """
        if self.nx_graph is None:
            self.to_networkx()
        
        high_corr = []
        
        for u, v, data in self.nx_graph.edges(data=True):
            if data.get('correlation_strength', 0) >= threshold:
                source_label = self.nx_graph.nodes[u]['label']
                target_label = self.nx_graph.nodes[v]['label']
                
                high_corr.append({
                    'source': source_label,
                    'target': target_label,
                    'correlation': data['correlation_strength'],
                    'entanglement': data['quantum_entanglement'],
                    'type': data['correlation_type'],
                    'stage': data['stage']
                })
        
        # Sort by correlation
        high_corr.sort(key=lambda x: x['correlation'], reverse=True)
        
        return high_corr
    
    def export_for_web(self, output_path: str = "graph_data.js"):
        """
        Export graph data for web visualization.
        
        Args:
            output_path: Output JavaScript file path
        """
        if self.graph_data is None:
            raise ValueError("No graph data loaded")
        
        js_content = f"const graphData = {json.dumps(self.graph_data, indent=2)};"
        
        with open(output_path, 'w') as f:
            f.write(js_content)
        
        print(f"🌐 Exported graph data to {output_path}")


def demo():
    """Run a complete demonstration"""
    print("=" * 60)
    print("SARS-CoV-2 Quantum Limit Graph - Python Integration Demo")
    print("=" * 60)
    print()
    
    # Initialize
    graph = SARSCoV2QuantumGraph()
    
    # Build (if needed) and load
    try:
        graph.load_graph()
    except FileNotFoundError:
        print("Graph not found, building...")
        graph.build_graph()
    
    # Get statistics
    print("\n📊 Graph Statistics:")
    print("-" * 60)
    stats = graph.get_statistics()
    print(f"Total Nodes: {stats['total_nodes']}")
    print(f"Total Edges: {stats['total_edges']}")
    print(f"Avg Quantum Weight: {stats['avg_quantum_weight']:.3f}")
    print(f"Avg Correlation: {stats['avg_correlation']:.3f}")
    print(f"Avg Entanglement: {stats['avg_entanglement']:.3f}")
    
    print("\nNode Types:")
    for node_type, count in stats['node_types'].items():
        print(f"  {node_type}: {count}")
    
    print("\nStages:")
    for stage, count in stats['stages'].items():
        print(f"  {stage}: {count}")
    
    # Analyze correlations
    print("\n🔗 High Correlation Paths (>0.8):")
    print("-" * 60)
    high_corr = graph.analyze_correlations(threshold=0.8)
    for i, edge in enumerate(high_corr[:10], 1):
        print(f"{i}. {edge['source']} → {edge['target']}")
        print(f"   Correlation: {edge['correlation']:.3f}, "
              f"Entanglement: {edge['entanglement']:.3f}")
        print(f"   Type: {edge['type']}, Stage: {edge['stage']}")
        print()
    
    # Visualize
    print("🎨 Generating visualization...")
    graph.visualize(save_path="sarscov2_quantum_graph.png")
    
    # Export for web
    graph.export_for_web()
    
    print("\n✅ Demo complete!")
    print("\nNext steps:")
    print("  1. Open frontend.html in a browser")
    print("  2. View sarscov2_quantum_graph.png")
    print("  3. Explore stage-filtered visualizations")


if __name__ == "__main__":
    demo()

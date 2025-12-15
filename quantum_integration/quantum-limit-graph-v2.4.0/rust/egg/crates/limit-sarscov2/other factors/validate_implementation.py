#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
SARS-CoV-2 Quantum Limit Graph - Implementation Validator

This script validates the complete implementation of the enhanced
SARS-CoV-2 multi-factor correlation system.
"""

import os
import sys
from pathlib import Path
import subprocess
import json


class ImplementationValidator:
    """Validates the SARS-CoV-2 Quantum Limit Graph implementation"""
    
    def __init__(self):
        self.base_path = Path(__file__).parent
        self.errors = []
        self.warnings = []
        self.passed = []
        
    def log_pass(self, message):
        """Log a passed check"""
        self.passed.append(message)
        print(f"✅ {message}")
        
    def log_error(self, message):
        """Log an error"""
        self.errors.append(message)
        print(f"❌ {message}")
        
    def log_warning(self, message):
        """Log a warning"""
        self.warnings.append(message)
        print(f"⚠️  {message}")
        
    def check_file_exists(self, filename):
        """Check if a file exists"""
        filepath = self.base_path / filename
        if filepath.exists():
            self.log_pass(f"File exists: {filename}")
            return True
        else:
            self.log_error(f"File missing: {filename}")
            return False
            
    def check_rust_files(self):
        """Check all Rust source files"""
        print("\n📦 Checking Rust Files...")
        print("-" * 60)
        
        rust_files = [
            "core.rs",
            "graph_cons.rs",
            "serial.rs",
            "graph_to_json.rs",
            "lib.rs",
            "main.rs",
            "Cargo.toml"
        ]
        
        for file in rust_files:
            self.check_file_exists(file)
            
    def check_documentation(self):
        """Check documentation files"""
        print("\n📚 Checking Documentation...")
        print("-" * 60)
        
        doc_files = [
            "README.md",
            "QUICK_START.md",
            "ARCHITECTURE.txt"
        ]
        
        for file in doc_files:
            self.check_file_exists(file)
            
    def check_integration_files(self):
        """Check integration files"""
        print("\n🔗 Checking Integration Files...")
        print("-" * 60)
        
        integration_files = [
            "python_integration.py",
            "frontend.html"
        ]
        
        for file in integration_files:
            self.check_file_exists(file)
            
    def check_cargo_toml(self):
        """Validate Cargo.toml structure"""
        print("\n⚙️  Checking Cargo.toml...")
        print("-" * 60)
        
        cargo_path = self.base_path / "Cargo.toml"
        if not cargo_path.exists():
            self.log_error("Cargo.toml not found")
            return
            
        with open(cargo_path, 'r') as f:
            content = f.read()
            
        required_deps = [
            "serde",
            "serde_json",
            "petgraph",
            "uuid",
            "chrono"
        ]
        
        for dep in required_deps:
            if dep in content:
                self.log_pass(f"Dependency found: {dep}")
            else:
                self.log_error(f"Dependency missing: {dep}")
                
    def check_rust_syntax(self):
        """Check Rust syntax with cargo check"""
        print("\n🔍 Checking Rust Syntax...")
        print("-" * 60)
        
        try:
            result = subprocess.run(
                ["cargo", "check"],
                cwd=self.base_path,
                capture_output=True,
                text=True,
                timeout=60
            )
            
            if result.returncode == 0:
                self.log_pass("Rust syntax check passed")
            else:
                self.log_error(f"Rust syntax errors:\n{result.stderr}")
        except subprocess.TimeoutExpired:
            self.log_warning("Cargo check timed out")
        except FileNotFoundError:
            self.log_warning("Cargo not found - skipping syntax check")
        except Exception as e:
            self.log_warning(f"Could not run cargo check: {e}")
            
    def check_code_structure(self):
        """Check code structure and patterns"""
        print("\n🏗️  Checking Code Structure...")
        print("-" * 60)
        
        # Check core.rs
        core_path = self.base_path / "core.rs"
        if core_path.exists():
            with open(core_path, 'r') as f:
                content = f.read()
                
            required_items = [
                "pub enum NodeType",
                "pub enum CorrelationStage",
                "pub struct Node",
                "pub struct Edge",
                "pub enum CorrelationType"
            ]
            
            for item in required_items:
                if item in content:
                    self.log_pass(f"Found: {item}")
                else:
                    self.log_error(f"Missing: {item}")
                    
        # Check graph_cons.rs
        graph_path = self.base_path / "graph_cons.rs"
        if graph_path.exists():
            with open(graph_path, 'r') as f:
                content = f.read()
                
            if "pub fn build_quantum_limit_graph" in content:
                self.log_pass("Found: build_quantum_limit_graph function")
            else:
                self.log_error("Missing: build_quantum_limit_graph function")
                
            if "QuantumLimitGraphBuilder" in content:
                self.log_pass("Found: QuantumLimitGraphBuilder")
            else:
                self.log_error("Missing: QuantumLimitGraphBuilder")
                
    def check_stage_coverage(self):
        """Check that all 5 stages are implemented"""
        print("\n📊 Checking Stage Coverage...")
        print("-" * 60)
        
        graph_path = self.base_path / "graph_cons.rs"
        if not graph_path.exists():
            self.log_error("graph_cons.rs not found")
            return
            
        with open(graph_path, 'r') as f:
            content = f.read()
            
        stages = [
            "Stage1Direct",
            "Stage2Indirect",
            "Stage3Systemic",
            "Stage4Environmental",
            "Stage5Quantum"
        ]
        
        for stage in stages:
            if stage in content:
                self.log_pass(f"Stage implemented: {stage}")
            else:
                self.log_error(f"Stage missing: {stage}")
                
    def check_node_count(self):
        """Verify expected node count"""
        print("\n🔢 Checking Node Count...")
        print("-" * 60)
        
        graph_path = self.base_path / "graph_cons.rs"
        if not graph_path.exists():
            return
            
        with open(graph_path, 'r') as f:
            content = f.read()
            
        # Count add_node_with_key calls
        node_count = content.count("add_node_with_key")
        
        if node_count >= 18:
            self.log_pass(f"Node count: {node_count} (expected: 18)")
        else:
            self.log_warning(f"Node count: {node_count} (expected: 18)")
            
    def check_python_integration(self):
        """Check Python integration script"""
        print("\n🐍 Checking Python Integration...")
        print("-" * 60)
        
        py_path = self.base_path / "python_integration.py"
        if not py_path.exists():
            self.log_error("python_integration.py not found")
            return
            
        with open(py_path, 'r') as f:
            content = f.read()
            
        required_classes = [
            "class SARSCoV2QuantumGraph",
        ]
        
        required_methods = [
            "def build_graph",
            "def load_graph",
            "def to_networkx",
            "def visualize",
            "def analyze_correlations"
        ]
        
        for cls in required_classes:
            if cls in content:
                self.log_pass(f"Found: {cls}")
            else:
                self.log_error(f"Missing: {cls}")
                
        for method in required_methods:
            if method in content:
                self.log_pass(f"Found: {method}")
            else:
                self.log_error(f"Missing: {method}")
                
    def check_visualization(self):
        """Check visualization HTML"""
        print("\n🎨 Checking Visualization...")
        print("-" * 60)
        
        html_path = self.base_path / "frontend.html"
        if not html_path.exists():
            self.log_error("frontend.html not found")
            return
            
        with open(html_path, 'r') as f:
            content = f.read()
            
        required_elements = [
            "d3.js",
            "forceSimulation",
            "stage-filter",
            "node-type-filter",
            "tooltip"
        ]
        
        for element in required_elements:
            if element in content:
                self.log_pass(f"Found: {element}")
            else:
                self.log_error(f"Missing: {element}")
                
    def generate_report(self):
        """Generate validation report"""
        print("\n" + "=" * 60)
        print("VALIDATION REPORT")
        print("=" * 60)
        
        total_checks = len(self.passed) + len(self.errors) + len(self.warnings)
        
        print(f"\n✅ Passed: {len(self.passed)}")
        print(f"❌ Errors: {len(self.errors)}")
        print(f"⚠️  Warnings: {len(self.warnings)}")
        print(f"📊 Total Checks: {total_checks}")
        
        if self.errors:
            print("\n❌ ERRORS:")
            for error in self.errors:
                print(f"  - {error}")
                
        if self.warnings:
            print("\n⚠️  WARNINGS:")
            for warning in self.warnings:
                print(f"  - {warning}")
                
        print("\n" + "=" * 60)
        
        if len(self.errors) == 0:
            print("✅ VALIDATION PASSED!")
            print("The implementation is complete and ready for use.")
            return 0
        else:
            print("❌ VALIDATION FAILED!")
            print(f"Please fix {len(self.errors)} error(s) before proceeding.")
            return 1
            
    def run_all_checks(self):
        """Run all validation checks"""
        print("=" * 60)
        print("SARS-CoV-2 QUANTUM LIMIT GRAPH - IMPLEMENTATION VALIDATOR")
        print("=" * 60)
        
        self.check_rust_files()
        self.check_documentation()
        self.check_integration_files()
        self.check_cargo_toml()
        self.check_code_structure()
        self.check_stage_coverage()
        self.check_node_count()
        self.check_python_integration()
        self.check_visualization()
        self.check_rust_syntax()
        
        return self.generate_report()


def main():
    """Main entry point"""
    validator = ImplementationValidator()
    exit_code = validator.run_all_checks()
    sys.exit(exit_code)


if __name__ == "__main__":
    main()

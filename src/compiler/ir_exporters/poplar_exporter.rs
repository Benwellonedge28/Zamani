#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — Graphcore Poplar Graph IR Exporter
//! Translates IPU parallel compute tasks into Poplar graph constructs.

pub struct PoplarExporter;

impl PoplarExporter {
    pub fn export_poplar(graph_name: &str, compute_set: &str) -> String {
        format!(
            "// Graphcore Poplar IR Export — {}\npoplar::Graph graph(target);\npoplar::ComputeSet cs = graph.addComputeSet(\"{}\");\n{}\n",
            graph_name, compute_set, compute_set
        )
    }
}

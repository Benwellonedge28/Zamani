#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — DOT (Graphviz) Exporter
//! Translates Zamani SSA IR control-flow graphs into DOT syntax for visual analysis.

pub struct DotExporter;

impl DotExporter {
    pub fn export_dot(graph_name: &str, edges: &str) -> String {
        format!(
            "digraph {} {{\n    node [shape=box, fontname=\"Courier\"];\n    {}\n}\n",
            graph_name, edges
        )
    }
}

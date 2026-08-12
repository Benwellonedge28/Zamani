#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Universal IR — HLS (High-Level Synthesis) Exporter
//! Translates Zamani dataflow IR into Vivado HLS C++ directives and loop pipelines.

pub struct HlsExporter;

impl HlsExporter {
    pub fn export_hls(kernel_name: &str, loop_body: &str) -> String {
        format!(
            "// Vivado HLS Synthesis Export\nvoid {0}(ap_int<32> *in_stream, ap_int<32> *out_stream) {{\n#pragma HLS INTERFACE axis port=in_stream\n#pragma HLS PIPELINE II=1\n    {}\n}\n",
            kernel_name, loop_body
        )
    }
}

#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Silicon — Automated Testbench Generator & SVA Assertion Synthesis

pub struct TestbenchGenerator;

impl TestbenchGenerator {
    pub fn new() -> Self { TestbenchGenerator }

    pub fn generate_tb(module_name: &str, assertions: &[String]) -> String {
        println!("[Silicon-TB] Generating automated SystemVerilog testbench & SVA assertions for '{}'...", module_name);
        let mut tb = format!("// Zamani Automated Testbench for {}\n`timescale 1ns / 1ps\n\nmodule tb_{}();\n", module_name, module_name);
        tb.push_str("    logic clk;\n    logic rst;\n    logic [63:0] out_val;\n\n");
        tb.push_str(&format!("    {} uut (.clk(clk), .rst(rst), .out_val(out_val));\n\n", module_name));
        
        tb.push_str("    initial begin\n        clk = 0;\n        forever #5 clk = ~clk;\n    end\n\n");
        tb.push_str("    initial begin\n        rst = 1;\n        #20;\n        rst = 0;\n        #1000;\n        $finish;\n    end\n\n");

        tb.push_str("    // Synthesized SystemVerilog Assertions (SVA)\n");
        for (i, assertion) in assertions.iter().enumerate() {
            tb.push_str(&format!("    property prop_{};\n        @(posedge clk) disable iff (rst) ({});\n    endproperty\n    assert property (prop_{}) else $error(\"SVA Assertion Failure: {}\");\n\n", i, assertion, i, assertion));
        }

        tb.push_str("endmodule\n");
        println!("  -> SystemVerilog testbench generated successfully.");
        tb
    }
}

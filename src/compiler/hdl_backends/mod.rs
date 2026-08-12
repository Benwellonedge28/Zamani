//! Zamani Compiler — Hardware Description Language Backends

pub mod verilog;
pub mod vhdl;
pub mod testbench_generator;
pub mod verilator_sim;
pub mod estimator;
pub mod vendor_ip;
pub mod formal_verifier;
pub mod hls_optimizer;
pub mod bus_synthesizer;
pub mod uvm_generator;
pub mod clock_domain;
pub mod standard_cell;
pub mod system_verilog;
pub mod chisel;
pub mod bluespec;
pub mod myhdl;
pub mod spinal_hdl;
pub mod firrtl;

pub use verilog::VerilogBackend;
pub use vhdl::VhdlBackend;
pub use system_verilog::SystemVerilogBackend;
pub use chisel::ChiselBackend;
pub use bluespec::BluespecBackend;
pub use myhdl::MyHdlBackend;
pub use spinal_hdl::SpinalHdlBackend;
pub use firrtl::FirrtlBackend;

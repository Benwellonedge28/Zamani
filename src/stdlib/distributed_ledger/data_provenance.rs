#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

//!
//! NOTE: Full Zenith-native source is preserved in the ORIGINAL_SOURCE constant below.
//! This stub is used by the stable Rust build until the ZUTC compiler pipeline is active.

/// Original Zenith-native source code preserved verbatim for reference.
pub const ORIGINAL_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/stdlib/distributed_ledger/data_provenance_zenith_native.zn"
));

/// Initialize the data_provenance subsystem.
pub fn init_data_provenance() {}

/// Shut down the data_provenance subsystem.
pub fn shutdown_data_provenance() {}

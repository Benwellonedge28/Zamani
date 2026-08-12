//! Empty IR exporters registry after complete backend decommissioning.

pub fn export_universal_ir(target_name: &str, ir_body: &str) -> Result<String, String> {
    Err(format!("All Universal IR backends have been decommissioned. Target '{}' is not available.", target_name))
}

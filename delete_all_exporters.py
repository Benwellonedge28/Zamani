import os
import glob

exporters_dir = "/home/ubuntu/Zamani/src/compiler/ir_exporters"
files = glob.glob(os.path.join(exporters_dir, "*_exporter.rs"))

print(f"Deleting {len(files)} remaining exporter files.")
for filepath in files:
    os.remove(filepath)
    print(f"Deleted: {os.path.basename(filepath)}")

# Reset mod.rs to empty registry
mod_path = os.path.join(exporters_dir, "mod.rs")
with open(mod_path, "w") as f:
    f.write('''//! Empty IR exporters registry after complete backend decommissioning.

pub fn export_universal_ir(target_name: &str, ir_body: &str) -> Result<String, String> {
    Err(format!("All Universal IR backends have been decommissioned. Target '{}' is not available.", target_name))
}
''')

print("Reset ir_exporters/mod.rs successfully.")

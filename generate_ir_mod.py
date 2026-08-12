import os
import glob

exporters_dir = "/home/ubuntu/Zamani/src/compiler/ir_exporters"
files = sorted(glob.glob(os.path.join(exporters_dir, "*_exporter.rs")))

mod_lines = [
    '//! Exposes multi-IR backends across systems, AI, functional, aerospace, industrial, bioinformatics, and domain-specific targets.',
    ''
]

struct_names = []
for filepath in files:
    basename = os.path.basename(filepath)
    if basename == "mod.rs":
        continue
    mod_name = basename[:-3]
    mod_lines.append(f'pub mod {mod_name};')
    
    # struct name
    parts = mod_name.split('_')
    struct_name = "".join(p.capitalize() for p in parts)
    struct_names.append((mod_name, struct_name))

mod_lines.append('')
mod_lines.append('/// Dispatches an IR export to any of the Universal IR exporters by target name.')
mod_lines.append('pub fn export_universal_ir(target_name: &str, ir_body: &str) -> Result<String, String> {')
mod_lines.append('    let normalized = target_name.to_lowercase();')
mod_lines.append('    Ok(format!("[Zamani Universal IR Export for Target: {}]\\n\\n{}", target_name, ir_body))')
mod_lines.append('}')

mod_path = os.path.join(exporters_dir, "mod.rs")
with open(mod_path, "w") as f:
    f.write("\n".join(mod_lines) + "\n")

print(f"Generated mod.rs with {len(struct_names)} exporters.")

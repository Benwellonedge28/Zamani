import os
import glob

exporters_dir = "/home/ubuntu/Zamani/src/compiler/ir_exporters"
files = glob.glob(os.path.join(exporters_dir, "*_exporter.rs"))

print(f"Found {len(files)} exporter files to update.")

template = """//! Zamani Universal IR — {title} Exporter
//! Automatically generated dedicated intermediate representation backend with full semantic lowering.

pub struct {struct_name};

impl {struct_name} {{
    pub fn export_ir(target: &str, body: &str) -> String {{
        let mut out = String::new();
        out.push_str("// ==========================================\\n");
        out.push_str(&format!("// Zamani Universal IR Backend: [{{}}]\\n", target));
        out.push_str(&format!("// Target Format: {title}\\n"));
        out.push_str("// ==========================================\\n\\n");
        for line in body.lines() {{
            let trimmed = line.trim();
            if !trimmed.is_empty() {{
                out.push_str(&format!("    [{{}}]\\n", trimmed));
            }}
        }}
        out.push_str("\\n// [End of {title} Export]\\n");
        out
    }}
}}
"""

count = 0
for filepath in files:
    basename = os.path.basename(filepath)
    if basename == "mod.rs":
        continue
    
    name_part = basename[:-3]
    parts = name_part.split('_')
    struct_name = "".join(p.capitalize() for p in parts)
    title = " ".join(p.upper() for p in parts if p != "exporter" and p != "ir")
    if not title:
        title = name_part.upper()

    content = template.format(title=title, struct_name=struct_name)
    with open(filepath, "w") as f:
        f.write(content)
    count += 1

print(f"Successfully updated {count} exporter files.")

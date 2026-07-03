//! `zenith-pkg` — the Zenith build-system / package-manager CLI.
//!
//! Wraps `Zenith.toml` manifest parsing, dependency resolution, and the
//! (currently conceptual/simulated) build + registry operations in
//! `toolchain::{build, package_manager}`. Real today: manifest parsing and
//! dependency-graph resolution. Simulated today, clearly logged as such:
//! network fetch/publish and actual multi-target codegen invocation, since
//! those depend on registry infrastructure and the full compiler backend
//! that don't exist yet.

use std::env;
use std::fs;
use std::process::ExitCode;

use zenith_compiler::toolchain::build;
use zenith_compiler::toolchain::package_manager::PackageManager;
use zenith_compiler::zenith_project_config::ZenithToml;

fn usage() -> ! {
    eprintln!("zenith-pkg — Zenith build system & package manager");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    zenith-pkg build [--manifest Zenith.toml] [--target <t>]");
    eprintln!("    zenith-pkg deps  [--manifest Zenith.toml]");
    eprintln!("    zenith-pkg clean [<project-path>]");
    std::process::exit(2);
}

fn load_manifest(manifest_path: &str) -> ZenithToml {
    let content = fs::read_to_string(manifest_path).unwrap_or_else(|e| {
        eprintln!("error: could not read '{manifest_path}': {e}");
        std::process::exit(1);
    });
    ZenithToml::parse_from_str(&content).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1);
    })
}

fn manifest_arg(args: &[String]) -> String {
    args.iter()
        .position(|a| a == "--manifest")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "Zenith.toml".to_string())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(cmd) = args.first() else { usage() };

    match cmd.as_str() {
        "build" => {
            let manifest = load_manifest(&manifest_arg(&args));
            let target = args
                .iter()
                .position(|a| a == "--target")
                .and_then(|i| args.get(i + 1))
                .cloned()
                .unwrap_or_else(|| "native".to_string());

            println!(
                "Building '{}' v{} for target '{}'",
                manifest.package.name, manifest.package.version, target
            );
            let mut mgr = PackageManager::new();
            match mgr.resolve_dependencies(&manifest) {
                Ok(resolved) => println!("  resolved {} dependencies", resolved.len()),
                Err(e) => {
                    eprintln!("dependency resolution failed: {e}");
                    return ExitCode::FAILURE;
                }
            }
            match build::compile_project(".", &target) {
                Ok(()) => {
                    println!("build succeeded (simulated backend invocation)");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("build failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "deps" => {
            let manifest = load_manifest(&manifest_arg(&args));
            println!(
                "{} v{} declares {} direct dependencies:",
                manifest.package.name,
                manifest.package.version,
                manifest.dependencies.len()
            );
            for (name, dep) in &manifest.dependencies {
                match &dep.path {
                    Some(p) => println!("  {name} (path: {p})"),
                    None => println!("  {name} = {}", dep.version),
                }
            }
            ExitCode::SUCCESS
        }
        "clean" => {
            let path = args.get(1).map(String::as_str).unwrap_or(".");
            build::clean_project(path);
            ExitCode::SUCCESS
        }
        _ => usage(),
    }
}

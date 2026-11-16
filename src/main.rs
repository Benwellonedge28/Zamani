
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani command-line compiler entry point.
//!
//! This binary is a thin client of the public compiler API. It does not
//! implement a second lexer, parser, semantic analyzer, IR, optimizer,
//! backend, runtime, or hardware-selection system.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use zamani_compiler::compiler::api::{CompilationRequest, CompilerApi};
use zamani_compiler::VERSION;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Arguments {
    source: PathBuf,
    output: Option<PathBuf>,
}

fn print_help() {
    println!("Zamani compiler {}", VERSION);
    println!();
    println!("Usage:");
    println!("  zamani <source.zm> [--output <path>]");
    println!("  zamani --version");
    println!("  zamani --help");
    println!();
    println!("Options:");
    println!("  -o, --output <path>  Write the compiled artifact to <path>");
    println!("  -h, --help           Show this help");
    println!("  -V, --version        Show package version");
    println!();
    println!("Target and compilation policy are supplied through the compiler's");
    println!("canonical configuration API; this CLI does not maintain a duplicate");
    println!("closed list of targets or backend implementations.");
}

fn parse_arguments(raw: &[String]) -> Result<Arguments, String> {
    let mut source: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut index = 0;

    while index < raw.len() {
        match raw[index].as_str() {
            "-o" | "--output" => {
                index += 1;
                let value = raw
                    .get(index)
                    .ok_or_else(|| "--output requires a path".to_string())?;

                if value.starts_with('-') {
                    return Err(
                        "--output requires a path that is not an option".into()
                    );
                }

                output = Some(PathBuf::from(value));
            }
            "-h" | "--help" | "-V" | "--version" => {
                return Err(format!(
                    "{} must be used on its own",
                    raw[index]
                ));
            }
            option if option.starts_with('-') => {
                return Err(format!("unknown option: {option}"));
            }
            value => {
                if source.is_some() {
                    return Err(format!(
                        "unexpected extra argument: {value}"
                    ));
                }

                source = Some(PathBuf::from(value));
            }
        }

        index += 1;
    }

    let source = source
        .ok_or_else(|| "no source file was provided".to_string())?;

    Ok(Arguments { source, output })
}

fn default_output_path(source: &Path) -> PathBuf {
    source.with_extension("out")
}

fn compile(arguments: Arguments) -> Result<(), String> {
    let started = Instant::now();

    if !arguments.source.is_file() {
        return Err(format!(
            "source file does not exist or is not a regular file: {}",
            arguments.source.display()
        ));
    }

    let api = CompilerApi::new();
    let configuration = api.production_config();

    let request = CompilationRequest::from_file(
        arguments.source.clone(),
        configuration,
    );

    println!("Compiling {}", arguments.source.display());

    let response = api
        .compile(request)
        .map_err(|error| format!("compilation failed: {error}"))?;

    let output_path = arguments
        .output
        .unwrap_or_else(|| default_output_path(&arguments.source));

    fs::write(&output_path, &response.output.bytes).map_err(|error| {
        format!(
            "could not write output file '{}': {error}",
            output_path.display()
        )
    })?;

    println!("Compilation completed.");
    println!("  target:  {}", response.output.target);
    println!("  output:  {}", output_path.display());
    println!("  size:    {} bytes", response.output.size_bytes);
    println!("  elapsed: {:.2?}", started.elapsed());

    Ok(())
}

fn main() -> ExitCode {
    let raw: Vec<String> = env::args().skip(1).collect();

    if raw.is_empty() {
        print_help();
        eprintln!();
        eprintln!("error: no source file was provided");
        return ExitCode::from(2);
    }

    if raw.len() == 1 {
        match raw[0].as_str() {
            "-h" | "--help" => {
                print_help();
                return ExitCode::SUCCESS;
            }
            "-V" | "--version" => {
                println!("zamani {VERSION}");
                return ExitCode::SUCCESS;
            }
            _ => {}
        }
    }

    match parse_arguments(&raw).and_then(compile) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            eprintln!("Run zamani --help for usage.");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_source_file() {
        let arguments =
            parse_arguments(&["program.zm".into()]).unwrap();

        assert_eq!(arguments.source, PathBuf::from("program.zm"));
        assert_eq!(arguments.output, None);
    }

    #[test]
    fn parses_explicit_output_path() {
        let arguments = parse_arguments(&[
            "program.zm".into(),
            "--output".into(),
            "program.bin".into(),
        ])
        .unwrap();

        assert_eq!(arguments.source, PathBuf::from("program.zm"));
        assert_eq!(
            arguments.output,
            Some(PathBuf::from("program.bin"))
        );
    }

    #[test]
    fn rejects_missing_source() {
        assert!(parse_arguments(&[]).is_err());
    }

    #[test]
    fn rejects_missing_output_value() {
        assert!(
            parse_arguments(&[
                "program.zm".into(),
                "--output".into(),
            ])
            .is_err()
        );
    }

    #[test]
    fn rejects_unknown_options() {
        assert!(
            parse_arguments(&[
                "program.zm".into(),
                "--target".into(),
            ])
            .is_err()
        );
    }

    #[test]
    fn derives_a_neutral_default_artifact_path() {
        assert_eq!(
            default_output_path(Path::new("program.zm")),
            PathBuf::from("program.out")
        );
    }
}

// SPDX-License-Identifier: Apache-2.0 OR MIT
//! `ls` — LeanSpec AST parser, architectural validator, and CAS listing engine CLI.

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use ls::{check_directory, format_report, parse_spec, ValidationReport};

fn print_usage() {
    println!("ls — LeanSpec AST parser & architectural validator");
    println!();
    println!("USAGE:");
    println!("    ls check [DIR]       Validate specifications in DIR (default: specs/)");
    println!("    ls list [DIR]        List specifications with metadata in DIR (default: specs/)");
    println!("    ls parse <FILE>      Parse single specification file into JSON AST");
    println!("    ls --version, -v     Print version information");
    println!("    ls --help, -h        Print this help message");
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return ExitCode::SUCCESS;
    }

    match args[1].as_str() {
        "--version" | "-v" => {
            println!("ls {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        "--help" | "-h" => {
            print_usage();
            ExitCode::SUCCESS
        }
        "check" => {
            let target_dir = args.get(2).map(String::as_str).unwrap_or("specs");
            let path = Path::new(target_dir);

            match check_directory(path) {
                Ok((result, formatted)) => {
                    print!("{formatted}");
                    if result.report.is_clean() {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    }
                }
                Err(err) => {
                    eprintln!("ls: failed to scan directory '{}': {err}", path.display());
                    ExitCode::FAILURE
                }
            }
        }
        "list" => {
            let target_dir = args.get(2).map(String::as_str).unwrap_or("specs");
            let path = Path::new(target_dir);

            match ls::scan_specs_dir(path) {
                Ok(result) => {
                    println!(
                        "{:<6} {:<32} {:<12} {:<10} {:<6}",
                        "NUM", "SLUG", "STATUS", "PRIORITY", "LINES"
                    );
                    println!("{:-<6} {:-<32} {:-<12} {:-<10} {:-<6}", "", "", "", "", "");
                    for doc in &result.documents {
                        let priority = doc
                            .frontmatter
                            .priority
                            .map(|p| format!("{p:?}"))
                            .unwrap_or_else(|| "-".to_owned());
                        println!(
                            "{:<6} {:<32} {:<12} {:<10} {:<6}",
                            doc.number,
                            doc.slug,
                            doc.frontmatter.status.as_str(),
                            priority,
                            doc.line_count
                        );
                    }
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("ls: failed to scan directory '{}': {err}", path.display());
                    ExitCode::FAILURE
                }
            }
        }
        "parse" => {
            let Some(file_path) = args.get(2) else {
                eprintln!("ls: parse requires a file path argument");
                return ExitCode::FAILURE;
            };
            let path = Path::new(file_path);
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(file_path);

            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(err) => {
                    eprintln!("ls: cannot read file '{}': {err}", path.display());
                    return ExitCode::FAILURE;
                }
            };

            let mut report = ValidationReport::default();
            match parse_spec(file_name, &content, &mut report) {
                Some(doc) => {
                    match serde_json::to_string_pretty(&doc) {
                        Ok(json) => {
                            println!("{json}");
                            ExitCode::SUCCESS
                        }
                        Err(err) => {
                            eprintln!("ls: json serialization error: {err}");
                            ExitCode::FAILURE
                        }
                    }
                }
                None => {
                    eprintln!("ls: failed to parse spec '{}'", file_name);
                    eprint!("{}", format_report(&report));
                    ExitCode::FAILURE
                }
            }
        }
        unknown => {
            eprintln!("ls: unrecognized command '{unknown}'");
            print_usage();
            ExitCode::FAILURE
        }
    }
}

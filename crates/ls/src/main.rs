// SPDX-License-Identifier: Apache-2.0 OR MIT
//! `ls` — LeanSpec AST parser, architectural validator, and CAS listing engine CLI.

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use ls::{
    check_directory, format_report, parse_spec, scan_cas_path, scan_specs_dir, ValidationReport,
};

fn print_usage() {
    println!("ls — LeanSpec AST parser, architectural validator & CAS listing engine");
    println!();
    println!("USAGE:");
    println!("    ls check [DIR]        Validate specifications in DIR (default: specs/)");
    println!(
        "    ls hash [PATH]        Compute BLAKE3 CAS hash of file or directory (default: specs/)"
    );
    println!(
        "    ls list [DIR]         List specifications with metadata in DIR (default: specs/)"
    );
    println!("    ls parse <FILE>       Parse single specification file into JSON AST");
    println!("    ls theorems [PATH]    List formal theorem statements & verification status");
    println!("    ls --version, -v      Print version information");
    println!("    ls --help, -h         Print this help message");
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
        "hash" => {
            let target_path = args.get(2).map(String::as_str).unwrap_or("specs");
            let path = Path::new(target_path);

            match scan_cas_path(path) {
                Ok(digest) => {
                    println!("{digest}  {}", path.display());
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!(
                        "ls: cannot compute CAS hash for '{}': {err}",
                        path.display()
                    );
                    ExitCode::FAILURE
                }
            }
        }
        "list" => {
            let target_dir = args.get(2).map(String::as_str).unwrap_or("specs");
            let path = Path::new(target_dir);

            match scan_specs_dir(path) {
                Ok(result) => {
                    println!(
                        "{:<6} {:<28} {:<12} {:<10} {:<8} {:<6} {:<16}",
                        "NUM", "SLUG", "STATUS", "PRIORITY", "THEOREMS", "LINES", "BLAKE3"
                    );
                    println!(
                        "{:-<6} {:-<28} {:-<12} {:-<10} {:-<8} {:-<6} {:-<16}",
                        "", "", "", "", "", "", ""
                    );
                    for doc in &result.documents {
                        let priority = doc
                            .frontmatter
                            .priority
                            .map(|p| format!("{p:?}"))
                            .unwrap_or_else(|| "-".to_owned());
                        let short_hash = if doc.content_hash.len() >= 16 {
                            &doc.content_hash[..16]
                        } else {
                            &doc.content_hash
                        };
                        println!(
                            "{:<6} {:<28} {:<12} {:<10} {:<8} {:<6} {:<16}",
                            doc.number,
                            doc.slug,
                            doc.frontmatter.status.as_str(),
                            priority,
                            doc.theorems.len(),
                            doc.line_count,
                            short_hash,
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
                Some(doc) => match serde_json::to_string_pretty(&doc) {
                    Ok(json) => {
                        println!("{json}");
                        ExitCode::SUCCESS
                    }
                    Err(err) => {
                        eprintln!("ls: json serialization error: {err}");
                        ExitCode::FAILURE
                    }
                },
                None => {
                    eprintln!("ls: failed to parse spec '{}'", file_name);
                    eprint!("{}", format_report(&report));
                    ExitCode::FAILURE
                }
            }
        }
        "theorems" => {
            let target_path = args.get(2).map(String::as_str).unwrap_or("specs");
            let path = Path::new(target_path);

            if path.is_file() {
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(target_path);
                let content = match fs::read_to_string(path) {
                    Ok(c) => c,
                    Err(err) => {
                        eprintln!("ls: cannot read file '{}': {err}", path.display());
                        return ExitCode::FAILURE;
                    }
                };
                let mut report = ValidationReport::default();
                if let Some(doc) = parse_spec(file_name, &content, &mut report) {
                    print_doc_theorems(&doc);
                    ExitCode::SUCCESS
                } else {
                    eprintln!("ls: failed to parse specification '{}'", file_name);
                    ExitCode::FAILURE
                }
            } else {
                match scan_specs_dir(path) {
                    Ok(result) => {
                        for doc in &result.documents {
                            print_doc_theorems(doc);
                        }
                        ExitCode::SUCCESS
                    }
                    Err(err) => {
                        eprintln!("ls: failed to scan directory '{}': {err}", path.display());
                        ExitCode::FAILURE
                    }
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

fn print_doc_theorems(doc: &ls::SpecDocument) {
    if doc.theorems.is_empty() {
        return;
    }
    println!(
        "Specification: {} ({})",
        doc.stem,
        doc.frontmatter.status.as_str()
    );
    println!(
        "{:<28} {:<12} {:<10} {:<30}",
        "THEOREM", "STATUS", "TACTICS", "SIGNATURE"
    );
    println!("{:-<28} {:-<12} {:-<10} {:-<30}", "", "", "", "");
    for th in &doc.theorems {
        let tactics_str = if th.tactics.is_empty() {
            "-".to_owned()
        } else {
            th.tactics.join(",")
        };
        let status_str = match th.proof_status {
            ls::ProofStatus::Proved { tactic_count } => format!("proved({tactic_count})"),
            ls::ProofStatus::Sorry => "sorry".to_owned(),
            ls::ProofStatus::Axiom => "axiom".to_owned(),
            ls::ProofStatus::Omitted => "omitted".to_owned(),
        };
        println!(
            "{:<28} {:<12} {:<10} {:<30}",
            th.name, status_str, tactics_str, th.signature
        );
    }
    println!();
}

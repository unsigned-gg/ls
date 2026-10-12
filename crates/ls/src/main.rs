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
    println!("    ls pf interp <FILE>   Interpolate projectfile expressions (§3.8)");
    println!("    ls pf ci resolve <F>  Resolve org.projectfile.ci intent DAG");
    println!("    ls pf ci generate <F> Lower DAG to Justfile (--target just)");
    println!("    ls pf image tags <F>  Derive org.projectfile.image variant tags");
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
        "pf" => handle_projectfile_command(&args[2..]),
        unknown => {
            eprintln!("ls: unrecognized command '{unknown}'");
            print_usage();
            ExitCode::FAILURE
        }
    }
}

fn handle_projectfile_command(args: &[String]) -> ExitCode {
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        println!("ls pf — Projectfile v1 normative engine & DAG compiler");
        println!();
        println!("USAGE:");
        println!("    ls pf interp <FILE> [--expr <EXPR>]    Interpolate projectfile (§3.8)");
        println!("    ls pf ci resolve <FILE>                Resolve org.projectfile.ci DAG");
        println!("    ls pf ci generate <FILE> [--target just] Lower DAG to Justfile");
        println!("    ls pf image tags <FILE>                Derive org.projectfile.image tags");
        return ExitCode::SUCCESS;
    }

    match args[0].as_str() {
        "interp" => {
            let Some(file_path) = args.get(1) else {
                eprintln!("ls pf: interp requires a file path argument");
                return ExitCode::FAILURE;
            };
            let content = match fs::read_to_string(file_path) {
                Ok(c) => c,
                Err(err) => {
                    eprintln!("ls pf: cannot read file '{file_path}': {err}");
                    return ExitCode::FAILURE;
                }
            };
            let val: serde_json::Value = match serde_yaml::from_str(&content) {
                Ok(v) => v,
                Err(err) => {
                    eprintln!("ls pf: cannot parse YAML in '{file_path}': {err}");
                    return ExitCode::FAILURE;
                }
            };
            let interpolator = ls::ProjectfileInterpolator::new(&val);
            if args.len() >= 4 && args[2] == "--expr" {
                let expr = &args[3];
                let out = interpolator.interpolate_string(expr, None, 0);
                println!("{out}");
            } else {
                let lines = interpolator.interpolate_lines(&content);
                for line in lines {
                    println!("{line}");
                }
            }
            ExitCode::SUCCESS
        }
        "ci" => {
            if args.len() < 2 {
                eprintln!("ls pf ci: requires a subcommand: 'resolve' or 'generate'");
                return ExitCode::FAILURE;
            }
            match args[1].as_str() {
                "resolve" => {
                    let Some(file_path) = args.get(2) else {
                        eprintln!("ls pf ci resolve: requires a file path argument");
                        return ExitCode::FAILURE;
                    };
                    let content = match fs::read_to_string(file_path) {
                        Ok(c) => c,
                        Err(err) => {
                            eprintln!("ls pf ci resolve: cannot read file '{file_path}': {err}");
                            return ExitCode::FAILURE;
                        }
                    };
                    let val: serde_json::Value = match serde_yaml::from_str(&content) {
                        Ok(v) => v,
                        Err(err) => {
                            eprintln!(
                                "ls pf ci resolve: cannot parse YAML in '{file_path}': {err}"
                            );
                            return ExitCode::FAILURE;
                        }
                    };
                    let res = ls::CiResolver::resolve_fixture(&val);
                    let json = match serde_json::to_string_pretty(&res) {
                        Ok(j) => j,
                        Err(err) => {
                            eprintln!("ls pf ci resolve: json serialization error: {err}");
                            return ExitCode::FAILURE;
                        }
                    };
                    println!("{json}");
                    ExitCode::SUCCESS
                }
                "generate" => {
                    let Some(file_path) = args.get(2) else {
                        eprintln!("ls pf ci generate: requires a file path argument");
                        return ExitCode::FAILURE;
                    };
                    let content = match fs::read_to_string(file_path) {
                        Ok(c) => c,
                        Err(err) => {
                            eprintln!("ls pf ci generate: cannot read file '{file_path}': {err}");
                            return ExitCode::FAILURE;
                        }
                    };
                    let val: serde_json::Value = match serde_yaml::from_str(&content) {
                        Ok(v) => v,
                        Err(err) => {
                            eprintln!(
                                "ls pf ci generate: cannot parse YAML in '{file_path}': {err}"
                            );
                            return ExitCode::FAILURE;
                        }
                    };
                    let res = ls::CiResolver::resolve_fixture(&val);
                    if let Some(ref reject) = res.reject {
                        eprintln!("ls pf ci generate: DAG rejected with code: '{reject}'");
                        return ExitCode::FAILURE;
                    }
                    let options = ls::JustTargetOptions::default();
                    let out = ls::JustfileCompiler::compile_from_doc(&val, &res, &options);
                    print!("{out}");
                    ExitCode::SUCCESS
                }
                unknown => {
                    eprintln!("ls pf ci: unrecognized subcommand '{unknown}'");
                    ExitCode::FAILURE
                }
            }
        }
        "image" => {
            if args.len() < 2 {
                eprintln!("ls pf image: requires a subcommand: 'tags'");
                return ExitCode::FAILURE;
            }
            match args[1].as_str() {
                "tags" => {
                    let Some(file_path) = args.get(2) else {
                        eprintln!("ls pf image tags: requires a file path argument");
                        return ExitCode::FAILURE;
                    };
                    let content = match fs::read_to_string(file_path) {
                        Ok(c) => c,
                        Err(err) => {
                            eprintln!("ls pf image tags: cannot read file '{file_path}': {err}");
                            return ExitCode::FAILURE;
                        }
                    };
                    let val: serde_json::Value = match serde_yaml::from_str(&content) {
                        Ok(v) => v,
                        Err(err) => {
                            eprintln!(
                                "ls pf image tags: cannot parse YAML in '{file_path}': {err}"
                            );
                            return ExitCode::FAILURE;
                        }
                    };
                    let res = ls::ImageResolver::resolve_fixture(&val);
                    let json = match serde_json::to_string_pretty(&res) {
                        Ok(j) => j,
                        Err(err) => {
                            eprintln!("ls pf image tags: json serialization error: {err}");
                            return ExitCode::FAILURE;
                        }
                    };
                    println!("{json}");
                    ExitCode::SUCCESS
                }
                unknown => {
                    eprintln!("ls pf image: unrecognized subcommand '{unknown}'");
                    ExitCode::FAILURE
                }
            }
        }
        unknown => {
            eprintln!("ls pf: unrecognized command '{unknown}'");
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

// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Rust `xtask` Lowering Compiler for `org.projectfile.ci`.
//!
//! Generates a deterministic, zero-external-dependency Rust `xtask` runner
//! from a resolved CI intent DAG (`CiResolveResult` and `projectfile.yaml` AST).
//!
//! # Invariants
//! - Zero-I/O during compilation: operates purely on in-memory strings and serde values.
//! - Deterministic output formatting and topological dependency ordering.
//! - Enforces host throttling (`-j 2` / `JOBS`) on generated Cargo invocations.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

use super::ci::CiResolveResult;

/// Configuration options for Rust `xtask` code generation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XtaskTargetOptions {
    /// Default concurrent job limit on shared hosts (default: `2`).
    pub default_jobs: usize,
    /// Default aggregate function / subcommand name (default: `"xwork"`).
    pub default_target: String,
    /// Whether to emit provenance header comments.
    pub emit_preamble: bool,
    /// Emit a dispatching runner instead of one that spawns each stage as a program.
    ///
    /// The generated `run_xwork` then takes a closure that runs one stage, so the stage
    /// names in the projectfile stay the single source of order while the host `xtask`
    /// keeps the implementation of every stage. The generated module spawns no process.
    pub dispatch: bool,
}

impl Default for XtaskTargetOptions {
    fn default() -> Self {
        Self {
            default_jobs: 2,
            default_target: "xwork".to_string(),
            emit_preamble: true,
            dispatch: false,
        }
    }
}

/// The dispatch-mode runner (SPEC-0003 requirements 9, 10). `@TARGET_DOC@` is replaced with
/// the goal escaped for a doc comment, `@TARGET@` with the goal escaped for a format string,
/// `@IDENT@` with its identifier and `@TABLE@` with the stage table name.
const DISPATCH_RUNNER: &str = r#"/// Execute the `@TARGET_DOC@` verification pipeline in topological DAG order.
///
/// `run_stage` runs one stage and returns its exit code. The first stage
/// that does not succeed stops the pipeline. Then one summary line follows
/// for each stage, and one verdict line.
pub fn run_@IDENT@<F>(root: &Path, mut run_stage: F) -> ExitCode
where
    F: FnMut(&Path, &str, &[&str]) -> ExitCode,
{
    let width = @TABLE@
        .iter()
        .map(|(stage, _)| stage.len())
        .max()
        .unwrap_or(0);
    let mut lines = Vec::with_capacity(@TABLE@.len());
    let mut failed = None;
    for &(stage, args) in @TABLE@ {
        if failed.is_some() {
            lines.push(format!("@TARGET@: not run {stage}"));
            continue;
        }
        println!("xtask: [{stage}] running...");
        let start = Instant::now();
        let ok = run_stage(root, stage, args) == ExitCode::SUCCESS;
        let secs = start.elapsed().as_secs_f64();
        if ok {
            lines.push(format!("@TARGET@: ok      {stage:<width$}  {secs:.1}s"));
        } else {
            eprintln!("xtask: [{stage}] FAILED");
            lines.push(format!("@TARGET@: FAILED  {stage:<width$}  {secs:.1}s"));
            failed = Some(stage);
        }
    }
    for line in &lines {
        println!("{line}");
    }
    match failed {
        Some(stage) => {
            println!("@TARGET@: FAILED at {stage}");
            ExitCode::FAILURE
        }
        None => {
            println!("@TARGET@: ok ({} stages)", @TABLE@.len());
            ExitCode::SUCCESS
        }
    }
}
"#;

/// Spawn mode: drop the variables `cargo run` sets for the xtask package, so a
/// stage does not take itself for that package (SPEC-0003 requirement 13).
const SPAWN_ENV_SCRUB: &str = r#"        for (key, _) in env::vars_os() {
            let Some(key) = key.to_str() else {
                continue;
            };
            if key.starts_with("CARGO_PKG_")
                || key == "CARGO_MANIFEST_DIR"
                || key == "CARGO_MANIFEST_PATH"
            {
                command.env_remove(key);
            }
        }
"#;

/// Zero-I/O Rust `xtask` Lowering Compiler.
pub struct XtaskCompiler;

impl XtaskCompiler {
    /// Compiles a resolved CI execution plan into a deterministic Rust `xtask` module.
    #[must_use]
    pub fn compile(resolved: &CiResolveResult, options: &XtaskTargetOptions) -> String {
        Self::compile_with_tools(resolved, options, &[])
    }

    fn compile_with_tools(
        resolved: &CiResolveResult,
        options: &XtaskTargetOptions,
        tools: &[(String, String, String)],
    ) -> String {
        let mut out = String::new();
        let fn_ident = Self::sanitize_ident(&options.default_target).to_ascii_lowercase();
        let upper = fn_ident.to_ascii_uppercase();
        // A constant name cannot start with a digit; `run_<ident>` always can.
        let table = if upper.starts_with(|c: char| c.is_ascii_digit()) {
            format!("GOAL_{upper}_STAGES")
        } else {
            format!("{upper}_STAGES")
        };
        let target_doc = Self::literal_body(&options.default_target);

        if options.emit_preamble {
            out.push_str("// SPDX-License-Identifier: Apache-2.0 OR MIT\n");
            out.push_str(
                "// Code generated by ls pf ci (LeanSpec Zero-I/O Lowering Engine). DO NOT EDIT.\n",
            );
            out.push_str("// Specification: https://projectfile.org/spec/v1\n\n");
        }

        if options.dispatch {
            out.push_str("use std::path::Path;\n");
            out.push_str(concat!("use std::", "process::ExitCode;\n"));
            out.push_str("use std::time::Instant;\n\n");
        } else {
            out.push_str("use std::env;\n");
            out.push_str("use std::path::Path;\n");
            out.push_str(concat!("use std::", "process::{Command, ExitCode};\n\n"));
        }

        if options.dispatch {
            out.push_str("/// Default concurrent job limit on shared hosts.\n");
        }
        out.push_str(&format!(
            "pub const DEFAULT_JOBS: usize = {};\n\n",
            options.default_jobs
        ));

        if !tools.is_empty() {
            out.push_str(
                "/// Pinned tools from `gg.unsigned.ls.tools`: (crate, version, binary).\n",
            );
            out.push_str("pub const TOOLS: &[(&str, &str, &str)] = &[\n");
            for (krate, version, bin) in tools {
                out.push_str(&format!("    ({krate:?}, {version:?}, {bin:?}),\n"));
            }
            out.push_str("];\n\n");
        }

        // Map prerequisites for each tool from before-pairs
        let mut tool_deps: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        for tool in &resolved.runs {
            tool_deps.entry(tool.as_str()).or_default();
        }

        for (first, second) in &resolved.before {
            if let Some(deps) = tool_deps.get_mut(second.as_str()) {
                deps.insert(first.as_str());
            }
        }

        // Ordered stage table
        out.push_str("/// Topologically ordered CI stages lowered from `org.projectfile.ci`.\n");
        out.push_str(&format!("pub const {table}: &[(&str, &[&str])] = &[\n"));
        for tool in &resolved.runs {
            let arg_list = Self::split_args(resolved.args.get(tool).map(String::as_str));
            let formatted_args: Vec<String> = arg_list.iter().map(|a| format!("{a:?}")).collect();
            out.push_str(&format!(
                "    ({tool:?}, &[{}]),\n",
                formatted_args.join(", ")
            ));
        }
        out.push_str("];\n\n");

        // Runner function
        if options.dispatch {
            out.push_str(
                &DISPATCH_RUNNER
                    .replace("@TARGET_DOC@", &target_doc)
                    .replace("@TARGET@", &target_doc.replace('{', "{{").replace('}', "}}"))
                    .replace("@IDENT@", &fn_ident)
                    .replace("@TABLE@", &table),
            );
            return out;
        }
        out.push_str(&format!(
            "/// Execute the `{target_doc}` verification pipeline in topological DAG order.\n"
        ));
        out.push_str(&format!(
            "pub fn run_{fn_ident}(root: &Path) -> ExitCode {{\n"
        ));
        out.push_str("    let jobs = env::var(\"JOBS\")\n");
        out.push_str("        .or_else(|_| env::var(\"CARGO_BUILD_JOBS\"))\n");
        out.push_str("        .unwrap_or_else(|_| DEFAULT_JOBS.to_string());\n\n");
        out.push_str(&format!("    for &(tool, args) in {table} {{\n"));
        out.push_str("        println!(\"xtask: [{tool}] running...\");\n");
        out.push_str("        let mut command = Command::new(tool);\n");
        out.push_str("        command\n");
        out.push_str("            .args(args)\n");
        out.push_str("            .env(\"JOBS\", &jobs)\n");
        out.push_str("            .env(\"CARGO_BUILD_JOBS\", &jobs)\n");
        out.push_str("            .current_dir(root);\n");
        out.push_str(SPAWN_ENV_SCRUB);
        out.push_str("        let status = command.status();\n");
        out.push_str("        match status {\n");
        out.push_str("            Ok(s) if s.success() => {}\n");
        out.push_str("            Ok(_) => {\n");
        out.push_str("                eprintln!(\"xtask: [{tool}] FAILED\");\n");
        out.push_str("                return ExitCode::FAILURE;\n");
        out.push_str("            }\n");
        out.push_str("            Err(err) => {\n");
        out.push_str("                eprintln!(\"xtask: cannot execute {tool}: {err}\");\n");
        out.push_str("                return ExitCode::FAILURE;\n");
        out.push_str("            }\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
        out.push_str("    ExitCode::SUCCESS\n");
        out.push_str("}\n");

        out
    }

    /// Compiles a projectfile document AST and resolved DAG into a Rust `xtask` module.
    ///
    /// `resolved` is lowered as given. An empty closure (for example a `--goal` node
    /// with no needs) gives an empty stage table, never the flagged goals of `doc`.
    #[must_use]
    pub fn compile_from_doc(
        doc: &Value,
        resolved: &CiResolveResult,
        options: &XtaskTargetOptions,
    ) -> String {
        let tools = Self::tools_from_doc(doc);
        Self::compile_with_tools(resolved, options, &tools)
    }

    /// The rows of `gg.unsigned.ls.tools` in name order (SPEC-0003 requirement 14).
    /// A row without a string `crate`, `version` and `bin` is left out.
    fn tools_from_doc(doc: &Value) -> Vec<(String, String, String)> {
        let Some(tools) = doc
            .get("gg")
            .and_then(|g| g.get("unsigned"))
            .and_then(|u| u.get("ls"))
            .and_then(|l| l.get("tools"))
            .and_then(Value::as_object)
        else {
            return Vec::new();
        };
        tools
            .values()
            .filter_map(|row| {
                let field = |key: &str| row.get(key).and_then(Value::as_str).map(str::to_string);
                Some((field("crate")?, field("version")?, field("bin")?))
            })
            .collect()
    }

    fn split_args(raw: Option<&str>) -> Vec<String> {
        match raw {
            Some(s) if !s.trim().is_empty() => {
                s.split_whitespace().map(ToString::to_string).collect()
            }
            _ => Vec::new(),
        }
    }

    /// `raw` as the body of a Rust string literal: quotes, backslashes and control
    /// characters escaped, without the surrounding quotes.
    fn literal_body(raw: &str) -> String {
        let quoted = format!("{raw:?}");
        quoted[1..quoted.len() - 1].to_string()
    }

    fn sanitize_ident(raw: &str) -> String {
        raw.chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn compile_generates_deterministic_rust_xtask_module() {
        let doc = json!({
            "nodes": {
                "hygiene": {
                    "needs": {
                        "toolchain-check": true,
                        "spec-check": true
                    }
                },
                "check": {
                    "goal": true,
                    "needs": {
                        "hygiene": true,
                        "cargo-clippy": { "args": "--workspace -j 2 -- -D warnings" }
                    }
                }
            }
        });
        let resolved = super::super::ci::CiResolver::resolve_fixture(&doc);
        let opts = XtaskTargetOptions::default();
        let code = XtaskCompiler::compile_from_doc(&doc, &resolved, &opts);

        assert!(code.contains("pub const DEFAULT_JOBS: usize = 2;"));
        assert!(code.contains("pub const XWORK_STAGES: &[(&str, &[&str])]"));
        assert!(code.contains("(\"toolchain-check\", &[])"));
        assert!(code.contains("(\"spec-check\", &[])"));
        assert!(code.contains(
            "(\"cargo-clippy\", &[\"--workspace\", \"-j\", \"2\", \"--\", \"-D\", \"warnings\"])"
        ));
        assert!(code.contains("pub fn run_xwork(root: &Path) -> ExitCode"));
    }

    fn sample_doc() -> Value {
        json!({
            "nodes": {
                "hygiene": { "needs": { "toolchain-check": true, "spec-check": true } },
                "check": {
                    "goal": true,
                    "needs": {
                        "hygiene": true,
                        "cargo-fmt": { "args": "--all -- --check" }
                    }
                }
            }
        })
    }

    #[test]
    fn dispatch_mode_emits_closure_runner_and_spawns_nothing() {
        let doc = sample_doc();
        let resolved = super::super::ci::CiResolver::resolve_fixture(&doc);
        let opts = XtaskTargetOptions {
            dispatch: true,
            ..XtaskTargetOptions::default()
        };
        let code = XtaskCompiler::compile_from_doc(&doc, &resolved, &opts);

        assert!(code.contains("pub fn run_xwork<F>(root: &Path, mut run_stage: F) -> ExitCode"));
        assert!(code.contains("F: FnMut(&Path, &str, &[&str]) -> ExitCode,"));
        assert!(code.contains("(\"cargo-fmt\", &[\"--all\", \"--\", \"--check\"])"));
        assert!(
            !code.contains("Command"),
            "dispatch mode must not spawn a process"
        );
        assert!(
            !code.contains("std::env"),
            "dispatch mode must not read the environment"
        );
    }

    #[test]
    fn dispatch_mode_keeps_dependency_order() {
        let doc = sample_doc();
        let resolved = super::super::ci::CiResolver::resolve_fixture(&doc);
        let opts = XtaskTargetOptions {
            dispatch: true,
            ..XtaskTargetOptions::default()
        };
        let code = XtaskCompiler::compile_from_doc(&doc, &resolved, &opts);
        let spec = code.find("\"spec-check\"").expect("spec-check stage");
        let fmt = code.find("\"cargo-fmt\"").expect("cargo-fmt stage");
        assert!(
            spec < fmt,
            "hygiene stages must precede the goal's own tools"
        );
    }

    fn dispatch() -> XtaskTargetOptions {
        XtaskTargetOptions {
            dispatch: true,
            ..XtaskTargetOptions::default()
        }
    }

    // SPEC-0003 requirement 10: dispatch mode prints one summary line per stage.
    #[test]
    fn dispatch_mode_prints_a_summary_line_for_each_stage() {
        let doc = sample_doc();
        let resolved = super::super::ci::CiResolver::resolve_fixture(&doc);
        let code = XtaskCompiler::compile_from_doc(&doc, &resolved, &dispatch());
        for needle in [
            "ok     ",
            "FAILED ",
            "not run",
            "FAILED at",
            "Instant::now()",
        ] {
            assert!(code.contains(needle), "dispatch output lacks `{needle}`");
        }
        assert!(
            !code.contains("Command"),
            "dispatch mode must not spawn a process"
        );
        assert!(
            !code.contains("std::env"),
            "dispatch mode must not read the environment"
        );
    }

    // SPEC-0003 requirement 11: the table and the runner take the goal's name.
    #[test]
    fn stage_table_and_runner_take_the_goal_name() {
        let doc = sample_doc();
        let resolved = super::super::ci::CiResolver::resolve_fixture(&doc);
        let opts = XtaskTargetOptions {
            default_target: "install".to_string(),
            ..dispatch()
        };
        let code = XtaskCompiler::compile_from_doc(&doc, &resolved, &opts);
        assert!(code.contains("pub const INSTALL_STAGES: &[(&str, &[&str])]"));
        assert!(code.contains("pub fn run_install<F>("));
        assert!(code.contains("in INSTALL_STAGES"));
        assert!(!code.contains("XWORK_STAGES"));
        let default = XtaskCompiler::compile_from_doc(&doc, &resolved, &dispatch());
        assert!(
            default.contains("pub const XWORK_STAGES"),
            "default name changed"
        );
    }

    // SPEC-0003 requirement 11: a goal with an empty closure lowers to an empty table,
    // never to the flagged goals of the same document.
    #[test]
    fn goal_with_an_empty_closure_emits_an_empty_table() {
        let ci = json!({
            "nodes": {
                "noop": { "needs": {} },
                "check": { "goal": true, "needs": { "cargo-test": true } }
            }
        });
        let doc = json!({ "org": { "projectfile": { "ci": ci.clone() } } });
        let resolved = super::super::ci::CiResolver::resolve_goal(&ci, "noop");
        assert_eq!(resolved.reject, None);
        assert!(resolved.runs.is_empty());
        let opts = XtaskTargetOptions {
            default_target: "noop".to_string(),
            ..dispatch()
        };
        let code = XtaskCompiler::compile_from_doc(&doc, &resolved, &opts);
        assert!(
            code.contains("pub const NOOP_STAGES: &[(&str, &[&str])] = &[\n];"),
            "the noop table is not empty:\n{code}"
        );
        assert!(!code.contains("cargo-test"), "the flagged goal leaked in");
    }

    // SPEC-0003 requirement 11: a goal name reaches the generated code as data. Braces,
    // quotes and a leading digit still give a module that compiles.
    #[test]
    fn goal_names_are_escaped_in_generated_code() {
        let doc = sample_doc();
        let resolved = super::super::ci::CiResolver::resolve_fixture(&doc);
        let named = |goal: &str| {
            let opts = XtaskTargetOptions {
                default_target: goal.to_string(),
                ..dispatch()
            };
            XtaskCompiler::compile_from_doc(&doc, &resolved, &opts)
        };
        let braces = named("ci{x}\"");
        assert!(
            braces.contains(r#"format!("ci{{x}}\": not run {stage}")"#),
            "format strings are not escaped:\n{braces}"
        );
        assert!(braces.contains("pub const CI_X___STAGES"), "{braces}");
        let digit = named("2fa");
        assert!(digit.contains("pub const GOAL_2FA_STAGES"), "{digit}");
        assert!(digit.contains("pub fn run_2fa<F>("));
        let upper = named("Install");
        assert!(upper.contains("pub fn run_install<F>("), "{upper}");
    }

    // SPEC-0003 requirement 13: spawn mode drops the cargo run package variables.
    #[test]
    fn spawn_mode_removes_the_cargo_run_package_variables() {
        let doc = sample_doc();
        let resolved = super::super::ci::CiResolver::resolve_fixture(&doc);
        let code = XtaskCompiler::compile_from_doc(&doc, &resolved, &XtaskTargetOptions::default());
        for needle in [
            "env_remove",
            "\"CARGO_PKG_\"",
            "\"CARGO_MANIFEST_DIR\"",
            "\"CARGO_MANIFEST_PATH\"",
        ] {
            assert!(code.contains(needle), "spawn output lacks `{needle}`");
        }
    }

    // SPEC-0003 requirement 14: gg.unsigned.ls.tools becomes a TOOLS table in name order.
    #[test]
    fn tools_section_becomes_a_tools_table() {
        let doc = json!({
            "gg": { "unsigned": { "ls": { "tools": {
                "nextest": { "crate": "cargo-nextest", "version": "0.9.104", "bin": "cargo-nextest" },
                "deny": { "crate": "cargo-deny", "version": "0.19.9", "bin": "cargo-deny" }
            } } } },
            "org": { "projectfile": { "ci": sample_doc() } }
        });
        let resolved = super::super::ci::CiResolver::resolve_fixture(&sample_doc());
        let code = XtaskCompiler::compile_from_doc(&doc, &resolved, &dispatch());
        assert!(code.contains("pub const TOOLS: &[(&str, &str, &str)] = &["));
        let deny = code.find("(\"cargo-deny\", \"0.19.9\", \"cargo-deny\")");
        let nextest = code.find("(\"cargo-nextest\", \"0.9.104\", \"cargo-nextest\")");
        assert!(deny.is_some() && nextest.is_some(), "missing rows:\n{code}");
        assert!(deny < nextest, "rows are not in name order");
        let none = XtaskCompiler::compile_from_doc(&sample_doc(), &resolved, &dispatch());
        assert!(!none.contains("TOOLS"), "no tools section, no table");
    }

    // SPEC-0003 requirement 14: a tool row without crate, version or bin is left out.
    #[test]
    fn tools_rows_need_crate_version_and_bin() {
        let doc = json!({
            "gg": { "unsigned": { "ls": { "tools": {
                "broken": { "crate": "cargo-deny", "version": "0.19.9" }
            } } } },
            "org": { "projectfile": { "ci": sample_doc() } }
        });
        let resolved = super::super::ci::CiResolver::resolve_fixture(&sample_doc());
        let code = XtaskCompiler::compile_from_doc(&doc, &resolved, &dispatch());
        assert!(
            !code.contains("(\"cargo-deny\""),
            "an incomplete row was emitted"
        );
    }

    #[test]
    fn default_mode_still_spawns_each_stage() {
        let doc = sample_doc();
        let resolved = super::super::ci::CiResolver::resolve_fixture(&doc);
        let code = XtaskCompiler::compile_from_doc(&doc, &resolved, &XtaskTargetOptions::default());
        assert!(code.contains("Command::new(tool)"));
        assert!(!code.contains("run_stage"));
    }
}

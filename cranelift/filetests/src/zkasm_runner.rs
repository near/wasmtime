//! zkASM code runner

use serde_derive::Deserialize;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use tempfile::{NamedTempFile, TempDir};

/// Counters consumed during the execution of zkAsm program.
/// Matches json structure returned by zkAsm interpreter.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Counters {
    /// Number of ARITH opcodes used.
    pub cnt_arith: String,
    /// Number of BINARY opcodes used.
    pub cnt_binary: String,
    /// Number of KECCAK opcodes used.
    pub cnt_keccak_f: String,
    /// Number of MEM_ALIGN opcodes used.
    pub cnt_mem_align: String,
    /// Total number of execution steps.
    pub cnt_steps: u64,
}

/// Status of the execution of zkAsm program.
/// Matches json structure returned by zkAsm interpreter.
#[derive(Deserialize, Debug)]
pub enum ExecutionStatus {
    /// Program ran till completion.
    #[serde(rename = "pass")]
    Success,
    /// Program failed during compilation or execution.
    #[serde(rename = "runtime error")]
    RuntimeError,
}

/// Result of the execution of zkAsm program.
#[derive(Deserialize)]
pub struct ExecutionResult {
    /// Path to the main zkAsm file that was executed.
    pub path: String,
    /// Status of the execution.
    pub status: ExecutionStatus,
    /// Error message in case the execution failed.
    pub error: Option<String>,
    /// Profiling information about this execution.
    /// Only populated for the successful executions.
    pub counters: Option<Counters>,
}

impl ExecutionResult {
    /// Pretty-prints the execution error message.
    pub fn format_error(&self) -> String {
        match &self.error {
            Some(s) => s.replace("\\n", "\n"),
            None => "None".to_string(),
        }
    }
}

/// Runs a given snippet of zkAsm code.
pub fn run_zkasm(contents: &str) -> anyhow::Result<ExecutionResult> {
    let tmp_dir = TempDir::new()?;
    let zkasm_file = tmp_dir.path().join("code.zkasm");
    std::fs::write(&zkasm_file, contents)?;
    let mut results = run_zkasm_path(&zkasm_file)?;
    assert_eq!(results.len(), 1);
    Ok(results.remove(0))
}

/// Sets up the helpers required to execute generated zkASM. Generates a subdirectory in the
/// provided `path`.
fn create_zkasm_helpers(path: &Path) -> anyhow::Result<()> {
    std::fs::create_dir(path.join("helpers"))?;
    let helpers_file = path.join("helpers/2-exp.zkasm");
    std::fs::write(
        helpers_file,
        include_str!("../../zkasm_data/generated/helpers/2-exp.zkasm"),
    )?;
    Ok(())
}

/// Returns the path to the Node module necessary to execute zkASM. No assumptions regarding the
/// current path of the caller are made.
fn node_module_path() -> String {
    // The node module necessary to execute zkAsm lives in `wasmtime/tests/zkasm/package.json`.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/zkasm/")
        .display()
        .to_string()
}

/// Generates a `Command` to launch `npm` in a manner compatible with the target OS.
fn new_npm_command() -> Command {
    #[cfg(target_os = "windows")]
    return std::process::Command::new("cmd").args(["/C", "npm"]);
    #[cfg(not(target_os = "windows"))]
    return std::process::Command::new("npm");
}

/// Runs zkAsm at a specified path.
/// If the directory path is passed, all zkAsm files in that directory will be executed.
pub fn run_zkasm_path(input_path: &Path) -> anyhow::Result<Vec<ExecutionResult>> {
    let dir_path = if input_path.is_dir() {
        input_path
    } else {
        input_path.parent().unwrap()
    };

    let mut output_file = NamedTempFile::new()?;
    create_zkasm_helpers(dir_path)?;
    let output = new_npm_command()
        .args([
            "--prefix",
            &node_module_path(),
            "test",
            input_path.to_str().unwrap(),
            output_file.path().to_str().unwrap(),
        ])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to run `npm`: {}; stderr: {}",
            output.status,
            String::from_utf8(output.stderr)?
        ));
    }

    let mut buf = String::new();
    output_file.read_to_string(&mut buf)?;
    if buf.is_empty() {
        return Ok(Vec::new());
    }
    let execution_results = serde_json::from_str(&buf)?;
    Ok(execution_results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_zkasm_success() {
        let code = r#"
start:
    2 + 3 => A
    5 => B
    B: ASSERT
finalizeExecution:
    ${beforeLast()}  :JMPN(finalizeExecution)
    :JMP(start)
        "#;
        let result = run_zkasm(&code).expect("Failed to run zkAsm");
        assert!(
            matches!(result.status, ExecutionStatus::Success),
            "Error: {}",
            result.format_error()
        );
    }

    #[test]
    fn test_run_zkasm_runtime_error() {
        let code = r#"
start:
    2 + 2 => A
    5 => B
    B: ASSERT
finalizeExecution:
    ${beforeLast()}  :JMPN(finalizeExecution)
    :JMP(start)
        "#;
        let result = run_zkasm(&code).expect("Failed to run zkAsm");
        assert!(
            matches!(result.status, ExecutionStatus::RuntimeError),
            "Error: {}",
            result.format_error()
        );
    }
}

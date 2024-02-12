use serde_derive::Deserialize;
use std::io::Read;
use std::path::Path;
use tempfile::{NamedTempFile, TempDir};

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Counters {
    cnt_arith: String,
    cnt_binary: String,
    cnt_keccak_f: String,
    cnt_mem_align: String,
    cnt_steps: u64,
}

#[derive(Deserialize, Debug)]
pub enum ExecutionStatus {
    #[serde(rename = "pass")]
    Success,
    #[serde(rename = "runtime error")]
    RuntimeError,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ExecutionResult {
    /// Path to the main zkAsm file that was executed.
    path: String,
    /// Status of the execution.
    status: ExecutionStatus,
    /// Error message in case the execution failed.
    error: Option<String>,
    /// Profiling information about this execution.
    /// Only populated for the successful executions.
    counters: Option<Counters>,
}

impl ExecutionResult {
    #[allow(dead_code)]
    fn format_error(&self) -> String {
        match &self.error {
            Some(s) => s.replace("\\n", "\n"),
            None => "None".to_string(),
        }
    }
}

/// Runs a given snippet of zkAsm code.
#[allow(dead_code)]
pub fn run_zkasm(contents: &str) -> anyhow::Result<ExecutionResult> {
    let tmp_dir = TempDir::new()?;
    let zkasm_file = tmp_dir.path().join("code.zkasm");
    std::fs::write(&zkasm_file, contents)?;
    let mut results = run_zkasm_path(&zkasm_file)?;
    assert_eq!(results.len(), 1);
    Ok(results.remove(0))
}

/// Runs zkAsm at a specified path.
/// If the directory path is passed, all zkAsm files in that directory will be executed.
#[allow(dead_code)]
pub fn run_zkasm_path(input_path: &Path) -> anyhow::Result<Vec<ExecutionResult>> {
    let dir_path = if input_path.is_dir() {
        input_path
    } else {
        input_path.parent().unwrap()
    };
    std::fs::create_dir(dir_path.join("helpers"))?;
    let helpers_file = dir_path.join("helpers/2-exp.zkasm");
    std::fs::write(
        helpers_file,
        include_str!("../../zkasm_data/generated/helpers/2-exp.zkasm"),
    )?;

    let mut output_file = NamedTempFile::new()?;
    let output = std::process::Command::new("npm")
        .args([
            "--prefix",
            "../../tests/zkasm",
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

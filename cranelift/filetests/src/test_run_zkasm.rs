//! Test command for compiling CLIF files to .zkasm, running it, and verifying their results
//!
//! using [RunCommand](cranelift_reader::RunCommand)s.

use crate::runone::FileUpdate;
use crate::subtest::SubTest;
use crate::zkasm_codegen;
use crate::zkasm_runner::{self, ExecutionStatus};
use cranelift_codegen::isa::TargetIsa;
use cranelift_codegen::settings::Flags;
use cranelift_codegen::{self, ir};
use cranelift_reader::{parse_run_command, RunCommand, TestCommand, TestFile};
use std::borrow::Cow;
use std::collections::HashMap;

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct TestResults {
    results: HashMap<String, String>,
}

struct TestRunZkasm;

pub fn subtest(parsed: &TestCommand) -> anyhow::Result<Box<dyn SubTest>> {
    assert_eq!(parsed.command, "run-zkasm");
    if !parsed.options.is_empty() {
        anyhow::bail!("No options allowed on {}", parsed);
    }
    Ok(Box::new(TestRunZkasm))
}

impl SubTest for TestRunZkasm {
    fn name(&self) -> &'static str {
        "run-zkasm"
    }

    fn is_mutating(&self) -> bool {
        false
    }

    fn needs_isa(&self) -> bool {
        false
    }

    /// Runs the entire subtest for a given target
    /// TODO: in zkasm we don't really run test for given target, we run for zkasm target
    fn run_target<'a>(
        &self,
        testfile: &TestFile,
        _: &mut FileUpdate,
        _: &'a str,
        _: &'a Flags,
        _: Option<&'a dyn TargetIsa>,
    ) -> anyhow::Result<()> {
        let mut zkasm_functions: Vec<Vec<String>> = Vec::new();
        let mut invocations: Vec<Vec<String>> = Vec::new();
        let mut invoke_names: Vec<String> = Vec::new();
        let mut expected_results: HashMap<String, String> = Default::default();
        for (func, details) in &testfile.functions {
            zkasm_functions.push(zkasm_codegen::compile_clif_function(func));
            for comment in details.comments.iter() {
                if let Some(command) = parse_run_command(comment.text, &func.signature)? {
                    match command {
                        RunCommand::Print(_) => {
                            todo!()
                        }
                        RunCommand::Run(invoke, compare, expected) => {
                            invoke_names.push(zkasm_codegen::invoke_name(&invoke));
                            expected_results.insert(
                                zkasm_codegen::invoke_name(&invoke),
                                expected[0].to_string(),
                            );
                            invocations
                                .push(zkasm_codegen::compile_invocation(invoke, compare, expected));
                        }
                    }
                }
            }
        }
        let main = zkasm_codegen::build_test_main(invoke_names);
        let zkasm_program = zkasm_codegen::build_test_zkasm(zkasm_functions, invocations, main);
        println!("{}", zkasm_program);
        let execution_result = zkasm_runner::run_zkasm(&zkasm_program).unwrap();
        match execution_result.error {
            Some(err) => panic!("Zkasm runtime error: {}", err),
            None => (),
        };
        // TODO: Maybe it is better to create "Unknown error" as error in case when
        // status is RuntimeError and current error is None, and remove this filed?
        debug_assert!(matches!(execution_result.status, ExecutionStatus::Success));
        let test_data = std::fs::read_to_string("../tests/zkasm/testoutput.json")?;
        let results: TestResults = serde_json::from_str(&test_data)?;
        let mut fail_report: String = Default::default();
        for (testname, status) in results.results {
            if status != "pass" {
                let fail_parsed: Vec<&str> = testname.split("__").collect();
                let args = fail_parsed[1..fail_parsed.len()].join(", ").replace("MINUS", "-");
                let fail_formatted = format!(
                    "Failed to calculate {}({}). Expected: {}. Actual:{}\n",
                    fail_parsed[0], args, expected_results[&testname], status
                );
                fail_report.push_str(&fail_formatted);
            }
        }
        if fail_report != "" {
            panic!("{}", fail_report);
        }
        println!("All tests in passed");
        Ok(())
    }

    fn run(
        &self,
        _func: Cow<ir::Function>,
        _context: &crate::subtest::Context,
    ) -> anyhow::Result<()> {
        unreachable!()
    }
}

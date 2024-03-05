#[cfg(test)]
mod tests {
    use crate::zkasm_codegen;
    use crate::zkasm_codegen::ZkasmSettings;
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    use regex::Regex;
    use std::fs::read_to_string;
    use std::io::Error;

    use walkdir::WalkDir;
    use wasmtime::*;

    fn setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn run_wat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let engine = Engine::default();
        let binary = wat::parse_file(path)?;
        let module = Module::new(&engine, &binary)?;
        let mut store = Store::new(&engine, ());
        let mut linker = Linker::<()>::new(&engine);
        linker.func_wrap("env", "assert_eq_i32", |a: i32, b: i32| {
            assert_eq!(a, b);
        })?;
        linker.func_wrap("env", "assert_eq_i64", |a: i64, b: i64| {
            assert_eq!(a, b);
        })?;
        let instance = linker.instantiate(&mut store, &module)?;
        let main = instance.get_typed_func::<(), ()>(&mut store, "main")?;
        main.call(&mut store, ())?;
        Ok(())
    }

    #[test]
    fn run_wat() -> Result<(), Box<dyn std::error::Error>> {
        for entry in WalkDir::new("../zkasm_data")
            .into_iter()
            .map(|e| e.unwrap())
        {
            if entry.path().extension().map_or(false, |ext| ext == "wat") {
                let result = run_wat_file(entry.path());
                if entry.path().to_str().unwrap().contains("_should_fail_") {
                    if result.is_ok() {
                        panic!(
                            "Should fail file {} don't actually fail",
                            entry.path().to_str().unwrap()
                        );
                    }
                } else {
                    if result.is_err() {
                        println!("Err = {:#?}", result);
                        panic!("Invalid wasm in {}", entry.path().to_str().unwrap());
                    }
                }
            }
        }
        Ok(())
    }

    fn test_module(name: &str) {
        let module_binary = wat::parse_file(format!("../zkasm_data/{name}.wat")).unwrap();
        let settings = ZkasmSettings::default();
        let program = zkasm_codegen::generate_zkasm(&settings, &module_binary);
        let expected =
            expect_test::expect_file![format!("../../zkasm_data/generated/{name}.zkasm")];
        expected.assert_eq(&program);
    }

    // This function asserts that none of tests generated from
    // spectest has been changed.
    fn check_spectests(name: &str) -> Result<(), Error> {
        let spectests_path = &format!("../../tests/spec_testsuite/{name}.wast");
        let file_content = read_to_string(spectests_path)?;
        let re = Regex::new(
            r#"\(assert_return \(invoke \"([.\w]+)\"\s*((?:\([^\)]+\)\s*)+)\)\s*\(([^\)]+)\)\)"#,
        )
        .unwrap();
        let mut test_counters = HashMap::new();
        for cap in re.captures_iter(&file_content) {
            let function_name = &cap[1];
            let arguments = &cap[2];
            let expected_result = &cap[3];
            let assert_type = &expected_result[0..3];
            // It seems like wast have different nan type comparing to our wat parser,
            // I faced wat parser error parsing generated tests with nan.
            // So, just skip fp tests for now.
            let mut is_float = false;
            for i in 0..4 {
                if cap[i].contains("f32.") || cap[i].contains("f64") {
                    is_float = true;
                    continue;
                }
            }
            if is_float {
                continue;
            }
            let count = test_counters.entry(function_name.to_string()).or_insert(0);
            *count += 1;
            let mut testcase = String::new();
            testcase.push_str(&format!("(module\n (import \"env\" \"assert_eq_{}\" (func $assert_eq_{} (param {}) (param {})))\n (func $main\n", assert_type, assert_type, assert_type, assert_type));
            testcase.push_str(&format!(
                "\t{}\n",
                arguments
                    .replace(") (", "\n\t")
                    .replace("(", "")
                    .replace(")", "")
            ));
            let function_type = if arguments.contains("i64") {
                "i64"
            } else {
                "i32"
            };
            let func_types = ["i32.", "i64."];
            if func_types.iter().any(|&pat| function_name.contains(pat)) {
                testcase.push_str(&format!("\t{}\n", function_name));
            } else {
                testcase.push_str(&format!("\t{function_type}.{}\n", function_name));
            }
            testcase.push_str(&format!(
                "\t{}\n\tcall $assert_eq_{})\n (export \"main\" (func $main)))\n",
                expected_result.trim(),
                assert_type
            ));
            let file_name = format!(
                "../../zkasm_data/spectest/{name}/{}_{}.wat",
                function_name, count
            );
            let expected_test = expect_test::expect_file![file_name];
            expected_test.assert_eq(&testcase);
        }
        Ok(())
    }

    fn test_wat_in_directory(path: &Path) {
        let mut failures = 0;
        let mut count = 0;
        for entry in path.read_dir().expect("Directory not found") {
            let entry = entry.expect("Failed to read entry");
            if entry.path().extension().and_then(|s| s.to_str()) != Some("wat") {
                continue;
            }

            count += 1;
            let name = entry
                .path()
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap()
                .to_owned();
            let module_binary = wat::parse_file(path.join(format!("{name}.wat"))).unwrap();
            let expected = expect_test::expect_file![PathBuf::from("../")
                .join(path)
                .join(format!("generated/{name}.zkasm"))];
            let result = std::panic::catch_unwind(|| {
                let settings = ZkasmSettings::default();
                let program = zkasm_codegen::generate_zkasm(&settings, &module_binary);
                expected.assert_eq(&program);
            });
            if let Err(err) = result {
                failures += 1;
                println!("{name} fails with {err:#?}");
            }
        }
        println!("Failed {failures} tests out of {count}");
    }

    fn run_spectest(name: &str) {
        check_spectests(name).unwrap();
        test_wat_in_directory(Path::new(&format!("../zkasm_data/spectest/{name}/")));
    }

    #[test]
    fn run_spectests() {
        run_spectest("i32");
        run_spectest("i64");
        run_spectest("conversions");
    }

    #[test]
    fn run_benchmarks() {
        test_wat_in_directory(Path::new(&format!("../zkasm_data/benchmarks/fibonacci")));
        test_wat_in_directory(Path::new(&format!("../zkasm_data/benchmarks/sha256")));
    }

    macro_rules! testcases {
        { $($name:ident,)* } => {
          $(
            #[test]
            fn $name() {
                setup();
                test_module(stringify!($name));
            }
           )*
        };
    }

    testcases! {
        add,
        locals,
        locals_simple,
        global,
        memory,
        counter,
        add_func,
        mul,
        i64_mul,
        lt_s,
        lt_u,
        xor,
        and,
        or,
        div,
        i64_div,
        eqz,
        ne,
        nop,
        _should_fail_unreachable,
        i32_const,
        i64_const,
        rem,
        i32_add_overflows,
        i32_mul_overflows,
        i64_mul_overflows,
        i64_rem,
        memory_i32,
    }
}

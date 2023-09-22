#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use cranelift_codegen::entity::EntityRef;
    use cranelift_codegen::ir::function::FunctionParameters;
    use cranelift_codegen::ir::ExternalName;
    use cranelift_codegen::isa::zkasm;
    use cranelift_codegen::{settings, FinalizedMachReloc, FinalizedRelocTarget};
    use cranelift_wasm::{translate_module, DummyEnvironment};

    fn generate_preamble(start_func_index: usize) -> Vec<String> {
        let mut program: Vec<String> = Vec::new();
        program.push("start:".to_string());
        program.push("  zkPC + 2 => RR".to_string());
        program.push(format!("  :JMP(function_{})", start_func_index));
        program.push("  :JMP(finalizeExecution)".to_string());
        program
    }

    fn generate_postamble() -> Vec<String> {
        let mut program: Vec<String> = Vec::new();
        program.push("finalizeExecution:".to_string());
        program.push("  ${beforeLast()}  :JMPN(finalizeExecution):".to_string());
        program.push("                   :JMP(start)".to_string());
        program
    }

    fn fix_relocs(
        code_buffer: &mut Vec<u8>,
        params: &FunctionParameters,
        relocs: &[FinalizedMachReloc],
    ) {
        let mut delta = 0i32;
        for reloc in relocs {
            let start = (reloc.offset as i32 + delta) as usize;
            let mut pos = start;
            while code_buffer[pos] != b'\n' {
                pos += 1;
                delta -= 1;
            }

            let code = if let FinalizedRelocTarget::ExternalName(ExternalName::User(name)) =
                reloc.target
            {
                let name = &params.user_named_funcs()[name];
                if name.index == 0 {
                    b"  B :ASSERT".to_vec()
                } else {
                    format!("  zkPC + 2 => RR\n  :JMP(function_{})", name.index)
                        .as_bytes()
                        .to_vec()
                }
            } else {
                b"  UNKNOWN".to_vec()
            };
            delta += code.len() as i32;

            code_buffer.splice(start..pos, code);
        }
    }

    fn optimize_labels(code: &[&str], func_index: usize) -> Vec<String> {
        let mut label_definition: HashMap<usize, usize> = HashMap::new();
        let mut label_uses: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut lines = Vec::new();
        for (index, line) in code.iter().enumerate() {
            let mut line = line.to_string();
            if line.starts_with(&"label_") {
                let label_index: usize = line[6..line.len() - 1]
                    .parse()
                    .expect("Failed to parse label index");
                line = format!("L{func_index}_{label_index}:");
                label_definition.insert(label_index, index);
            } else if line.contains(&"label_") {
                let pos = line.find(&"label_").unwrap();
                let pos_end = pos + line[pos..].find(&")").unwrap();
                let label_index: usize = line[pos + 6..pos_end]
                    .parse()
                    .expect("Failed to parse label index");
                line.replace_range(pos..pos_end, &format!("L{func_index}_{label_index}"));
                label_uses.entry(label_index).or_default().push(index);
            }
            lines.push(line);
        }

        let mut lines_to_delete = Vec::new();
        for (label, label_line) in label_definition {
            match label_uses.entry(label) {
                std::collections::hash_map::Entry::Occupied(uses) => {
                    if uses.get().len() == 1 {
                        let use_line = uses.get()[0];
                        if use_line + 1 == label_line {
                            lines_to_delete.push(use_line);
                            lines_to_delete.push(label_line);
                        }
                    }
                }
                std::collections::hash_map::Entry::Vacant(_) => {
                    lines_to_delete.push(label_line);
                }
            }
        }
        lines_to_delete.sort();
        lines_to_delete.reverse();
        for index in lines_to_delete {
            lines.remove(index);
        }
        lines
    }

    fn test_module(name: &str) {
        // TODO: Replace "sparc" with "zkasm" when
        // https://github.com/bytecodealliance/target-lexicon/pull/94 lands.
        let flag_builder = settings::builder();
        let isa_builder = zkasm::isa_builder("sparc-unknown-unknown".parse().unwrap());
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let mut dummy_environ = DummyEnvironment::new(isa.frontend_config());
        let module_binary = wat::parse_file(format!("zkasm_data/{name}.wat")).unwrap();
        translate_module(&module_binary, &mut dummy_environ).unwrap();

        let mut program: Vec<String> = Vec::new();
        let start_func = dummy_environ
            .info
            .start_func
            .expect("Must have a start function");
        program.append(&mut generate_preamble(start_func.index()));

        let num_func_imports = dummy_environ.get_num_func_imports();
        let mut context = cranelift_codegen::Context::new();
        for (def_index, func) in dummy_environ.info.function_bodies.iter() {
            let func_index = num_func_imports + def_index.index();
            program.push(format!("function_{}:", func_index));

            let mut mem = vec![];
            context.func = func.clone();
            let compiled_code = context
                .compile_and_emit(&*isa, &mut mem, &mut Default::default())
                .unwrap();
            let mut code_buffer = compiled_code.code_buffer().to_vec();
            fix_relocs(
                &mut code_buffer,
                &func.params,
                compiled_code.buffer.relocs(),
            );

            let code = std::str::from_utf8(&code_buffer).unwrap();
            let lines: Vec<&str> = code.lines().collect();
            let mut lines = optimize_labels(&lines, func_index);
            program.append(&mut lines);

            context.clear();
        }

        program.append(&mut generate_postamble());

        let expected = expect_test::expect_file![format!("../zkasm_data/generated/{name}.zkasm")];
        let program = program.join("\n");
        expected.assert_eq(&program);
    }

    macro_rules! testcases {
        { $($name:ident,)* } => {
          $(
            #[test]
            fn $name() {
                test_module(stringify!($name));
            }
           )*
        };
    }

    testcases! {
        add,
        locals,
        locals_simple,
        counter,
        fibonacci,
        add_func,
        add_memory,
    }
}

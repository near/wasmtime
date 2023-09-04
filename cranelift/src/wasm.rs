//! CLI tool to use the functions provided by the [cranelift-wasm](../cranelift_wasm/index.html)
//! crate.
//!
//! Reads Wasm binary/text files, translates the functions' code to Cranelift IR.
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::too_many_arguments, clippy::cognitive_complexity)
)]

use crate::disasm::print_all;
use anyhow::{Context as _, Result};
use clap::Parser;
use cranelift_codegen::ir::ExternalName;
use cranelift_codegen::print_errors::{pretty_error, pretty_verifier_error};
use cranelift_codegen::settings::FlagsOrIsa;
use cranelift_codegen::timing;
use cranelift_codegen::Context;
use cranelift_entity::EntityRef;
use cranelift_reader::parse_sets_and_triple;
use cranelift_wasm::{translate_module, DummyEnvironment, FuncIndex};
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// For verbose printing: only print if the `$x` expression is true.
macro_rules! vprintln {
    ($x: expr, $($tts:tt)*) => {
        if $x {
            println!($($tts)*);
        }
    };
}
/// For verbose printing: prints in color if the `$x` expression is true.
macro_rules! vcprintln {
    ($x: expr, $use_color: expr, $term: ident, $color: expr, $($tts:tt)*) => {
        if $x {
            if $use_color {
                $term.set_color(ColorSpec::new().set_fg(Some($color)))?;
            }
            println!($($tts)*);
            if $use_color {
                $term.reset()?;
            }
        }
    };
}
/// For verbose printing: prints in color (without an appended newline) if the `$x` expression is true.
macro_rules! vcprint {
    ($x: expr, $use_color: expr, $term: ident, $color: expr, $($tts:tt)*) => {
        if $x {
            if $use_color {
                $term.set_color(ColorSpec::new().set_fg(Some($color)))?;
            }
            print!($($tts)*);
            if $use_color {
                $term.reset()?;
            }
        }
    };
}

/// Compiles Wasm binary/text into Cranelift IR and then into target language
#[derive(Parser)]
pub struct Options {
    /// Be more verbose
    #[clap(short, long)]
    verbose: bool,

    /// Print the resulting Cranelift IR
    #[clap(short)]
    print: bool,

    /// Print pass timing report
    #[clap(short = 'T')]
    report_times: bool,

    /// Print machine code disassembly
    #[clap(short = 'D', long)]
    disasm: bool,

    /// Configure Cranelift settings
    #[clap(long = "set")]
    settings: Vec<String>,

    /// Specify the Cranelift target
    #[clap(long = "target")]
    target: String,

    /// Specify an input file to be used. Use '-' for stdin.
    files: Vec<PathBuf>,

    /// Print bytecode size
    #[clap(short = 'X')]
    print_size: bool,

    /// Just decode Wasm into Cranelift IR, don't compile it to native code
    #[clap(short = 't')]
    just_decode: bool,

    /// Just checks the correctness of Cranelift IR translated from Wasm
    #[clap(short = 'c')]
    check_translation: bool,

    /// Use colors in output? [options: auto/never/always; default: auto]
    #[clap(long = "color", default_value("auto"))]
    color: ColorOpt,
}

#[derive(PartialEq, Eq, Clone)]
enum ColorOpt {
    Auto,
    Never,
    Always,
}

impl std::str::FromStr for ColorOpt {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "auto" => Ok(ColorOpt::Auto),
            "never" => Ok(ColorOpt::Never),
            "always" => Ok(ColorOpt::Always),
            _ => Err(format!("expected auto/never/always, found: {}", s)),
        }
    }
}

pub fn run(options: &Options) -> Result<()> {
    let parsed = parse_sets_and_triple(&options.settings, &options.target)?;
    for path in &options.files {
        let name = String::from(path.as_os_str().to_string_lossy());
        handle_module(options, path, &name, parsed.as_fisa())?;
    }
    Ok(())
}

fn handle_module(options: &Options, path: &Path, name: &str, fisa: FlagsOrIsa) -> Result<()> {
    let color_choice = match options.color {
        ColorOpt::Auto => ColorChoice::Auto,
        ColorOpt::Always => ColorChoice::Always,
        ColorOpt::Never => ColorChoice::Never,
    };
    let mut terminal = StandardStream::stdout(color_choice);
    let use_color = terminal.supports_color() && options.color == ColorOpt::Auto
        || options.color == ColorOpt::Always;
    vcprint!(
        options.verbose,
        use_color,
        terminal,
        Color::Yellow,
        "Handling: "
    );
    vprintln!(options.verbose, "\"{}\"", name);
    vcprint!(
        options.verbose,
        use_color,
        terminal,
        Color::Magenta,
        "Translating... "
    );

    let module_binary = if path.to_str() == Some("-") {
        let stdin = std::io::stdin();
        let mut buf = Vec::new();
        stdin
            .lock()
            .read_to_end(&mut buf)
            .context("failed to read stdin")?;
        wat::parse_bytes(&buf)?.into()
    } else {
        wat::parse_file(path)?
    };

    let isa = match fisa.isa {
        Some(isa) => isa,
        None => {
            anyhow::bail!("Error: the wasm command requires an explicit isa.");
        }
    };

    let mut dummy_environ = DummyEnvironment::new(isa.frontend_config());
    translate_module(&module_binary, &mut dummy_environ)?;

    vcprintln!(options.verbose, use_color, terminal, Color::Green, "ok");

    if options.just_decode {
        if !options.print {
            return Ok(());
        }

        let num_func_imports = dummy_environ.get_num_func_imports();
        for (def_index, func) in dummy_environ.info.function_bodies.iter() {
            let func_index = num_func_imports + def_index.index();
            let mut context = Context::new();
            context.func = func.clone();
            if let Some(start_func) = dummy_environ.info.start_func {
                if func_index == start_func.index() {
                    println!("; Selected as wasm start function");
                }
            }
            vprintln!(options.verbose, "");
            for export_name in
                &dummy_environ.info.functions[FuncIndex::new(func_index)].export_names
            {
                println!("; Exported as \"{}\"", export_name);
            }
            println!("{}", context.func.display());
            vprintln!(options.verbose, "");
        }
        let _ = terminal.reset();
        return Ok(());
    }

    if options.check_translation {
        vcprint!(
            options.verbose,
            use_color,
            terminal,
            Color::Magenta,
            "Checking... "
        );
    } else {
        vcprint!(
            options.verbose,
            use_color,
            terminal,
            Color::Magenta,
            "Compiling... "
        );
    }

    if options.print_size {
        vprintln!(options.verbose, "");
    }

    println!("start:");
    let start_func = dummy_environ
        .info
        .start_func
        .expect("Must have a start function");
    println!("  zkPC + 2 => RR");
    // TODO(akashin): This is a poor translation between DefinedFuncIndex and FuncIndex.
    // Ideally, we would use some library function for this.
    println!("  :JMP(function_{})", start_func.index() - 1);
    println!("  :JMP(finalizeExecution)");

    let num_func_imports = dummy_environ.get_num_func_imports();
    let mut total_module_code_size = 0;
    let mut context = Context::new();
    for (def_index, func) in dummy_environ.info.function_bodies.iter() {
        println!("function_{}:", def_index.index());
        context.func = func.clone();

        let mut saved_size = None;
        let func_index = num_func_imports + def_index.index();
        let mut mem = vec![];
        let (relocs, traps, stack_maps) = if options.check_translation {
            if let Err(errors) = context.verify(fisa) {
                anyhow::bail!("{}", pretty_verifier_error(&context.func, None, errors));
            }
            (vec![], vec![], vec![])
        } else {
            let compiled_code = context
                .compile_and_emit(isa, &mut mem, &mut Default::default())
                .map_err(|err| anyhow::anyhow!("{}", pretty_error(&err.func, err.inner)))?;
            let code_info = compiled_code.code_info();
            let mut code_buffer = compiled_code.code_buffer().to_vec();
            let mut delta = 0i32;
            for reloc in compiled_code.buffer.relocs() {
                let start = (reloc.offset as i32 + delta) as usize;
                let mut pos = start;
                while code_buffer[pos] != b'\n' {
                    pos += 1;
                    delta -= 1;
                }

                let code = if let ExternalName::User(name) = reloc.name {
                    if name.index() == 0 {
                        b"  B :ASSERT".to_vec()
                    } else {
                        format!("  JMP(function_{})", name.index()).as_bytes().to_vec()
                    }
                } else {
                    b"  UNKNOWN".to_vec()
                };
                delta += code.len() as i32;

                code_buffer.splice(start..pos, code);
            }

            if let Ok(code) = std::str::from_utf8(&code_buffer) {
                println!("{code}",);
            }

            if options.print_size {
                println!(
                    "Function #{} code size: {} bytes",
                    func_index, code_info.total_size,
                );
                total_module_code_size += code_info.total_size;
                println!(
                    "Function #{} bytecode size: {} bytes",
                    func_index,
                    dummy_environ.func_bytecode_sizes[def_index.index()]
                );
            }

            if options.disasm {
                saved_size = Some(code_info.total_size);
            }
            (
                compiled_code.buffer.relocs().to_vec(),
                compiled_code.buffer.traps().to_vec(),
                compiled_code.buffer.stack_maps().to_vec(),
            )
        };

        if options.print {
            vprintln!(options.verbose, "");
            if let Some(start_func) = dummy_environ.info.start_func {
                if func_index == start_func.index() {
                    println!("; Selected as wasm start function");
                }
            }
            for export_name in
                &dummy_environ.info.functions[FuncIndex::new(func_index)].export_names
            {
                println!("; Exported as \"{}\"", export_name);
            }
            println!("{}", context.func.display());
            vprintln!(options.verbose, "");
        }

        if let Some(total_size) = saved_size {
            print_all(
                isa,
                &context.func.params,
                &mem,
                total_size,
                options.print,
                &relocs,
                &traps,
                &stack_maps,
            )?;
        }

        context.clear();
    }

    let postamble = "finalizeExecution:
  ${beforeLast()}  :JMPN(finalizeExecution)
                   :JMP(start)
";
    println!("{postamble}");

    if !options.check_translation && options.print_size {
        println!("Total module code size: {} bytes", total_module_code_size);
        let total_bytecode_size: usize = dummy_environ.func_bytecode_sizes.iter().sum();
        println!("Total module bytecode size: {} bytes", total_bytecode_size);
    }

    if options.report_times {
        println!("{}", timing::take_current());
    }

    vcprintln!(options.verbose, use_color, terminal, Color::Green, "ok");
    Ok(())
}

//! Compiles and runs a Cairo program.

use std::path::Path;

use anyhow::Context;
use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_compiler::diagnostics::check_and_eprint_diagnostics;
use cairo_lang_compiler::project::setup_project;
use cairo_lang_diagnostics::ToOption;
use cairo_lang_runner::SierraCasmRunner;
use cairo_lang_sierra_generator::db::SierraGenGroup;
use cairo_lang_sierra_generator::replace_ids::replace_sierra_ids_in_program;

/// Command line args parser.
/// Exits with 0/1 if the input is formatted correctly/incorrectly.
struct Args {
    /// The file to compile and run.
    path: String,
    /// In cases where gas is available, the amount of provided gas.
    available_gas: Option<usize>,
}

pub fn run_cairo(path: &str) -> anyhow::Result<String> {
    let args = Args {
        path: path.into(),
        available_gas: None,
    };

    let mut db_val = RootDatabase::default();
    let db = &mut db_val;

    let main_crate_ids = setup_project(db, Path::new(&args.path))?;

    if check_and_eprint_diagnostics(db) {
        anyhow::bail!("failed to compile: {}", args.path);
    }

    let sierra_program = db
        .get_sierra_program(main_crate_ids)
        .to_option()
        .with_context(|| "Compilation failed without any diagnostics.")?;
    let runner = SierraCasmRunner::new(
        replace_sierra_ids_in_program(db, &sierra_program),
        args.available_gas.is_some(),
    )
    .with_context(|| "Failed setting up runner.")?;
    let result = runner
        .run_function("::main", &[], args.available_gas)
        .with_context(|| "Failed to run the function.")?;
    match result.value {
        cairo_lang_runner::RunResultValue::Success(values) => Ok(format!("{values:?}")),
        cairo_lang_runner::RunResultValue::Panic(values) => {
            anyhow::bail!("Run panicked with err: {values:?}")
        }
    }
}

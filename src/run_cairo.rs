//! Compiles and runs a Cairo program.
use std::path::Path;

use anyhow::{Context, Ok};
use cairo_compiler::db::RootDatabase;
use cairo_compiler::diagnostics::check_and_eprint_diagnostics;
use cairo_compiler::project::setup_project;
use cairo_diagnostics::ToOption;
use cairo_runner::SierraCasmRunner;
use cairo_sierra_generator::db::SierraGenGroup;
use cairo_sierra_generator::replace_ids::replace_sierra_ids_in_program;

pub fn run_cairo(path: &str) -> anyhow::Result<String> {
    let mut db_val = RootDatabase::default();
    let db = &mut db_val;

    let main_crate_ids = setup_project(db, Path::new(&path))?;

    if check_and_eprint_diagnostics(db) {
        anyhow::bail!("failed to compile: {}", path);
    }

    let sierra_program = db
        .get_sierra_program(main_crate_ids)
        .to_option()
        .with_context(|| "Compilation failed without any diagnostics.")?;
    let runner = SierraCasmRunner::new(replace_sierra_ids_in_program(db, &sierra_program), false)
        .with_context(|| "Failed setting up runner.")?;
    let result = runner
        .run_function("::main", &[], None)
        .with_context(|| "Failed to run the function.")?;
    let mut result_str = String::new();
    match result.value {
        cairo_runner::RunResultValue::Success(values) => {
            for val in values.iter() {
                result_str.push_str(&val.to_string());
                result_str.push_str(",")
            }
        }
        cairo_runner::RunResultValue::Panic(values) => {
            println!("Run panicked with err values: {values:?}")
        }
    }
    Ok(result_str)
}

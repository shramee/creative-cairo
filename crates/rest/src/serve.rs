#![feature(internal_output_capture)]
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;

use actix_files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::{Context, Ok};
use casm::instructions::Instruction;
use casm::{casm, casm_extend};
// use clap::Parser;
use compiler::db::RootDatabase;
use compiler::diagnostics::check_diagnostics;
use compiler::project::setup_project;
use itertools::chain;
use sierra::program::StatementIdx;
use sierra_gas::calc_gas_info;
use sierra_gas::gas_info::GasInfo;
use sierra_generator::db::SierraGenGroup;
use sierra_generator::replace_ids::replace_sierra_ids_in_program;
use sierra_to_casm::metadata::Metadata;

fn cairo_runner(path: &String) -> Result<String, anyhow::Error> {
    let mut db_val = RootDatabase::default();
    let db = &mut db_val;

    let main_crate_ids = setup_project(db, Path::new(&path))?;

    if check_diagnostics(db) {
        anyhow::bail!("Failed to get path");
    }

    let sierra_program = db
        .get_sierra_program(main_crate_ids)
        .with_context(|| "Compilation failed without any diagnostics.")?;
    let function_sizes = function_to_input_output_sizes(&sierra_program, db);

    let sierra_program = Arc::new(replace_sierra_ids_in_program(db, &sierra_program));
    let main_func =
        find_main(&sierra_program).with_context(|| "Main function not provided in module.")?;
    let metadata = create_metadata(&sierra_program, false)?;
    let program = sierra_to_casm::compiler::compile(&sierra_program, &metadata, false)
        .with_context(|| "Failed lowering to casm.")?;
    let entry_code = create_entry_code(main_func, Some(0), metadata, &program)?;

    let (input_size, output_size) = function_sizes[&main_func.entry_point];
    let (memory, ap) = casm::run::run_function(chain!(entry_code, program.instructions).collect())
        .with_context(|| "Failed running casm code.")?;

    let mut result_str = String::new();
    let printed_result_size = output_size - input_size;
    for cell in &memory[(ap - printed_result_size)..ap] {
        match cell {
            None => result_str.push_str("0,"),
            Some(value) => {
                result_str.push_str(&value.to_string());
                result_str.push_str(",")
            }
        }
    }

    Ok(result_str)
}

/// Returns the instructions to add to the begining of the code to successfully call the main
/// function.
fn create_entry_code(
    main_func: &sierra::program::GenFunction<StatementIdx>,
    available_gas: Option<usize>,
    metadata: Metadata,
    program: &sierra_to_casm::compiler::CairoProgram,
) -> Result<Vec<Instruction>, anyhow::Error> {
    let mut ctx = casm! {};
    for (i, ty) in main_func.signature.param_types.iter().enumerate() {
        if &main_func.signature.ret_types[i] != ty {
            anyhow::bail!("We only support main functions with no parameters.");
        }
        if ty == &"RangeCheck".into() {
            casm_extend! {ctx,
                %{ memory[ap + 0] = segments.add() %}
                ap += 1;
            }
        } else if ty == &"GasBuiltin".into() {
            if let Some(available_gas) = available_gas {
                let initial_gas = available_gas
                    .checked_sub(metadata.gas_info.function_costs[&main_func.id] as usize);
                if let Some(initial_gas) = initial_gas {
                    casm_extend! {ctx,
                        [ap + 0] = initial_gas, ap++;
                    }
                } else {
                    anyhow::bail!("Not enough gas to call function.");
                }
            } else {
                anyhow::bail!("GasBuiltin is required while no `available_gas` value provided.");
            }
        } else {
            anyhow::bail!("Inputs for main are not supported.");
        }
    }
    let before_final_call = ctx.current_code_offset;
    let final_call_size = 3;
    let offset = final_call_size
        + program.debug_info.sierra_statement_info[main_func.entry_point.0].code_offset;
    casm_extend! {ctx,
        call rel offset;
        ret;
    }
    assert_eq!(before_final_call + final_call_size, ctx.current_code_offset);
    Ok(ctx.instructions)
}

/// Creates the metadata required for a Sierra program lowering to casm.
fn create_metadata(
    sierra_program: &Arc<sierra::program::Program>,
    calc_gas: bool,
) -> Result<Metadata, anyhow::Error> {
    let gas_info = if calc_gas {
        calc_gas_info(sierra_program).with_context(|| {
            "Failed calculating gas usage, it is likely a call for `get_gas` is missing."
        })?
    } else {
        GasInfo { variable_values: HashMap::new(), function_costs: HashMap::new() }
    };
    let metadata = Metadata { function_ap_change: HashMap::new(), gas_info };
    Ok(metadata)
}

/// Find the main function of the program.
fn find_main(
    sierra_program: &Arc<sierra::program::Program>,
) -> Option<&sierra::program::GenFunction<StatementIdx>> {
    sierra_program
        .funcs
        .iter()
        .find(|f| if let Some(name) = &f.id.debug_name { name.ends_with("::main") } else { false })
}

/// Returns the total sizes of inputs and outputs per a Sierra function entry point.
fn function_to_input_output_sizes(
    sierra_program: &Arc<sierra::program::Program>,
    db: &mut RootDatabase,
) -> HashMap<StatementIdx, (usize, usize)> {
    let function_sizes =
        HashMap::<StatementIdx, (usize, usize)>::from_iter(sierra_program.funcs.iter().map(|f| {
            let mut input_size = 0;
            for ty in &f.signature.param_types {
                input_size += db.get_type_info(ty.clone()).unwrap().size;
            }
            let mut output_size = 0;
            for ty in &f.signature.ret_types {
                output_size += db.get_type_info(ty.clone()).unwrap().size;
            }
            (f.entry_point, (input_size, output_size))
        }));
    function_sizes
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/cairo", web::post().to(handle_connection))
            .service(actix_files::Files::new("/static", ".").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn cairo_run_inner(cairo: String) -> Result<String, anyhow::Error> {
    let mut file = File::create("compile.cairo")?;
    file.write_all(&cairo.as_bytes())?;

    let path = String::from("compile.cairo");

    cairo_runner(&path)
}

fn handle_cairo_run(cairo: String) -> String {
    std::io::set_output_capture(Some(Default::default()));

    let captured = std::io::set_output_capture(None);
    let captured = captured.unwrap();
    let captured = Arc::try_unwrap(captured).unwrap();
    let captured = captured.into_inner().unwrap();
    let captured = String::from_utf8(captured).unwrap();

    let result = std::panic::catch_unwind(|| cairo_run_inner(cairo));

    if result.is_ok() {
        format!("{:#?}", result.unwrap())
    } else {
        format!("Error occurred.\n{}", captured)
    }
}

async fn handle_connection(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(handle_cairo_run(req_body))
}

mod utils;

use std::io::Cursor;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
// use anyhow::{Context, Ok};
use casm::instructions::Instruction;
use casm::{casm, casm_extend};
use clap::Parser;
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

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start)]
pub fn start() {
    crate::utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn run_cairo_program( program_json: &str ) -> Result<String, JsError> {
	// let program = Program::from_reader(Cursor::new(program_json), Some("main"))?;
		
    Ok(String::from(""))
}

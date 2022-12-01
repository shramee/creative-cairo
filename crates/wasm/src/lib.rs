mod utils;

use cairo_rs::{
    hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor,
    types::program::Program,
    vm::{runners::cairo_runner::CairoRunner, vm_core::VirtualMachine},
};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

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
    let program = Program::from_reader(Cursor::new(program_json), Some("main"))?;
		
    let mut runner = CairoRunner::new(&program, "all", false)?;
    let mut vm = VirtualMachine::new(program.prime, false);
    let hint_processor = BuiltinHintProcessor::new_empty();

    let end = runner.initialize(&mut vm)?;
    runner.run_until_pc(end, &mut vm, &hint_processor)?;

    let mut buffer = Cursor::new(Vec::new());
    runner.write_output(&mut vm, &mut buffer)?;
    Ok(String::from_utf8(buffer.into_inner())?)
}

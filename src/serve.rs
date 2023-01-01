#![feature(internal_output_capture)]
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

use actix_files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use clap::Parser;

mod run_cairo;
use run_cairo::run_cairo;

static SERVE_PATH: &str = "./static";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Serving files from {}", SERVE_PATH);
    HttpServer::new(|| {
        App::new()
            .route("/cairo", web::post().to(handle_connection))
            .service(actix_files::Files::new("/", SERVE_PATH).show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn write_cairo_to_file(cairo: String) -> String {
    let mut file = File::create("sketch/lib.cairo").unwrap();
    let _r = file.write_all(&cairo.as_bytes());
    String::from("sketch")
}

fn handle_cairo_run(cairo: String) -> String {
    std::io::set_output_capture(Some(Default::default()));

    let captured = std::io::set_output_capture(None);
    let captured = captured.unwrap();
    let captured = Arc::try_unwrap(captured).unwrap();
    let captured = captured.into_inner().unwrap();
    let captured = String::from_utf8(captured).unwrap();

    let result = std::panic::catch_unwind(|| {
        let path = write_cairo_to_file(cairo);
        run_cairo(&path).unwrap()
    });

    if result.is_ok() {
        format!("{:#?}", result.unwrap())
    } else {
        format!("Error occurred.\n{}", captured)
    }
}

async fn handle_connection(req_body: String) -> impl Responder {
    let run_response = handle_cairo_run(req_body);
    println!("'{:#?}'", run_response);
    HttpResponse::Ok().body(run_response)
}

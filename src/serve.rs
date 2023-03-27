use std::fs::File;
use std::io::prelude::*;

use actix_cors::Cors;
use actix_files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use clap::Parser;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod run_cairo;
use run_cairo::run_cairo;

static SERVE_PATH: &str = "./static";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = match std::env::var("CREATIVE_CAIRO_PORT") {
        Ok(val) => val.parse::<u16>().unwrap(),
        Err(_e) => 8080,
    };

    println!("Serving on port {}", port);
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .route("/cairo", web::post().to(handle_connection))
            .service(actix_files::Files::new("/", SERVE_PATH).show_files_listing())
    });

    match std::env::var("CREATIVE_CAIRO_SSL_KEY") {
        Ok(ssl_key) => {
            let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            builder
                .set_private_key_file(ssl_key, SslFiletype::PEM)
                .unwrap();
            builder.set_certificate_chain_file("cert.pem").unwrap();
            server
                .bind_openssl(("0.0.0.0", port), builder)
                .unwrap()
                .run()
                .await
        }
        Err(_e) => server.bind(("0.0.0.0", port)).unwrap().run().await,
    }
}

fn write_cairo_to_file(cairo: String) -> String {
    let mut file = File::create("sketch/lib.cairo").unwrap();
    let _r = file.write_all(&cairo.as_bytes());
    String::from("sketch")
}

fn handle_cairo_run(cairo: String) -> String {
    let path = write_cairo_to_file(cairo);
    match run_cairo(&path) {
        Ok(v) => v,
        Err(e) => e.to_string(),
    }
}

async fn handle_connection(req_body: String) -> impl Responder {
    let run_response = handle_cairo_run(req_body);
    println!("{run_response:#?}");
    HttpResponse::Ok().body(run_response)
}

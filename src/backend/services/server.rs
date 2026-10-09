use actix_web::{App, HttpServer};
use std::net::TcpListener;
use std::time::Instant;
use colored::*;

use crate::handlers::register::configure_routes;
use crate::utils::cors::cors;
use crate::utils::env::{get_backend_port, get_backend_version};
use crate::utils::local_ip::get_local_ip;
use crate::utils::log;

pub async fn start_server() -> std::io::Result<()> {
    let start_time = Instant::now();

    let backend_version = get_backend_version();
    let backend_port = get_backend_port();
    let local_url = format!("http://localhost:{}", backend_port);
    let network_url = format!("http://{}:{}", get_local_ip(), backend_port);

    println!("{} {}", "   ★ Actix Web".bright_blue().bold(), backend_version.bright_blue().bold());
    println!("{} {}", "   - Local:", local_url);
    println!("{} {}", "   - Network:", network_url);
    println!("{}", "   - Environments: .env");
    println!();

    log::success("Starting...");

    let listener = match TcpListener::bind(("127.0.0.1", backend_port)) {
        Ok(l) => l,
        Err(_) => {
            log::error(&format!("Port {} is already in use", backend_port));
            std::process::exit(1);
        }
    };

    let server = HttpServer::new(move || {
        App::new()
            .wrap(cors())
            .configure(configure_routes)
    })
    .listen(listener)?;

    let elapsed = start_time.elapsed();
    let ms = elapsed.as_millis();

    if ms < 1000 {
        log::success(&format!("Ready in {}ms", ms));
    } else {
        log::success(&format!("Ready in {:.2}s", elapsed.as_secs_f32()));
    }

    server.run().await
}
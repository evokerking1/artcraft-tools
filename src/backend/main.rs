mod utils;
mod routes;
mod handlers;
mod services;

use services::server::start_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    start_server().await
}

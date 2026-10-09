use std::env;
use std::path::Path;

fn load_env() {
    let current_dir = env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
    let env_path = current_dir.join(".env");

    if env_path.exists() {
        if let Err(e) = dotenvy::from_path(&env_path) {
            eprintln!("Warning: Failed to load .env file: {}", e);
        }
    } else {
        if let Err(_) = dotenvy::dotenv() {
            eprintln!("Warning: No .env file found in current directory or parent directories");
        }
    }
}

pub fn get_backend_port() -> u16 {
    load_env();
    env::var("BACKEND_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}

pub fn get_cors_port() -> u16 {
    load_env();
    env::var("CORS_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000)
}

pub fn get_backend_version() -> String {
    load_env();
    env::var("BACKEND_VERSION").unwrap_or_else(|_| "1.0.0".to_string())
}

pub fn get_cors_origin() -> String {
    format!("http://localhost:{}", get_cors_port())
}


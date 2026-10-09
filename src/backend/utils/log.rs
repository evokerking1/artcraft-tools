use colored::*;

pub fn success(msg: &str) {
    println!("{} {}", " ✓".bright_green().bold(), msg);
}

#[allow(dead_code)]
pub fn warn(msg: &str) {
    println!("{} {}", " ⚠".bright_yellow().bold(), msg);
}

pub fn error(msg: &str) {
    println!("{} {}", " ✗".bright_red().bold(), msg);
}

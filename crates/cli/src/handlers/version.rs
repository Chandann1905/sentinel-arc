use console::style;

pub fn handle() {
    println!(
        "{} {}",
        style("Sentinel Arc CLI").cyan().bold(),
        env!("CARGO_PKG_VERSION")
    );
    println!(
        "Rust Version: {}",
        style(option_env!("RUSTC_VERSION").unwrap_or("unknown")).green()
    );
    println!(
        "Commit:       {}",
        style(option_env!("GIT_HASH").unwrap_or("unknown")).dim()
    );
}

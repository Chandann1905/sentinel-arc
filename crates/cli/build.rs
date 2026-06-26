use std::process::Command;

fn main() {
    if let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output() {
        if let Ok(hash) = String::from_utf8(output.stdout) {
            println!("cargo:rustc-env=GIT_HASH={}", hash.trim());
        }
    }

    if let Ok(output) = Command::new("rustc").arg("-V").output() {
        if let Ok(rustc) = String::from_utf8(output.stdout) {
            println!("cargo:rustc-env=RUSTC_VERSION={}", rustc.trim());
        }
    }
}

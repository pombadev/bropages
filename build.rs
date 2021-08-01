use std::{env, fs, path::Path, process::Command};

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();

    let git_hash = String::from_utf8(output.stdout).unwrap();
    let version = format!("v{}+{}", env!("CARGO_PKG_VERSION"), git_hash);

    fs::write(
        Path::new(&env::var("OUT_DIR").unwrap()).join("version"),
        version,
    )
    .unwrap();
}

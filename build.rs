use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
};

fn main() {
    let mut version = format!("v{}", env!("CARGO_PKG_VERSION"));

    let out = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .stdout(Stdio::piped())
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_default();

    if !out.is_empty() {
        version.push('+');
        version.push_str(out.trim());
    }

    fs::write(
        Path::new(&env::var("OUT_DIR").expect("OUT_DIR to be defined")).join("version"),
        &version,
    )
    .unwrap_or_else(|err| panic!("create version file with: {version} failed because: {err}"));
}

use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
};

fn main() {
    let mut version = format!("v{}", env!("CARGO_PKG_VERSION"));

    if let Ok(out) = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .stdout(Stdio::piped())
        .output()
    {
        if let Ok(hash) = String::from_utf8(out.stdout) {
            version.push('+');
            version.push_str(hash.trim());
        }
    }

    fs::write(
        Path::new(&env::var("OUT_DIR").expect("OUT_DIR to be defined")).join("version"),
        version.trim(),
    )
    .unwrap_or_else(|err| panic!("create version file with: {version} failed because: {err}"));
}

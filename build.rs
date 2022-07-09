use std::process::Command;
use std::io::{self, Write};

// rebuild front-end
fn main() {
    println!("cargo:rerun-if-changed=package.json");
    let output = Command::new("yarn")
        .arg("install")
        .output()
        .expect("Failed to install dependencies");

    if !output.status.success() {
        io::stdout().write_all(&output.stderr).unwrap();
    }

    println!("cargo:rerun-if-changed=frontend/");

    let output = Command::new("yarn")
        .arg("build")
        .output()
        .expect("Failed to builed frontend");

    if !output.status.success() {
        io::stdout().write_all(&output.stderr).unwrap();
    }
}


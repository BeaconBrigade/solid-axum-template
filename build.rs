use std::process::Command;

// rebuild front-end
fn main() {
    println!("cargo:rerun-if-changed=frontend/");

    Command::new("yarn")
        .arg("build")
        .output()
        .expect("Failed to builed frontend");
}

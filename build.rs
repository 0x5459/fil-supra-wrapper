use std::{fs, path::Path, process::Command};

fn main() {
    if Path::new(".git/HEAD").exists() {
        println!("cargo:rerun-if-changed=.git/HEAD");
    }

    build_supra_seal();
}

fn build_supra_seal() {
    let supra_seal_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("supra_seal");

    let deps_path = supra_seal_path.join("deps");
    if deps_path.exists() {
        fs::remove_dir_all(deps_path).expect("failed to remove supra_seal deps");
    }

    let output = Command::new(supra_seal_path.join("build.sh"))
        .output()
        .expect("failed to execute `supra_seal/build.sh`");
    println!("{:?}", output);
    dbg!(output);
}

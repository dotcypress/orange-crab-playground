use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use svd2ral::{generate, GeneratorConfig};

const SVD_FILE: &str = "orange_crab.svd";

fn main() {
    let xml = &mut String::new();
    File::open(SVD_FILE).unwrap().read_to_string(xml).unwrap();

    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    generate(&xml, crate_dir.join("src"), &GeneratorConfig::default()).unwrap();

    println!("cargo:rerun-if-changed={}", SVD_FILE);
    println!("cargo:rerun-if-env-changed=FORCE");
}

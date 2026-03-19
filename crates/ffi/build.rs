use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_file = PathBuf::from(&crate_dir)
        .join("include")
        .join("lingcode.h");
    
    // Create include directory
    std::fs::create_dir_all(output_file.parent().unwrap()).unwrap();
    
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("LINGCODE_H")
        .with_style(cbindgen::Style::Both)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(output_file);
}
use pkg_config::{Config, Error};
use std::{env, path::PathBuf};

fn main() {
    common().unwrap();

    let mut builder = bindgen::Builder::default()
        .rustified_enum("*")
        .prepend_enum_name(false)
        .derive_eq(true)
        .size_t_is_usize(true);

    builder = builder.header("src/wrapper.h");

    // Finish the builder and generate the bindings.
    builder
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        .write_to_file(output().join("bindings.rs"))
        .unwrap();
}

fn is_static() -> bool {
    env::var("CARGO_FEATURE_STATIC").is_ok()
}

fn output() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn common() -> Result<(), Error> {
    if let Ok(path) = env::var("XKBCOMMON_LIB_DIR") {
        for lib in &["xkbregistry"] {
            println!(
                "cargo:rustc-link-lib={}={}",
                if is_static() { "static" } else { "dylib" },
                lib
            );
        }

        println!("cargo:rustc-link-search=native={}", path);
    } else {
        Config::new().statik(is_static()).probe("xkbregistry")?;
    }

    Ok(())
}

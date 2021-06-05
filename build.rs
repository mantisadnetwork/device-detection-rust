use std::env;
use std::fs;
use std::option::Option::None;
use std::path::PathBuf;
use fs_extra::dir::{copy, CopyOptions};

use bindgen::callbacks::{DeriveTrait, EnumVariantCustomBehavior, EnumVariantValue, ImplementsTrait, IntKind, MacroParsingBehavior, ParseCallbacks};
use cc::Build;

fn scan(build: &mut Build, path: &str, suffix: &str) {
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        let p = path.unwrap().path();
        let str = p.to_str().unwrap();

        if str.ends_with(suffix) {
            build.file(str);
        }
    }
}

fn main() {
    let mut c = cc::Build::new();

    scan(&mut c, "./device-detection-cxx/src/", ".c");
    scan(&mut c, "./device-detection-cxx/src/hash/", ".c");
    scan(&mut c, "./device-detection-cxx/src/common-cxx/", ".c");

    c.compile("c");

    let source_files = vec!["src/shim/mod.rs"];

    let mut build = cxx_build::bridges(source_files);

    scan(&mut build, "./src/shim/", ".cpp");
    scan(&mut build, "./device-detection-cxx/src/", ".cpp");
    scan(&mut build, "./device-detection-cxx/src/hash/", ".cpp");
    scan(&mut build, "./device-detection-cxx/src/common-cxx/", ".cpp");

    build.include(".");
    build.flag("-std=c++11");
    build.compile("cxxbridge");

    /*
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut c = cc::Build::new();

    scan(&mut c, "./device-detection-cxx/src/", ".c");
    scan(&mut c, "./device-detection-cxx/src/hash/", ".c");
    scan(&mut c, "./device-detection-cxx/src/common-cxx/", ".c");

    c.compile("c");

    let mut cxx = cc::Build::new();
    cxx.cpp(true);
    cxx.flag("-std=c++11");

    scan(&mut cxx, "./device-detection-cxx/src/", ".cpp");
    scan(&mut cxx, "./device-detection-cxx/src/hash/", ".cpp");
    scan(&mut cxx, "./device-detection-cxx/src/common-cxx/", ".cpp");

    cxx.compile("cxx");

    bindgen::Builder::default()
        .header("device-detection-cxx/src/hash/EngineHash.hpp")
        .clang_arg("-std=c++11")
        .opaque_type("std::.*")
        .allowlist_type(".*EngineHash.*")

        // fixes for https://github.com/rust-lang/rust-bindgen/issues/1914
        .blocklist_type("difference_type")
        .blocklist_type("type_")
        .blocklist_type("__node_pointer")

        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("DeviceDetectionCxx.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=dylib=c++");
     */
}

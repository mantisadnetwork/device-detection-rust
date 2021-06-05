use std::fs;
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
    c.warnings(false);

    scan(&mut c, "./device-detection-cxx/src/", ".c");
    scan(&mut c, "./device-detection-cxx/src/hash/", ".c");
    scan(&mut c, "./device-detection-cxx/src/common-cxx/", ".c");

    c.compile("c");

    let source_files = vec!["src/shim/mod.rs"];

    let mut build = cxx_build::bridges(source_files);
    build.warnings(false);

    scan(&mut build, "./src/shim/", ".cpp");
    scan(&mut build, "./device-detection-cxx/src/", ".cpp");
    scan(&mut build, "./device-detection-cxx/src/hash/", ".cpp");
    scan(&mut build, "./device-detection-cxx/src/common-cxx/", ".cpp");

    build.include(".");
    build.flag("-std=c++11");
    build.compile("cxxbridge");

    println!("cargo:rerun-if-changed=src/shim/mod.rs");
    println!("cargo:rerun-if-changed=src/shim/shim.cpp");
    println!("cargo:rerun-if-changed=src/shim/shim.hpp");
}

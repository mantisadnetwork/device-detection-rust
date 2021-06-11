use cc::Build;
use std::fs;

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

#[cfg(feature = "static")]
fn builder() -> Build {
    return cc::Build::new();
}

#[cfg(not(feature = "static"))]
fn builder() -> Build {
    let mut build = cc::Build::new();
    build.static_flag(false);

    return build;
}

fn main() {
    println!("cargo:rerun-if-changed=src/shim/wrapper.h");
    println!("cargo:rerun-if-changed=src/shim/wrapper.c");

    let mut c = builder();
    c.warnings(false);

    scan(&mut c, "./src/shim/", ".c");
    scan(&mut c, "./device-detection-cxx/src/", ".c");
    scan(&mut c, "./device-detection-cxx/src/hash/", ".c");
    scan(&mut c, "./device-detection-cxx/src/common-cxx/", ".c");

    c.compile("51degrees");
}

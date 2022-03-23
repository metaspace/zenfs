fn main() {
    let rocks_cfg = pkg_config::Config::new()
        .atleast_version("7")
        .statik(true)
        .probe("rocksdb")
        .unwrap();
    let mut builder = autocxx_build::Builder::new("src/lib.rs", rocks_cfg.include_paths)
        .extra_clang_args(&["-std=c++17", "-DOS_LINUX"])
        .expect_build();
    builder
        .flag("-std=c++17")
        .flag("-DOS_LINUX")
        .compile("no-name");
    println!("cargo:rerun-if-changed=src/lib.rs");
}

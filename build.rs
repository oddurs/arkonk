fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // The Steam API library ships beside the executable. Linux only searches
    // there when the binary asks; macOS gets it from the library's @loader_path
    // install name and Windows searches the executable's directory first.
    if std::env::var_os("CARGO_FEATURE_STEAM").is_some()
        && std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "linux")
    {
        println!("cargo:rustc-link-arg-bins=-Wl,-rpath,$ORIGIN");
    }
}

use std::{env, io};

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=packaging/icons/arkonk.ico");
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // The Steam API library ships beside the executable. Linux only searches
    // there when the binary asks; macOS gets it from the library's @loader_path
    // install name and Windows searches the executable's directory first.
    if env::var_os("CARGO_FEATURE_STEAM").is_some() && os == "linux" {
        println!("cargo:rustc-link-arg-bins=-Wl,-rpath,$ORIGIN");
    }
    #[cfg(windows)]
    if os == "windows" {
        winresource::WindowsResource::new()
            .set_icon("packaging/icons/arkonk.ico")
            .set("ProductName", "ARKONK")
            .set("FileDescription", "ARKONK")
            .set("LegalCopyright", "Copyright (c) 2026 Oddur")
            .compile()?;
    }
    Ok(())
}

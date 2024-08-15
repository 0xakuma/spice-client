fn main() -> std::io::Result<()> {
    println!("cargo:rustc-link-lib=spice-client-glib-2.0.8");
    println!("cargo:rustc-link-lib=gio-2.0");
    // println!("cargo:rustc-link-lib=static=spice-common");
    println!("cargo:rustc-link-lib=gstreamer-1.0");
    // println!(
    //     "cargo:rustc-link-search=native={}/spice-common/build/common",
    //     env!("CARGO_MANIFEST_DIR")
    // );
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");

    Ok(())
}

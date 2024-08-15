use std::{env, path::PathBuf, process::Command};

fn main() -> std::io::Result<()> {
    println!("cargo:rustc-link-lib=spice-client-glib-2.0.8");
    println!("cargo:rustc-link-lib=gio-2.0");
    // println!("cargo:rustc-link-lib=static=spice-common");
    println!("cargo:rustc-link-lib=gstreamer-1.0");
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    generate_rust_types_from_shader_types();
    compile_shaders();

    Ok(())
}

fn compile_shaders() {
    println!("cargo:rerun-if-changed=shaders.metal");
    println!("cargo:rerun-if-changed=shader_types.h");

    let output = Command::new("xcrun")
        .arg("-sdk")
        .arg("macosx")
        .arg("metal")
        .args(&["-c", "src/shaders.metal"])
        .args(&["-o", "shaders.air"])
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
    if !output.status.success() {
        panic!(
            r#"
stdout: {}
stderr: {}
"#,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        );
    }

    Command::new("xcrun")
        .arg("-sdk")
        .arg("macosx")
        .arg("metallib")
        .arg("shaders.air")
        .args(&["-o", "shaders.metallib"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn generate_rust_types_from_shader_types() {
    println!("cargo:warning=MESSAGE");
    println!("cargo:warning={}", env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header("shader_types/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out = out.join("shader_bindings.rs");

    bindings.write_to_file(out).unwrap();
}

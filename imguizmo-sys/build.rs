use std::env;
use std::path::PathBuf;

fn main() {
    // println!("cargo:rustc-link-lib=cimgui");
    // println!("cargo:rustc-link-lib=ImGuizmo");
    // println!("cargo:rerun-if-changed=wrapper.hpp");

    // cc::Build::new()
    //     .cpp(true)
    //     .include("imgui")
    //     .include("ImGuizmo")
    //     .file("ImGuizmo/ImGuizmo.cpp")
    //     .compile("ImGuizmo");

    // let bindings = bindgen::Builder::default()
    //     .header("wrapper.hpp")
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     .generate()
    //     .expect("Unable to generate bindings");

    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_dir.join("bindings.rs"))
    //     .expect("Could not write bindings!");

    cxx_build::bridge("src/lib.rs")
        .file("ImGuizmo/ImGuizmo.cpp")
        .includes(["ImGuizmo", "imgui", "src"])
        .compile("ImGuizmo");

    println!("cargo:rerun-if-changed=src/lib.rs");
}

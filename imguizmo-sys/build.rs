fn main() {
    // let target_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // for file in target_path
    //     .iter()
    //     .filter(|f| f.to_string_lossy().contains("imgui-sys"))
    // {
    //     let path = std::path::PathBuf::from(file.to_str().unwrap());
    //     let path = path.join("out/third_party/imgui/imgui.o");

    //     if path.exists() {
    //         print!("cargo:rustc-link-lib={}", path.display().to_string());
    //     }
    // }

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
    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    println!("cargo:rerun-if-changed=ImGuizmo/ImGuizmo.cpp");
    println!("cargo:rerun-if-changed=ImGuizmo/ImGuizmo.h");
    println!("cargo:rerun-if-changed=imgui/imgui.h");
}

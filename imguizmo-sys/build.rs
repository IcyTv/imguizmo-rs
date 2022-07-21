fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("ImGuizmo/ImGuizmo.cpp")
        .includes(["ImGuizmo", "imgui", "src"])
        .compile("ImGuizmo");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=ImGuizmo/ImGuizmo.cpp");
    println!("cargo:rerun-if-changed=ImGuizmo/ImGuizmo.h");
    println!("cargo:rerun-if-changed=imgui/imgui.h");
}

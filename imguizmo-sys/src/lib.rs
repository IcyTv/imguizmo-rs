// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]
// #![allow(deref_nullptr)]
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cxx::bridge(namespace = "ImGuizmo")]
mod ffi {
    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    enum OPERATION {
        TRANSLATE_X = 1,
        TRANSLATE_Y = 2,
        TRANSLATE_Z = 4,
        ROTATE_X = 8,
        ROTATE_Y = 16,
        ROTATE_Z = 32,
        ROTATE_SCREEN = 64,
        SCALE_X = 128,
        SCALE_Y = 256,
        SCALE_Z = 512,
        BOUNDS = 1024,
        SCALE_XU = 2048,
        SCALE_YU = 4096,
        SCALE_ZU = 8192,

        TRANSLATE = 7,
        ROTATE = 120,
        SCALE = 896,
        SCALEU = 14336,
        //TRANSLATE | ROTATE | SCALE,
        UNIVERSAL = 14463,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    enum MODE {
        LOCAL,
        WORLD,
    }

    #[namespace = ""]
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct ImVec2 {
        x: f32,
        y: f32,
    }

    unsafe extern "C++" {
        include!("wrapper.h");

        #[namespace = ""]
        type ImDrawList;
        #[namespace = ""]
        type ImGuiContext;
        // #[namespace = ""]
        // type ImU32 = crate::ImU32;

        type OPERATION;
        type MODE;

        unsafe fn SetDrawlist(draw_list: *mut ImDrawList);
        fn BeginFrame();
        unsafe fn SetImGuiContext(context: *mut ImGuiContext);
        fn IsOver() -> bool;
        fn IsUsing() -> bool;

        fn Enable(enable: bool);

        unsafe fn DecomposeMatrixToComponents(
            matrix: *const f32,
            translation: *mut f32,
            rotation: *mut f32,
            scale: *mut f32,
        );
        unsafe fn RecomposeMatrixFromComponents(
            translation: *const f32,
            rotation: *const f32,
            scale: *const f32,
            matrix: *mut f32,
        );

        fn SetRect(x: f32, y: f32, width: f32, height: f32);
        fn SetOrthographic(is_orthographic: bool);

        unsafe fn DrawCubes(
            view: *const f32,
            projection: *const f32,
            matrices: *const f32,
            count: i32,
        );
        unsafe fn DrawGrid(
            view: *const f32,
            projection: *const f32,
            matric: *const f32,
            grid_size: f32,
        );

        unsafe fn Manipulate(
            view: *const f32,
            projection: *const f32,
            operation: OPERATION,
            mode: MODE,
            matrix: *mut f32,
            delta_matrix: *mut f32,
            snap: *const f32,
            local_bounds: *const f32,
            bounds_snap: *const f32,
        ) -> bool;

        unsafe fn ViewManipulate(
            view: *mut f32,
            length: f32,
            position: ImVec2,
            size: ImVec2,
            background_color: u32,
        );
        fn SetID(id: i32);

        #[rust_name = "IsOverOperation"]
        fn IsOver(op: OPERATION) -> bool;

        fn SetGizmoSizeClipSpace(value: f32);
        fn AllowAxisFlip(allow: bool);

    }
}

pub use ffi::*;

impl From<[f32; 2]> for ImVec2 {
    fn from(pos: [f32; 2]) -> Self {
        ImVec2 {
            x: pos[0],
            y: pos[1],
        }
    }
}

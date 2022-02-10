use glam::Mat4;
#[cfg(feature = "imgui-docking")]
pub use imgui_docking as imgui;
#[cfg(feature = "imgui-normal")]
pub use imgui_normal as imgui;

pub use imguizmo_sys as sys;

pub type ImGuizmoMode = sys::OPERATION; //TODO convert to bitflags
pub type ImGuizmoRotationMode = sys::MODE;

pub struct ImGuizmo;
// pub struct ImGuizmo {
//     // allow_axis_flip: bool,
// }

impl ImGuizmo {
    // pub fn new() -> Self {
    //     ImGuizmo {
    //         allow_axis_flip: true,
    //     }
    // }

    pub fn begin_frame() {
        sys::BeginFrame();
    }

    pub fn is_over() -> bool {
        sys::IsOver()
    }

    pub fn is_using() -> bool {
        sys::IsUsing()
    }

    pub fn enable(enable: bool) {
        sys::Enable(enable);
    }

    /// Decompose matrix
    ///
    /// As per docs: "These functions have some numerical stability issues for now. Use with caution."
    ///
    /// Also note that it might be faster to do the matrix composition in native rust code. The reason
    /// these functions exist, is because there might be some wierdness in the implementation, so for
    /// consistency sake.
    pub fn decompose_matrix_to_components(
        matrix: &glam::Mat4,
    ) -> (glam::Vec3, glam::Vec3, glam::Vec3) {
        let mut translation = glam::Vec3::ZERO;
        let mut rotation = glam::Vec3::ZERO;
        let mut scale = glam::Vec3::ZERO;
        unsafe {
            sys::DecomposeMatrixToComponents(
                matrix as *const _ as *const f32,
                &mut translation as *mut _ as *mut f32,
                &mut rotation as *mut _ as *mut f32,
                &mut scale as *mut _ as *mut f32,
            )
        };

        (translation, rotation, scale)
    }

    /// Compose matrix from components
    ///
    /// As per docs: "These functions have some numerical stability issues for now. Use with caution."
    ///
    /// Also note that it might be faster to do the matrix composition in native rust code. The reason
    /// these functions exist, is because there might be some wierdness in the implementation, so for
    /// consistency sake.
    pub fn recompose_matrix_from_components(
        translation: &glam::Vec3,
        rotation: &glam::Vec3,
        scale: &glam::Vec3,
    ) -> glam::Mat4 {
        let mut matrix = glam::Mat4::IDENTITY;
        unsafe {
            sys::RecomposeMatrixFromComponents(
                translation as *const _ as *const f32,
                rotation as *const _ as *const f32,
                scale as *const _ as *const f32,
                &mut matrix as *mut _ as *mut f32,
            )
        };

        matrix
    }

    pub fn draw_cube(view: &Mat4, projection: &Mat4, matrix: &[Mat4]) {
        unsafe {
            sys::DrawCubes(
                view as *const _ as *const f32,
                projection as *const _ as *const f32,
                matrix as *const _ as *const f32,
                matrix.len() as i32,
            )
        };
    }

    #[allow(clippy::too_many_arguments)]
    pub fn manipulate(
        view: &glam::Mat4,
        projection: &glam::Mat4,
        operation: ImGuizmoMode,
        mode: ImGuizmoRotationMode,
        matrix: &mut glam::Mat4,
        delta_matrix: Option<&mut glam::Mat4>,
        snap: Option<&glam::Vec3>,
        local_bounds: Option<&glam::Vec2>,
        bounds_snap: Option<&glam::Vec2>,
    ) {
        let delta_ptr = if let Some(delta_matrix) = delta_matrix {
            delta_matrix
        } else {
            std::ptr::null_mut()
        };
        let snap_ptr = if let Some(snap) = snap {
            snap
        } else {
            std::ptr::null()
        };
        let local_bounds_ptr = if let Some(local_bounds) = local_bounds {
            local_bounds
        } else {
            std::ptr::null()
        };
        let bounds_snap_ptr = if let Some(bounds_snap) = bounds_snap {
            bounds_snap
        } else {
            std::ptr::null()
        };

        unsafe {
            sys::Manipulate(
                view as *const _ as *const f32,
                projection as *const _ as *const f32,
                operation,
                mode,
                matrix as *mut _ as *mut f32,
                delta_ptr as *mut _ as *mut f32,
                snap_ptr as *const _ as *const f32,
                local_bounds_ptr as *const _ as *const f32,
                bounds_snap_ptr as *const _ as *const f32,
            )
        };

        // *matrix = matrix_arr.into();
        // if let Some(delta_matrix) = delta_matrix {
        //     *delta_matrix = matrix_arr.into();
        // }
    }

    pub fn view_manipulate(
        view: &mut glam::Mat4,
        distance: f32,
        position: &glam::Vec2,
        size: &glam::Vec2,
        background_color: u32,
    ) {
        let position = sys::ImVec2 {
            x: position.x,
            y: position.y,
        };

        let size = sys::ImVec2 {
            x: size.x,
            y: size.y,
        };

        unsafe {
            sys::ViewManipulate(
                view as *mut _ as *mut f32,
                distance,
                position,
                size,
                background_color,
            )
        }
    }

    pub fn set_allow_axis_flip(allow_axis_flip: bool) {
        // self.allow_axis_flip = allow_axis_flip;

        sys::AllowAxisFlip(allow_axis_flip);
    }

    /// Sets the draw List to ImGui::GetForegroundDrawList()
    ///
    /// Currently you cannot set your own draw list, since I cannot figure out how to get the
    /// foreground draw list from `imgui-rs`. That means the it does not matter what you put into the option.
    /// This api WILL be broken in the future.
    pub fn set_draw_list(_: Option<()>) {
        unsafe {
            let draw_lists = imgui::sys::igGetWindowDrawList();
            sys::SetDrawlist(draw_lists as *mut _);
        }
        // self.draw_list = Some(*draw_list_ptr);
    }

    pub fn set_rect(x: f32, y: f32, width: f32, height: f32) {
        sys::SetRect(x, y, width, height);
    }

    pub fn set_orthographic(ortho: bool) {
        sys::SetOrthographic(ortho);
    }

    pub fn draw_grid(view: &Mat4, proj: &Mat4, identity: &Mat4, size: f32) {
        unsafe {
            sys::DrawGrid(
                view as *const _ as *const f32,
                proj as *const _ as *const f32,
                identity as *const _ as *const f32,
                size,
            )
        }
    }

    pub fn set_id(id: i32) {
        sys::SetID(id);
    }
}

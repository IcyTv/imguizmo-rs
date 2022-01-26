#[cfg(feature = "imgui-docking")]
pub use imgui_docking as imgui;
#[cfg(feature = "imgui-normal")]
pub use imgui_normal as imgui;

pub use imguizmo_sys as sys;

type ImGuizmoMode = sys::OPERATION; //TODO convert to bitflags
type ImGuizmoRotationMode = sys::MODE;

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

    /// Decompose matrix, can be anyting that implements Into<mint::ColumnMatrix4<f32>>
    ///
    /// As per docs: "These functions have some numerical stability issues for now. Use with caution."
    ///
    /// Also note that it might be faster to do the matrix composition in native rust code. The reason
    /// these functions exist, is because there might be some wierdness in the implementation, so for
    /// consistency sake.
    pub fn decompose_matrix_to_components<M>(
        matrix: M,
    ) -> (mint::Vector3<f32>, mint::Vector3<f32>, mint::Vector3<f32>)
    where
        M: Into<mint::ColumnMatrix4<f32>>,
    {
        let matrix: mint::ColumnMatrix4<f32> = matrix.into();
        let matrix: [f32; 16] = matrix.into();
        let mut translation = [0.0f32; 3];
        let mut rotation = [0.0f32; 3];
        let mut scale = [0.0f32; 3];
        unsafe {
            sys::DecomposeMatrixToComponents(
                matrix.as_ptr(),
                translation.as_mut_ptr(),
                rotation.as_mut_ptr(),
                scale.as_mut_ptr(),
            )
        };

        (translation.into(), rotation.into(), scale.into())
    }

    /// Compose matrix from components, can be anyting that implements Into<mint::ColumnMatrix4<f32>>
    ///
    /// As per docs: "These functions have some numerical stability issues for now. Use with caution."
    ///
    /// Also note that it might be faster to do the matrix composition in native rust code. The reason
    /// these functions exist, is because there might be some wierdness in the implementation, so for
    /// consistency sake.
    pub fn recompose_matrix_from_components<V>(
        translation: V,
        rotation: V,
        scale: V,
    ) -> mint::ColumnMatrix4<f32>
    where
        V: Into<mint::Vector3<f32>>,
    {
        let translation: mint::Vector3<f32> = translation.into();
        let rotation: mint::Vector3<f32> = rotation.into();
        let scale: mint::Vector3<f32> = scale.into();
        let translation: [f32; 3] = translation.into();
        let rotation: [f32; 3] = rotation.into();
        let scale: [f32; 3] = scale.into();

        let mut matrix = [0.0f32; 16];

        unsafe {
            sys::RecomposeMatrixFromComponents(
                translation.as_ptr(),
                rotation.as_ptr(),
                scale.as_ptr(),
                matrix.as_mut_ptr(),
            )
        };

        matrix.into()
    }

    pub fn draw_cube<M>(view: M, projection: M, matrix: &[M])
    where
        M: Into<mint::ColumnMatrix4<f32>> + Copy,
    {
        let view: mint::ColumnMatrix4<f32> = view.into();
        let projection: mint::ColumnMatrix4<f32> = projection.into();
        let view: [f32; 16] = view.into();
        let projection: [f32; 16] = projection.into();

        let matrices: Vec<f32> = matrix
            .iter()
            .map(|m| {
                let col_mat: [f32; 16] = (*m).into().into();
                col_mat
            })
            .flatten()
            .collect::<Vec<_>>();
        unsafe {
            sys::DrawCubes(
                view.as_ptr(),
                projection.as_ptr(),
                matrices.as_ptr(),
                matrices.len() as i32,
            )
        };
    }

    pub fn manipulate<M, V>(
        view: M,
        projection: M,
        operation: ImGuizmoMode,
        mode: ImGuizmoRotationMode,
        matrix: &mut M,
        delta_matrix: Option<&mut M>,
        snap: Option<V>,
        local_bounds: Option<V>,
        bounds_snap: Option<V>,
    ) where
        M: Into<[f32; 16]> + Copy + From<[f32; 16]>, //TODO use mint::ColumnMatrix4<f32>...?
        V: Into<[f32; 3]>,
    {
        let view: [f32; 16] = view.into();
        let projection: [f32; 16] = projection.into();
        let mut matrix_arr: [f32; 16] = (*matrix).into().into();
        // let mut delta_matrix: [f32; 16] = [0.0f32; 16];
        let delta_ptr = if let Some(delta_matrix) = delta_matrix.as_deref() {
            let mut delta_matrix: [f32; 16] = (*delta_matrix).into().into();
            delta_matrix.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };
        let snap_ptr = if let Some(snap) = snap {
            let snap: [f32; 3] = snap.into().into();
            snap.as_ptr()
        } else {
            std::ptr::null()
        };
        let local_bounds_ptr = if let Some(local_bounds) = local_bounds {
            let local_bounds: [f32; 3] = local_bounds.into().into();
            local_bounds.as_ptr()
        } else {
            std::ptr::null()
        };
        let bounds_snap_ptr = if let Some(bounds_snap) = bounds_snap {
            let bounds_snap: [f32; 3] = bounds_snap.into().into();
            bounds_snap.as_ptr()
        } else {
            std::ptr::null()
        };

        unsafe {
            sys::Manipulate(
                view.as_ptr(),
                projection.as_ptr(),
                operation.into(),
                mode.into(),
                matrix_arr.as_mut_ptr(),
                delta_ptr,
                snap_ptr,
                local_bounds_ptr,
                bounds_snap_ptr,
            )
        };

        *matrix = matrix_arr.into();
        if let Some(delta_matrix) = delta_matrix {
            *delta_matrix = matrix_arr.into();
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
            sys::SetDrawlist(std::ptr::null_mut());
        }
        // self.draw_list = Some(*draw_list_ptr);
    }
}

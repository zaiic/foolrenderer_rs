use crate::math::preclude::{Vec2, Vec3, Vec4};
use crate::{vec2_zero, vec3_zero, vec4_zero};

const MAX_F32_VARIABLES: usize = 2;
const MAX_VEC2_VARIABLES: usize = 2;
const MAX_VEC3_VARIABLES: usize = 2;
const MAX_VEC4_VARIABLES: usize = 2;

#[derive(Debug)]
pub struct ShaderContext {
    /* Array to store various types of variables. */
    f32_vars: [f32; MAX_F32_VARIABLES],
    vec2_vars: [Vec2; MAX_VEC2_VARIABLES],
    vec3_vars: [Vec3; MAX_VEC3_VARIABLES],
    vec4_vars: [Vec4; MAX_VEC4_VARIABLES],

    /* Record whether the variable at each index is used. */
    f32_allocs: [bool; MAX_F32_VARIABLES],
    vec2_allocs: [bool; MAX_VEC2_VARIABLES],
    vec3_allocs: [bool; MAX_VEC3_VARIABLES],
    vec4_allocs: [bool; MAX_VEC4_VARIABLES],

    /* Queue of indices of various that have been used. */
    f32_idx_queue: [i8; MAX_F32_VARIABLES],
    vec2_idx_queue: [i8; MAX_VEC2_VARIABLES],
    vec3_idx_queue: [i8; MAX_VEC3_VARIABLES],
    vec4_idx_queue: [i8; MAX_VEC4_VARIABLES],

    /* Number of variables used. */
    f32_var_count: u8,
    vec2_var_count: u8,
    vec3_var_count: u8,
    vec4_var_count: u8,
}

impl Default for ShaderContext {
    fn default() -> Self {
        Self {
        /* Array to store various types of variables. */
        f32_vars: [0.; MAX_F32_VARIABLES],
        vec2_vars: [vec2_zero!(); MAX_VEC2_VARIABLES],
        vec3_vars: [vec3_zero!(); MAX_VEC3_VARIABLES],
        vec4_vars: [vec4_zero!(); MAX_VEC4_VARIABLES],

        /* Record whether the variable at each index is used. */
        f32_allocs: [false; MAX_F32_VARIABLES],
        vec2_allocs: [false; MAX_VEC2_VARIABLES],
        vec3_allocs: [false; MAX_VEC3_VARIABLES],
        vec4_allocs: [false; MAX_VEC4_VARIABLES],

        /* Queue of indices of various that have been used. */
        f32_idx_queue: [0; MAX_F32_VARIABLES],
        vec2_idx_queue: [0; MAX_VEC2_VARIABLES],
        vec3_idx_queue: [0; MAX_VEC3_VARIABLES],
        vec4_idx_queue: [0; MAX_VEC4_VARIABLES],

        /* Number of variables used. */
        f32_var_count: 0,
        vec2_var_count: 0,
        vec3_var_count: 0,
        vec4_var_count: 0,
        }
    }
}

impl ShaderContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        for ref mut i in self.f32_allocs {
            *i = false;
        }
        for ref mut i in self.vec2_allocs {
            *i = false;
        }
        for ref mut i in self.vec3_allocs {
            *i = false;
        }
        for ref mut i in self.vec4_allocs {
            *i = false;
        }
        self.f32_var_count = 0;
        self.vec2_var_count = 0;
        self.vec3_var_count = 0;
        self.vec4_var_count = 0;
    }

    pub fn get_f32(&mut self, index: usize) -> &mut f32 {
        &mut self.f32_vars[index]
    }

    pub fn get_vec2(&mut self, index: usize) -> &mut Vec2 {
        &mut self.vec2_vars[index]
    }

    pub fn get_vec3(&mut self, index: usize) -> &mut Vec3 {
        &mut self.vec3_vars[index]
    }

    pub fn get_vec4(&mut self, index: usize) -> &mut Vec4 {
        &mut self.vec4_vars[index]
    }
}

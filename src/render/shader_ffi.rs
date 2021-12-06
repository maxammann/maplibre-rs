use crate::render::camera;
use crate::render::camera::Camera;

type Vec2f32 = [f32; 2];
type Vec4f32 = [f32; 4];
type Mat4f32 = [Vec4f32; 4];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Globals {
    pub view_proj: Mat4f32, // 64 bytes
    pub view_position: Vec4f32, // 16 bytes
}

impl Globals {
    pub fn new(
        view_proj: Mat4f32,
        view_position: Vec4f32,
    ) -> Self {
        Self {
            view_proj,
            view_position,
        }
    }
}

unsafe impl bytemuck::Pod for Globals {}
unsafe impl bytemuck::Zeroable for Globals {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuVertex {
    pub position: Vec2f32,
    pub normal: Vec2f32,
    pub prim_id: u32,
    _pad1: i32, // _padX aligns it to 8 bytes = AlignOf(Vec2f32=vec2<f32>):
                // https://gpuweb.github.io/gpuweb/wgsl/#alignment-and-size
}

impl GpuVertex {
    pub fn new(position: Vec2f32, normal: Vec2f32, prim_id: u32) -> Self {
        Self {
            position,
            normal,
            prim_id,
            _pad1: Default::default()
        }
    }
}

unsafe impl bytemuck::Pod for GpuVertex {}
unsafe impl bytemuck::Zeroable for GpuVertex {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Primitive {
    pub color: Vec4f32,
    pub translate: Vec2f32,
    pub z_index: i32,
    pub width: f32,
    pub angle: f32,
    pub scale: f32,
    _pad1: i32, // _padX aligns it to 16 bytes = AlignOf(Vec4f32/vec4<f32>):
    _pad2: i32, // https://gpuweb.github.io/gpuweb/wgsl/#alignment-and-size
}

impl Default for Primitive {
    fn default() -> Self {
        Primitive::new([0.0; 4], [0.0; 2], 0, 0.0, 0.0, 1.0)
    }
}
impl Primitive {
    pub fn new(
        color: Vec4f32,
        translate: Vec2f32,
        z_index: i32,
        width: f32,
        angle: f32,
        scale: f32,
    ) -> Self {
        Self {
            color,
            translate,
            z_index,
            width,
            angle,
            scale,
            _pad1: Default::default(),
            _pad2: Default::default(),
        }
    }
}

unsafe impl bytemuck::Pod for Primitive {}
unsafe impl bytemuck::Zeroable for Primitive {}
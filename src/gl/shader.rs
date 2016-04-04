use super::*;
pub trait Shader {
    fn vertex(&mut self, camera: &super::Camera, model: &super::Model, iface: i32, nthvert: i32) -> Vec3i;
    fn fragment(&self, bar: Vec3f, color: &mut TGAColor) -> bool;
}

#[allow(dead_code)]
pub struct GourauShader {
    vary_intensity: Vec3f,
}

impl GourauShader {
    pub fn new() -> GourauShader { GourauShader { vary_intensity: Vec3f::zero() } }
}

impl Shader for GourauShader {
    fn vertex(&mut self, camera: &super::Camera, model: &super::Model,iface: i32, nthvert: i32) -> Vec3i {
        let mut gl_vertex = model.face_vert(iface, nthvert).embed(1.0);
        gl_vertex = camera.viewport * camera.projection * camera.modelview * gl_vertex;
        self.vary_intensity[nthvert as usize] = 0.0.max(model.face_normal(iface, nthvert) * camera.light_dir);
        (gl_vertex / gl_vertex[3]).proj().cast::<i32>()
    }
    fn fragment(&self, bar: Vec3f, color: &mut TGAColor) -> bool {
        let intensity = self.vary_intensity * bar;
        *color = TGAColor::with_color(RGBAColor(255, 255, 255, 255)) * intensity; 
        false
    } 
}


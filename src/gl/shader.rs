use super::*;
pub trait Shader {
    fn vertex(&mut self, camera: &super::Camera, model: &mut super::Model, iface: i32, nthvert: i32) -> Vec3i;
    fn fragment(&self, bar: Vec3f, color: &mut TGAColor) -> bool;
}

#[allow(dead_code)]
pub struct GourauShader {
    vary_intensity: Vec3f,
}

macro_rules! clamp {
    ($val: expr, $low: expr, $high: expr) => (
        if $val < $low { $low }
        else if $val > $high { $high }
        else { $val } 
    );
}

impl GourauShader {
    pub fn new() -> GourauShader { GourauShader { vary_intensity: Vec3f::zero() } }
}

impl Shader for GourauShader {
    fn vertex(&mut self, camera: &super::Camera, model: &mut super::Model,iface: i32, nthvert: i32) -> Vec3i {
        let mut gl_vertex = model.face_vert(iface, nthvert).embed(1.0);
        gl_vertex = camera.viewport * camera.projection * camera.modelview * gl_vertex;
        let cmp = model.face_normal(iface, nthvert) * camera.light_dir;
        let clamp = clamp!(cmp, 0.0, 1.0);
        self.vary_intensity[nthvert as usize] = clamp;
        (gl_vertex / gl_vertex[3]).proj().cast::<i32>()
    }
    fn fragment(&self, bar: Vec3f, color: &mut TGAColor) -> bool {
        let intensity = self.vary_intensity * bar;
        *color = TGAColor::with_color(RGBAColor(255, 255, 255, 255)) * intensity; 
        if color.red() == 0 && color.green() == 0 && color.blue() == 0 { print!("z "); }
        false
    } 
}


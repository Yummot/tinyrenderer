use gl::*;

pub trait Shader {
    fn vertex(&mut self, camera: &super::Camera, model: &mut super::Model, iface: i32, nthvert: i32) -> Vec4f;
    fn fragment(&self, bar: Vec3f, color: &mut Color) -> bool;
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
    fn vertex(&mut self, camera: &super::Camera, model: &mut super::Model,iface: i32, nthvert: i32) -> Vec4f {
        let gl_vertex = model.face_vert(iface, nthvert).embed(1.0);
        self.vary_intensity[nthvert as usize] = 0.0.max(model.face_normal(iface, nthvert) * camera.light_dir);
        camera.viewport * camera.projection * camera.modelview * gl_vertex
    }
    fn fragment(&self, bar: Vec3f, color: &mut Color) -> bool {
        let intensity = self.vary_intensity * bar;
        *color = Color::with_color(RGBAColor(255, 255, 255, 255)) * intensity; 
        false
    } 
}

#[allow(dead_code)]
pub struct ToonShader {
    vary_intensity: Vec3f,
    vary_mat3: Mat3,
}

impl ToonShader {
    pub fn new() -> ToonShader { ToonShader { vary_intensity: Vec3f::zero() , vary_mat3: Mat3::zero() } }
}

impl Shader for ToonShader {
    fn vertex(&mut self, camera: &super::Camera, model: &mut super::Model, iface: i32, nthvert: i32) -> Vec4f {
        let mut gl_vertex = model.face_vert(iface, nthvert).embed(1.0);
        gl_vertex = camera.projection * camera.modelview * gl_vertex;
        let proj = (gl_vertex / gl_vertex[3]).proj3();
        for i in 0..self.vary_mat3.ncols() as usize {
            self.vary_mat3[nthvert as usize][i] = proj[i];
        } 
        self.vary_intensity[nthvert as usize] = clamp!(model.face_normal(iface, nthvert) * camera.light_dir, 0.0, 1.0);
        gl_vertex = camera.viewport * gl_vertex;
        gl_vertex
    }
    fn fragment(&self, bar: Vec3f, color: &mut Color) -> bool {
        let mut intensity = self.vary_intensity * bar;
        if intensity > 0.85 { intensity = 1.0; }
        else if intensity > 0.60 { intensity = 0.80; }
        else if intensity > 0.45 { intensity = 0.60; }
        else if intensity > 0.30 { intensity = 0.45; }
        else if intensity > 0.15 { intensity = 0.30; } 
        *color = Color::with_color(RGBAColor(255,255,255,255)) * intensity;
        false 
    }    
}

#[allow(dead_code)]
pub struct FlatShader {
    vary_mat3: Mat3,
    light_dir: Vec3f,
}

impl FlatShader {
    pub fn new() -> Self { FlatShader { vary_mat3: Mat3::zero(), light_dir: Vec3f::zero() } }
}

impl Shader for FlatShader {
    fn vertex(&mut self, camera: &super::Camera, model: &mut super::Model, iface: i32, nthvert: i32) -> Vec4f {
        self.light_dir = camera.light_dir;
        let mut gl_vertex = model.face_vert(iface, nthvert).embed(1.0);
        gl_vertex = camera.projection * camera.modelview * gl_vertex;
        let proj = (gl_vertex / gl_vertex[3]).proj3();
        for i in 0..self.vary_mat3.ncols() as usize {
            self.vary_mat3[nthvert as usize][i] = proj[i];
        } 
        gl_vertex = camera.viewport * gl_vertex;
        gl_vertex
    }
    #[allow(unused_variables)]
    fn fragment(&self, bar: Vec3f, color: &mut Color) -> bool {
        let n = cross(Vec3f::from_vec(&self.vary_mat3[1]) - Vec3f::from_vec(&self.vary_mat3[0]), Vec3f::from_vec(&self.vary_mat3[2]) - Vec3f::from_vec(&self.vary_mat3[0])).normalize();
        let intensity = clamp!(n * self.light_dir, 0.0, 1.0);
        *color = Color::with_color(RGBAColor(255,255,255,255)) * intensity;
        false 
    }    
}


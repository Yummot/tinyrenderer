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
            self.vary_mat3[i][nthvert as usize] = proj[i];
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

#[allow(dead_code)]
pub struct DepthShader{
    vary_mat3: Mat3,
    depth: f32,
}

impl DepthShader {
    #[allow(dead_code)]
    pub fn new(depth: f32) -> DepthShader {
        DepthShader { vary_mat3: Mat3::zero(), depth: depth }
    } 
}

impl Shader for DepthShader {
    fn vertex(&mut self, camera: &super::Camera, model: &mut super::Model, iface: i32, nthvert: i32) -> Vec4f {
        let mut gl_vertex = model.face_vert(iface, nthvert).embed(1.0);
        gl_vertex = camera.viewport * camera.projection * camera.modelview * gl_vertex;
        let proj = (gl_vertex / gl_vertex[3]).proj3();
        self.vary_mat3[0][nthvert as usize] = proj[1];
        self.vary_mat3[1][nthvert as usize] = proj[1];
        self.vary_mat3[2][nthvert as usize] = proj[2];
        gl_vertex    
    }
    fn fragment(&self, bar: Vec3f, color: &mut Color) -> bool {
        let p = self.vary_mat3 * bar;
        *color = Color::with_color(RGBAColor(255, 255, 255, 255)) * (p.z / self.depth);
        false
    }
}

#[allow(dead_code)]
pub struct IShader<'a> {
    uniform_m: Mat4,
    uniform_mit: Mat4,
    uniform_mshadow: Mat4,
    varying_uv: [Vec3f;2],
    vary_mat3: Mat3,
    model_cache: Option<Model>,
    light_dir: Vec3f,        
    shadowbuffer: Option<&'a Vec<f32>>,
    height: usize,
    width: usize,
}

#[allow(dead_code)]
pub struct IShaderBuilder<'a> {
    m: Mat4,
    mit: Mat4,
    mshadow: Mat4,
    model_cache: Option<Model>,
    light_dir: Vec3f,        
    shadowbuffer: Option<&'a Vec<f32>>,
    height: usize,
    width: usize,    
}

impl<'a> IShaderBuilder<'a> {
    #[allow(dead_code)]
    pub fn new(m: Mat4, mit: Mat4, mshadow: Mat4) -> IShaderBuilder<'a> {
        IShaderBuilder {
            m: m,
            mit: mit,
            mshadow: mshadow, 
            model_cache: None,
            light_dir: Vec3f::zero(),
            shadowbuffer: None,
            height: 0,
            width: 0,
        }
    }
    #[allow(dead_code)]
    pub fn model(mut self, model: Model) -> IShaderBuilder<'a> { self.model_cache = Some(model); self }
    #[allow(dead_code)]
    pub fn light_dir(mut self, light_dir: Vec3f) -> IShaderBuilder<'a>{ self.light_dir = light_dir; self }
    #[allow(dead_code)]
    pub fn size(mut self, w: usize, h: usize) -> IShaderBuilder<'a> { self.width = w; self.height = h; self }
    #[allow(dead_code)]
    pub fn shadowbuffer(mut self, buffer: &'a Vec<f32>) -> IShaderBuilder<'a> { self.shadowbuffer = Some(buffer); self }
    #[allow(dead_code)]
    pub fn build(self) -> IShader<'a> {
        IShader {
            uniform_m: self.m,
            uniform_mit: self.mit,
            uniform_mshadow: self.mshadow,
            varying_uv: [Vec3f::zero();2],
            vary_mat3: Mat3::zero(),
            model_cache: self.model_cache,
            light_dir: self.light_dir,        
            shadowbuffer: self.shadowbuffer,
            height: self.height,
            width: self.width,    
        }
    }
}

impl<'a> Shader for IShader<'a> {
    fn vertex(&mut self, camera: &super::Camera, model: &mut super::Model, iface: i32, nthvert: i32) -> Vec4f {
        let tmp = model.uv(iface as usize, nthvert as usize);
        
        self.varying_uv[0][nthvert as usize] = tmp[0];
        self.varying_uv[1][nthvert as usize] = tmp[1];
        
        let gl_vertex = camera.viewport * camera.projection * camera.modelview * model.face_vert(iface, nthvert).embed(1.0);
        let proj = (gl_vertex / gl_vertex[3]).proj3();
          
        self.vary_mat3[0][nthvert as usize] = proj[0];
        self.vary_mat3[1][nthvert as usize] = proj[1];
        self.vary_mat3[2][nthvert as usize] = proj[2];
        
        gl_vertex
    }
    fn fragment(&self, bar: Vec3f, color: &mut Color) -> bool {
        let mut sb_p = self.uniform_mshadow * (self.vary_mat3 * bar).embed(1.0);
        sb_p = sb_p / sb_p[3];
        let idx = (sb_p[0] + sb_p[1] * self.width as f32) as usize;
        let shadow = 0.3 + 0.7 * if self.shadowbuffer.map(|x| x[idx]).unwrap() < sb_p[2] { 1.0 } else { 0.0 };
        let uv = {
            let mut ret = Vec2f::zero();
            ret[0] = self.varying_uv[0] * bar;
            ret[1] = self.varying_uv[1] * bar;
            ret
        };
        
        let (tmp, spec_exp, c) = match self.model_cache {
            None => return true,
            Some(ref model) => (model.normal(uv).embed(1.0), model.specular(uv), model.diffuse(uv)),   
        };
        
        let n = (self.uniform_mit * tmp).normalize().proj3();
        let l = (self.uniform_m * self.light_dir.embed(1.0)).normalize().proj3();
        let r = (n * (n * l * 0.2) - l).normalize();
        let spec = 0.0f32.max(r.z).powf(spec_exp);
        let diff = 0.0f32.max(n * l);
        *color = Color::with_color(RGBAColor(255,255,255,255));
        for i in 0..3 {
            color[i] = 255.0f32.min(20.0 + c[i] as f32 * shadow * (1.2 * diff + 0.6 * spec)) as u8;
        }
        let wc = *color;
        let st = true;
        false
    }
}
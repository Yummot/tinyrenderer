#[macro_use]
pub mod geometry;
pub mod model;
pub mod tga_image;
pub mod color;
pub mod shader;
pub use self::tga_image::*;
pub use self::geometry::*;
pub use self::model::*;
pub use self::shader::*;
pub use self::color::*;
use super::std;
extern crate num;

pub trait Cast {
    type Output;
    fn cast<T>(&self) -> Self::Output;
}

#[allow(dead_code)]
pub fn line(mut p0: Vec3i, mut p1: Vec3i, image: &mut TGAImage, color: Color) {
     let mut steep = false;
     if (p0.x - p1.x).abs() < (p0.y - p1.y).abs() {
         std::mem::swap(&mut p0.x, &mut p0.y);
         std::mem::swap(&mut p1.x, &mut p1.y);
         steep = true;
     }
     if p0.x > p1.x {
         std::mem::swap(&mut p0, &mut p1);
     }
     for x in p0.x..(p1.x + 1) {
         let t = (x as f32 - p0.x as f32) / (p1.x as f32 - p0.x as f32);
         let y = p0.y as f32 * (1.0 - t) + p1.y as f32 * t + 0.5;
         if steep {
             image.set(y, x, color);
         } else {
             image.set(x, y, color);
         }
     }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
fn barycentric(A: Vec2f, B: Vec2f, C: Vec2f, P: Vec2f) -> Vec3f {
    let mut s = [Vec3f::zero();2];
    for i in 0..2 {
        s[i][0] = C[i] - A[i];
        s[i][1] = B[i] - A[i];
        s[i][2] = A[i] - P[i];
    }
    
    let u = cross::<f32>(s[0], s[1]);
    if u[2].abs() > 1e-2 {
        return Vec3f::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
    }
    return Vec3f::new(-1.0, 1.0, 1.0)    
}

pub fn triangle<S: Shader>(pts: &mut [Vec4f], shader: &S, image: &mut TGAImage, zbuffer: &mut Vec<f32>) {
    let mut bboxmin = Vec2f::new(std::f32::MAX, std::f32::MAX);
    let mut bboxmax = Vec2f::new(std::f32::MIN, std::f32::MIN);
    for i in 0..3 {
        bboxmin[0] = bboxmin[0].min(pts[i][0] / pts[i][3]);
        bboxmax[0] = bboxmax[0].max(pts[i][0] / pts[i][3]);    
        bboxmin[1] = bboxmin[1].min(pts[i][1] / pts[i][3]);
        bboxmax[1] = bboxmax[1].max(pts[i][1] / pts[i][3]);  
    }
    
    let mut color = Color::new();
    for x in (bboxmin.x as i32)..(bboxmax.x as i32 + 1) {
        for y in (bboxmin.y as i32)..(bboxmax.y as i32 + 1) {
            let c = barycentric((pts[0] / pts[0][3]).proj2(), (pts[1] / pts[1][3]).proj2(), (pts[2] / pts[2][3]).proj2(), Vec2i::new(x, y).cast::<f32>());
            let z = pts[0][2] * c.x + pts[1][2] * c.y + pts[2][2] * c.z;
            let w = pts[0][3] * c.x + pts[1][3] * c.y + pts[2][3] * c.z;
            let frag_depth = z / w;
            if c.x < 0.0 || c.y < 0.0 || c.z < 0.0 || zbuffer[(x + y * image.get_width()) as usize] > frag_depth { 
                continue 
            }
            let discard = shader.fragment(c, &mut color);
            if !discard {
                zbuffer[(x + y * image.get_width()) as usize] = frag_depth;
                image.set(x, y, color);
            }
        } 
    }
}

#[allow(dead_code)]
pub struct Camera {
    pub modelview: Mat4,
    pub viewport: Mat4,
    pub projection: Mat4,
    pub light_dir: Vec3f,
    pub depth: f32,
}

// pub static mut CameraOne: Camera = Camera {
//     modelview: Mat4 { mat: [0f32;16] },
//     viewport: Mat4 { mat: [0f32;16] },
//     projection: Mat4 { mat: [0f32;16] },
//     light_dir: Vec3f { x: 1.0, y: 1.0, z: 1.0 }
// };


impl Camera {
    #[allow(dead_code)]
    pub fn new(depth: f32) -> Camera {
        Camera {
            modelview: Mat4::zero(),
            viewport: Mat4::zero(),
            projection: Mat4::zero(),
            light_dir: Vec3f::zero(),
            depth: depth,
        }    
    }
    pub fn set_light_dir(&mut self, light_dir: Vec3f) {
        self.light_dir = light_dir;
    }
    #[allow(dead_code)]
    pub fn viewport(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.viewport = Mat4::identity();
        self.viewport[(0,3)] = x as f32 + w as f32 / 2.0;
        self.viewport[(1,3)] = y as f32 + h as f32 / 2.0;
        self.viewport[(2,3)] = self.depth / 2.0;
        self.viewport[(0,0)] = w as f32 / 2.0;
        self.viewport[(1,1)] = h as f32 / 2.0;
        self.viewport[(2,2)] = self.depth / 2.0;
    }
    #[allow(dead_code)]
    pub fn projection(&mut self, coeff: f32) {
        self.projection = Mat4::identity();
        self.projection[(3,2)] = coeff;
    }
    #[allow(dead_code)]
    pub fn lookat(&mut self, eye: Vec3f, center: Vec3f, up: Vec3f) {
        let z = (eye-center).normalize();
        let x = cross(up,z).normalize();
        let y = cross(z,x).normalize();
        self.modelview = Mat4::identity();
        for i in 0..3 {
            self.modelview[(0,i)] = x[i];
            self.modelview[(1,i)] = y[i];
            self.modelview[(2,i)] = z[i];
            self.modelview[(i,3)] = -center[i];
        }        
    }
}
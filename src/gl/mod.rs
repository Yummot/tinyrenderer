#[macro_use]
pub mod geometry;
pub mod model;
pub mod tga_image;
pub mod shader;
pub use self::tga_image::*;
pub use self::geometry::*;
pub use self::model::*;
pub use self::shader::*;
use super::std;

#[allow(dead_code)]
pub fn line(mut p0: Vec3i, mut p1: Vec3i, image: &mut TGAImage, color: TGAColor) {
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
fn barycentric(A: Vec3i, B: Vec3i, C: Vec3i, P: Vec3i) -> Vec3f {
    let mut s = [Vec3i::zero();2];
    for i in 0..2 {
        s[i][0] = C[i] - A[i];
        s[i][1] = B[i] - A[i];
        s[i][2] = A[i] - P[i];
    }
    
    let u = cross::<f32>(s[0].cast::<f32>(), s[1].cast::<f32>());
    if u[2].abs() > 1e-2 {
        return Vec3f::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
    }
    return Vec3f::new(-1.0, 1.0, 1.0)    
}

// fn barycentric(points: &[Vec2i], point: Vec2i) -> Vec3f {
//     let u = cross(Vec3f::new(points[2][0]-points[0][0], points[1][0]-points[0][0], points[0][0]-point[0]), Vec3f::new(points[2][1]-points[0][1], points[1][1]-points[0][1], points[0][1]-point[1])); 
//     if u[2].abs() < 1.0 { return Vec3f::new(-1.0,1.0,1.0); } // triangle is degenerate, in this case return smth with negative coordinates 
//     let ret = Vec3f::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
//     return ret
// }

pub fn triangle<S: Shader>(pts: &mut [Vec3i], shader: &S, image: &mut TGAImage, zbuffer: &mut TGAImage) -> usize {
    let mut bboxmin = Vec2i::new(std::i32::MAX, std::i32::MAX);
    let mut bboxmax = Vec2i::new(std::i32::MIN, std::i32::MIN);
    let mut ret = 0;
    for i in 0..3 {
        bboxmin[0] = std::cmp::min(bboxmin[0], pts[i][0]);
        bboxmax[0] = std::cmp::max(bboxmax[0], pts[i][0]);    
        bboxmin[1] = std::cmp::min(bboxmin[1], pts[i][1]);
        bboxmax[1] = std::cmp::max(bboxmax[1], pts[i][1]);  
    }
    let mut p = Vec3i::new(bboxmin.x, bboxmin.y, 0);
    let mut color = TGAColor::new();
    while p.x <= bboxmax.x {
        p.y = bboxmin.y;
        while p.y <= bboxmax.y {
            let c = barycentric(pts[0], pts[1], pts[2], p);
            p.z = 0.0.max(255.0.min((pts[0].z as f32 * c.x + pts[1].z as f32 * c.y + pts[2].z as f32 * c.z + 0.5))) as i32;
            if c.x < 0.0 || c.y < 0.0 || c.z < 0.0 || zbuffer.get(p.x, p.y)[0] as i32 > p.z { 
                p.y += 1;
                continue 
            }
            let discard = shader.fragment(c, &mut color);
            if !discard {
                zbuffer.set(p.x, p.y, TGAColor::grayscale(p.z as u8));
                image.set(p.x, p.y, color);
                ret += 1;
            }
            p.y += 1;
        }
        p.x += 1;    
    }
    ret
}

#[allow(dead_code)]
pub struct Camera {
    modelview: Mat4,
    viewport: Mat4,
    projection: Mat4,
    pub light_dir: Vec3f,
}

// pub static mut CameraOne: Camera = Camera {
//     modelview: Mat4 { mat: [0f32;16] },
//     viewport: Mat4 { mat: [0f32;16] },
//     projection: Mat4 { mat: [0f32;16] },
//     light_dir: Vec3f { x: 1.0, y: 1.0, z: 1.0 }
// };


impl Camera {
    #[allow(dead_code)]
    pub fn new() -> Camera {
        Camera {
            modelview: Mat4::zero(),
            viewport: Mat4::zero(),
            projection: Mat4::zero(),
            light_dir: Vec3f::zero(),
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
        self.viewport[(2,3)] = 255.0 / 2.0;
        self.viewport[(0,0)] = w as f32 / 2.0;
        self.viewport[(1,1)] = h as f32 / 2.0;
        self.viewport[(2,2)] = 255.0 / 2.0;
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
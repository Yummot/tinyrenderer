#[macro_use]
mod geometry;
mod model;
#[cfg(test)]
mod tests;
mod tga_image;
use tga_image::*;
use geometry::*;


#[allow(dead_code)]
fn line(mut p0: Vec3i, mut p1: Vec3i, image: &mut TGAImage, color: TGAColor) {
     let mut steep = false;
     if (p0.x - p1.x).abs() < (p0.y - p1.y).abs() {
         std::mem::swap(&mut p0.x, &mut p0.y);
         std::mem::swap(&mut p1.x, &mut p1.y);
         steep = true;
     }
     if p0.x > p1.x {
         std::mem::swap(&mut p0, &mut p1);
     }
     for x in p0.x..p1.x {
         let t = (x as f32 - p0.x as f32) / (p1.x as f32 - p0.x as f32);
         let y = p0.y as f32 * (1.0 - t) + p1.y as f32 * t + 0.5;
         if steep {
             image.set(y, x, color);
         } else {
             image.set(x, y, color);
         }
     }
}

#[allow(dead_code)]
fn mat_to_vec3f(m: &Mat) -> Vec3f {
    Vec3f::new(m[0][0] / m[3][0], m[1][0] / m[3][0], m[2][0] / m[3][0])
}
#[allow(dead_code)]
fn vec3f_to_mat(v: Vec3f) -> Mat {
    Mat::builder(&[&[v.x],&[v.y],&[v.z],&[1.0]]).unwrap()
}
#[allow(dead_code)]
fn viewport(x: u32, y: u32, w: u32, h: u32, depth: u32) -> Mat {
    let mut ret = Mat::identity(4);
    ret[0][3] = x as f32 + w as f32 / 2.0;
    ret[1][3] = y as f32 + h as f32 / 2.0;
    ret[2][3] = depth as f32 / 2.0;

    ret[0][0] = w as f32 / 2.0;
    ret[1][1] = h as f32 / 2.0;
    ret[2][2] = depth as f32 / 2.0; 
    
    ret
}
#[allow(dead_code)]
fn translation(v: Vec3f) -> Mat {
    let mut ret = Mat::identity(4);
    ret[0][3] = v.x;
    ret[1][3] = v.y;
    ret[2][3] = v.z;
    ret
}
#[allow(dead_code)]
fn zoom(factor: f32) -> Mat {
    let mut z = Mat::identity(4);
    z[0][0] = factor;
    z[1][1] = factor;
    z[2][2] = factor;
    z
}
#[allow(dead_code)]
fn rotation_x(cosangle: f32, sinangle: f32) -> Mat {
    let mut r = Mat::identity(4);
    r[1][1] = cosangle;
    r[2][2] = cosangle;
    r[1][2] = -sinangle;
    r[2][1] =  sinangle;
    r
}
#[allow(dead_code)]
fn rotation_y(cosangle: f32, sinangle: f32) -> Mat {
    let mut r = Mat::identity(4);
    r[0][0] = cosangle;
    r[2][2] = cosangle;
    r[0][2] =  sinangle;
    r[2][0] = -sinangle;
    r
}
#[allow(dead_code)]
fn rotation_z(cosangle: f32, sinangle: f32) -> Mat {
    let mut r = Mat::identity(4);
    r[0][0] = cosangle;
    r[1][1] = cosangle;
    r[0][1] = -sinangle;
    r[1][0] =  sinangle;
    r
}

#[allow(non_snake_case)]
#[allow(dead_code)]
fn triangle(pts: &mut [Vec3i], image: &mut TGAImage, model: &model::Model, uv: &mut [Vec2i], intensity: f32, zbuffer: &mut [i32]) {   
    if (pts[0].y == pts[1].y &&  pts[0].y == pts[2].y) && (pts[0].x == pts[1].x &&  pts[0].x == pts[2].x) { return }
    if pts[0].y > pts[1].y { pts.swap(0,1); uv.swap(0,1); }
    if pts[0].y > pts[2].y { pts.swap(0,2); uv.swap(0,2); }
    if pts[1].y > pts[2].y { pts.swap(1,2); uv.swap(1,2); }   
    
    let total_height = pts[2].y -pts[0].y;
    for i in 0..total_height {
        let second_half = i > (pts[1].y - pts[0].y) || pts[1].y == pts[0].y;
        let segment_height = if second_half { pts[2].y - pts[1].y } else { pts[1].y - pts[0].y } as f32;
        let alpha = i as f32 / total_height as f32;
        let beta = if second_half { (i - pts[1].y + pts[0].y) as f32 / segment_height }
                   else { i as f32 / segment_height };
        
        let mut A = pts[0].check_add(&((pts[2] - pts[0]) * alpha).cast::<Vec3f>());
        let mut B = if second_half { 
            pts[1].check_add(&((pts[2] - pts[1]) * beta).cast::<Vec3f>()) 
            } else { 
                pts[0].check_add(&((pts[1] - pts[0]) * beta).cast::<Vec3f>()) 
            };
        let mut uvA = uv[0] + (uv[2] - uv[0]) * alpha;
        let mut uvB = if second_half { uv[1] + (uv[2] - uv[1]) * beta } else { uv[0] + (uv[1] - uv[0]) * beta };
        
        if A.x > B.x { 
            std::mem::swap(&mut A, &mut B); 
            std::mem::swap(&mut uvA, &mut uvB);
        }
        for j in A.x..(B.x + 1) {
            let phi = if B.x == A.x { 1. }
                      else { (j - A.x) as f32 / (B.x - A.x) as f32 };
            let p = (A.cast::<Vec3f>() + ((B - A) * phi).cast::<Vec3f>()).cast::<Vec3i>();
            
            let uvp = uvA + (uvB - uvA) *phi;
            let idx = (p.x + p.y * image.get_width()) as usize;
            if zbuffer[idx] < p.z {
                zbuffer[idx] = p.z;
                let color = model.diffuse(uvp);

                image.set(p.x, p.y, 
                    TGAColor::with_color(
                        RGBAColor(
                            (color.r as f32 * intensity) as u8, 
                            (color.g as f32 * intensity) as u8, 
                            (color.b as f32 * intensity) as u8, 
                            0)));
            }
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let width = 100;
    let height = 100;
    let depth = 255;
    let args: Vec<String> = std::env::args().collect();
    
    let white = TGAColor::with_color(RGBAColor(255,255,255,255));
    let red = TGAColor::with_color(RGBAColor(255,0,0,255));
    let green = TGAColor::with_color(RGBAColor(0,255,0,255));
    let blue = TGAColor::with_color(RGBAColor(0,0,255,255));
    let yellow = TGAColor::with_color(RGBAColor(255,255,0,255));
    
    let model = if args.len() == 1 { model::Model::open("obj/cube.obj") }
                   else if args.len() == 2 { 
                       if args[1].find(".obj") != None { model::Model::open(&args[1]) } 
                       else { panic!("Error: Parameter: {} is not an obj file.", args[1]); }
                   }
                   else { panic!("Too many parameters input."); };
    
    let mut image = TGAImage::with_info(width, height, tga_image::RGB);
    let vp = viewport(width as u32 / 4, width as u32 / 4, width as u32 / 2, height as u32 / 2, depth);
    
    {   
        let x = mat_to_vec3f(&(vp.mul(&vec3f_to_mat(Vec3f::new(1.0, 0.0, 0.0))))).cast::<Vec3i>();
        let y = mat_to_vec3f(&(vp.mul(&vec3f_to_mat(Vec3f::new(0.0, 1.0, 0.0))))).cast::<Vec3i>();
        let o = mat_to_vec3f(&(vp.mul(&vec3f_to_mat(Vec3f::new(0.0, 0.0, 0.0))))).cast::<Vec3i>();
        line(o, x, &mut image, red);
        line(o, y, &mut image, green);
    }
    

    let face = model.face(0);
    for j in 0..face.len() {
        let wp0 = model.vert(face[j] as usize);
        let wp1 = model.vert(face[(j + 1) % face.len()] as usize);
        {
            let sp0 = mat_to_vec3f(&(vp.mul(&vec3f_to_mat(wp0)))).cast::<Vec3i>();
            let sp1 = mat_to_vec3f(&(vp.mul(&vec3f_to_mat(wp1)))).cast::<Vec3i>();
            line(sp0, sp1, &mut image, white);
        }
        {
            let t = zoom(1.5);
            let sp0 = mat_to_vec3f(&(vp.mul(&t.mul(&vec3f_to_mat(wp0))))).cast::<Vec3i>();
            let sp1 = mat_to_vec3f(&(vp.mul(&t.mul(&vec3f_to_mat(wp1))))).cast::<Vec3i>();
            line(sp0, sp1, &mut image, yellow);
        }
    }
        
    
    image.flip_vertically().unwrap();
    image.write_tga_file("output.tga", tga_image::WRITE_RLE_FILE).unwrap();
    println!("Finished");
}
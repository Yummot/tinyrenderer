#[macro_use]
mod geometry;
mod model;
#[cfg(test)]
mod tests;
mod tga_image;
use tga_image::*;
use geometry::*;

//static WHITE: TGAColor = TGAColor { bytespp: 3, r: 255, g: 255, b: 255, a: 255 };
// const RED: TGAColor = TGAColor { bytespp: 3, r: 255, g: 0, b: 0, a: 255 };
// const GREEN: TGAColor = TGAColor { bytespp: 3, r: 0, g: 255, b: 0, a: 255 };
//const BLUE: TGAColor = TGAColor { bytespp: 3, r: 0, g: 0, b: 255, a: 255 };

#[allow(dead_code)]
fn line(mut p0: Vec2i, mut p1: Vec2i, image: &mut TGAImage, color: TGAColor) {
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
         let y = p0.y as f32 * (1.0 - t) + p1.y as f32 * t;
         if steep {
             image.set(y, x, color);
         } else {
             image.set(x, y, color);
         }
     }
}


#[allow(non_snake_case)]
fn triangle(pts: &mut [Vec3i], image: &mut TGAImage, color: TGAColor, zbuffer: &mut [i32]) {   
    if (pts[0].y == pts[1].y &&  pts[0].y == pts[1].y) && (pts[0].x == pts[1].x &&  pts[0].x == pts[2].x) { return }
    if pts[0].y > pts[1].y { pts.swap(0,1); }
    if pts[0].y > pts[2].y { pts.swap(0,2); }
    if pts[1].y > pts[2].y { pts.swap(1,2); }   
    
    let total_height = pts[2].y -pts[0].y;
    for i in 0..total_height {
        let second_half = i > (pts[1].y - pts[0].y) || pts[1].y == pts[0].y;
        let segment_height = if second_half { pts[2].y - pts[1].y } else { pts[1].y - pts[0].y } as f32;
        let alpha = i as f32 / total_height as f32;
        let beta = if second_half { (i - pts[1].y + pts[0].y) as f32 / segment_height }
                   else { i as f32 / segment_height };
        
        let mut A = pts[0] + (pts[2] - pts[0]).mul_num(alpha);
        let mut B = if second_half { pts[1] + (pts[2] - pts[1]).mul_num(beta) } else { pts[0] + (pts[1] - pts[0]).mul_num(beta) };
        
        if A.x > B.x { std::mem::swap(&mut A, &mut B); }
        for j in A.x..(B.x + 1) {
            let phi = if B.x == A.x { 1. }
                      else { (j - A.x) as f32 / (B.x - A.x) as f32 };
            let p = A + (B - A).mul_num(phi);
            let idx = (j + (pts[0].y + i) * image.get_width()) as usize;
            if zbuffer[idx] < p.z {
                zbuffer[idx] = p.z;
                image.set(j, pts[0].y + i, color);
            }
        }
    }
}

fn main() {
    let width = 800;
    let height = 800;
    let depth = 255;
    let args: Vec<String> = std::env::args().collect();
    
    let model = if args.len() == 1 { model::Model::open("obj/african_head.obj") }
                   else if args.len() == 2 { 
                       if args[1].find(".obj") != None { model::Model::open(&args[1]) } 
                       else { panic!("Error: Parameter: {} is not an obj file.", args[1]); }
                   }
                   else { panic!("Too many parameters input."); };
    

    let mut image = TGAImage::with_info(width,height,RGB);
    
    let mut zbuffer = vec![std::i32::MIN; width as usize * height as usize];
    
    let light_dir = Vec3f::new(0, 0, -1);
    for i in 0..model.nfaces() {
        let face = model.face(i);
        let mut screen_coords = [Vec3i::new(0,0,0);3];
        let mut world_coords = [Vec3f::new(0,0,0);3];
        for j in 0..3 {
            let v = model.vert(face[j] as usize);
            screen_coords[j] = Vec3i::new((v.x+1.0)* width as f32 / 2.0, (v.y+1.0) * height as f32 / 2.0, (v.z + 1.0) * depth as f32 /2.0);
            world_coords[j]  = v;
        }
        let mut n = cross((world_coords[2]-world_coords[0]),(world_coords[1]-world_coords[0]));
        n = n.normalize();
        let intensity = n * light_dir;
        if intensity > 0.0 {
            triangle(
                &mut screen_coords, 
                &mut image, 
                TGAColor::with_color(RGBAColor((intensity * 255.0) as u8, (intensity * 255.0) as u8, (intensity * 255.0) as u8, 255)),
                &mut zbuffer
                );
        }
    }
    
    image.flip_vertically().unwrap();
    image.write_tga_file("output.tga", tga_image::WRITE_RLE_FILE).unwrap();
    println!("Finished");
}
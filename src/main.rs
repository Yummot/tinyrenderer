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
fn triangle(v0: &mut [Vec3i], image: &mut TGAImage, color: TGAColor) {   
    if (v0.y == v1.y &&  v0.y == v2.y) && (v0.x == v1.x &&  v0.x == v2.x) { return }
    if v0.y > v1.y { std::mem::swap(&mut v0, &mut v1); }
    if v0.y > v2.y { std::mem::swap(&mut v0, &mut v2); }
    if v1.y > v2.y { std::mem::swap(&mut v1, &mut v2); }   
    
    let total_height = v2.y -v0.y;
    for i in 0..total_height {
        let second_half = i > (v1.y - v0.y) || v1.y == v0.y;
        let segment_height = if second_half { v2.y - v1.y } else { v1.y - v0.y } as f32;
        let alpha = i as f32 / total_height as f32;
        let beta = if second_half { (i - v1.y + v0.y) as f32 / segment_height }
                   else { i as f32 / segment_height };
        
        let mut A = v0 + (v2 - v0).mul_num(alpha);
        let mut B = if second_half { v1 + (v2 - v1).mul_num(beta) } else { v0 + (v1 - v0).mul_num(beta) };
        
        if A.x > B.x { std::mem::swap(&mut A, &mut B); }
        for j in A.x..(B.x + 1) {
            image.set(j, v0.y + i, color);
        }
    }
}

fn main() {
    let model = if args.len() == 1 { model::Model::open("obj/african_head.obj") }
                   else if args.len() == 2 { 
                       if args[1].find(".obj") != None { model::Model::open(&args[1]) } 
                       else { panic!("Error: Parameter: {} is not an obj file.", args[1]); }
                   }
                   else { panic!("Too many parameters input."); };
    
    let mut image = TGAImage::with_info(width,height,RGB);
    
    let light_dir = Vec3f::new(0, 0, -1);
    for i in 0..model.nfaces() {
        let face = model.face(i);
        let mut screen_coords = [Vec2i::new(0,0);3];
        let mut world_coords = [Vec3f::new(0,0,0);3];
        for j in 0..3 {
            let v = model.vert(face[j] as usize);
            screen_coords[j] = Vec2i::new((v.x+1.0)* width as f32 / 2.0, (v.y+1.0) * height as f32 / 2.0);
            world_coords[j]  = v;
        }
        let mut n = cross((world_coords[2]-world_coords[0]),(world_coords[1]-world_coords[0]));
        n = n.normalize();
        let intensity = n * light_dir;
        if intensity > 0.0 {
            triangle(&screen_coords, &mut image, TGAColor::with_color(RGBAColor((intensity * 255.0) as u8, (intensity * 255.0) as u8, (intensity * 255.0) as u8, 255)));
        }
    }
    
    image.flip_vertically().unwrap();
    image.write_tga_file("lesson2_output.tga", tga_image::WRITE_RLE_FILE).unwrap();
    println!("Finished");
}
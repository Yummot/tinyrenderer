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

fn barycentric(points: &[Vec2i], point: Vec2i) -> Vec3f {
    let u = cross(Vec3f::new(points[2][0]-points[0][0], points[1][0]-points[0][0], points[0][0]-point[0]), Vec3f::new(points[2][1]-points[0][1], points[1][1]-points[0][1], points[0][1]-point[1])); 
    if u[2].abs() < 1.0 { return Vec3f::new(-1.0,1.0,1.0); } // triangle is degenerate, in this case return smth with negative coordinates 
    let ret = Vec3f::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
    return ret
}

#[allow(non_snake_case)]
fn triangle(points: &[Vec2i], image: &mut TGAImage, color: TGAColor) {   
    let mut bboxmin = Vec2i::new(image.get_width() - 1, image.get_height() - 1);
    let mut bboxmax = Vec2i::new(0, 0);
    let clamp = Vec2i::new(image.get_width() - 1, image.get_height() - 1);
    
    for i in 0..3 {
        for j in 0..2 {
            bboxmin[j] = std::cmp::max(0, std::cmp::min(bboxmin[j], points[i][j]));
            bboxmax[j] = std::cmp::min(clamp[j], std::cmp::max(bboxmax[j], points[i][j]));
        }
    }
    
    let mut p = Vec2i::new(bboxmin.x, bboxmin.y);
    while p.x <= bboxmax.x {
        p.y = bboxmin.y;
        while p.y <= bboxmax.y {
            let bc_screen = barycentric(points, p);
            if bc_screen.x < 0.0 || bc_screen.y <0.0 || bc_screen.z < 0.0 { 
                p.y += 1;
                continue 
            }
            image.set(p.x, p.y, color);
            p.y += 1;
        }
        p.x += 1;
    }
}

fn main() {
    // let white = TGAColor::with_color(RGBAColor(255,255,255,255));
    // let red = TGAColor::with_color(RGBAColor(255,0,0,255));
    // let blue = TGAColor::with_color(RGBAColor(0,0,255,255));
    let width = 800;
    let height = 800;
    
    let args: Vec<String> = std::env::args().collect();
    
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
    
    // test triangle function
    // let t0 = [Vec2i::new(10, 70),   Vec2i::new(50, 160),  Vec2i::new(70, 80)]; 
    // let t1 = [Vec2i::new(180, 50),  Vec2i::new(150, 1),   Vec2i::new(70, 180)]; 
    // let t2 = [Vec2i::new(180, 150), Vec2i::new(120, 160), Vec2i::new(130, 180)];
    // let t = [Vec2i::new(10,10), Vec2i::new(100, 30), Vec2i::new(190, 160)];  
    
    // triangle(&t0, &mut image, white);
    // triangle(&t1, &mut image, red);
    // triangle(&t2, &mut image, blue);  `
    // triangle(&t, &mut image, TGAColor::with_color(RGBAColor(120,80,64,255)));
    
    image.flip_vertically().unwrap();
    image.write_tga_file("lesson2_output.tga", tga_image::WRITE_RLE_FILE).unwrap();
    println!("Finished");
}
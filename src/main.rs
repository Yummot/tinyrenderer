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
fn triangle(mut v0: Vec2i, mut v1: Vec2i, mut v2: Vec2i, image: &mut TGAImage, _color: TGAColor) {
    let red = TGAColor::with_color(RGBAColor(255,0,0,255));
    let blue = TGAColor::with_color(RGBAColor(0,0,255,255));
    
    if v0.y > v1.y { std::mem::swap(&mut v0, &mut v1); }
    if v0.y > v2.y { std::mem::swap(&mut v0, &mut v2); }
    if v1.y > v2.y { std::mem::swap(&mut v1, &mut v2); }   
    
    let total_height = v2.y -v0.y;
    for y in v0.y..(v1.y + 1) {
        let segment_height = v1.y - v0.y + 1;
        let alpha = (y as f32 - v0.y as f32) / total_height as f32;
        let beta = (y as f32 - v0.y as f32) / segment_height as f32;
        
        let A = v0 + (v2 - v0).mul_num(alpha);
        let B = v0 + (v1 - v0).mul_num(beta);
        
        image.set(A.x, y, red);
        image.set(B.x, y, blue);
    }
}

fn main() {
    let white = TGAColor::with_color(RGBAColor(255,255,255,255));
    let red = TGAColor::with_color(RGBAColor(255,0,0,255));
    let blue = TGAColor::with_color(RGBAColor(0,0,255,255));
    
    let mut image = TGAImage::with_info(200,200,RGB);
    
    let t0 = [Vec2i::new(10, 70),   Vec2i::new(50, 160),  Vec2i::new(70, 80)]; 
    let t1 = [Vec2i::new(180, 50),  Vec2i::new(150, 1),   Vec2i::new(70, 180)]; 
    let t2 = [Vec2i::new(180, 150), Vec2i::new(120, 160), Vec2i::new(130, 180)]; 
    
    triangle(t0[0], t0[1], t0[2], &mut image, white);
    triangle(t1[0], t1[1], t1[2], &mut image, red);
    triangle(t2[0], t2[1], t2[2], &mut image, blue);
    
    image.flip_vertically().unwrap();
    image.write_tga_file("lesson2_output.tga", tga_image::WRITE_RLE_FILE).unwrap();
    println!("Finished");
}
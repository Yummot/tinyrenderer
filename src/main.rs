#[macro_use]
mod geometry;
mod model;
#[cfg(test)]
mod tests;
mod tga_image;
use tga_image::*;

fn line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, image: &mut TGAImage, color: TGAColor) {
     let mut steep = false;
     if (x0 - x1).abs() < (y0 - y1).abs() {
         std::mem::swap(&mut x0, &mut y0);
         std::mem::swap(&mut x1, &mut y1);
         steep = true;
     }
     if x0 > x1 {
         std::mem::swap(&mut x0, &mut x1);
         std::mem::swap(&mut y0, &mut y1);
     }
     for x in x0..x1 {
         let t = (x as f32 - x0 as f32) / (x1 as f32 - x0 as f32);
         let y = y0 as f32 * (1.0 - t) + y1 as f32 * t;
         if steep {
             image.set(y as usize, x as usize, color);
         } else {
             image.set(x as usize, y as usize, color);
         }
     }
}

fn main() {
    let width = 800;
    let height = 800;
    let white = TGAColor::with_color(RGBAColor(255,255,255,255));
    let _red   = TGAColor::with_color(RGBAColor(255,0,0,255));
    let args: Vec<String> = std::env::args().collect();
    let model = if args.len() == 1 {
        model::Model::open("obj/african_head.obj")
    } else if args.len() == 2 {
        if args[1].find(".obj") != None {
            model::Model::open(&args[1])
        } else {
            panic!("Error: Wrong input parameter: {:?}", args[1])
        }
    } else {
        panic!("Too many input parameter.");   
    };
    
    let mut image = tga_image::TGAImage::with_info(width, height, tga_image::RGB);
    
    for i in 0..model.nfaces() {
        let face = model.face(i);
        for j in 0..3 {
            let v0 = model.vert(face[j] as usize);
            let v1 = model.vert(face[(j + 1) % 3] as usize);
            
            let x0 = ((v0.x + 1.0) * (width as f32) / 2.0) as i32;
            let y0 = ((v0.y + 1.0) * (height as f32) / 2.0) as i32;
            let x1 = ((v1.x + 1.0) * (width as f32) / 2.0) as i32;
            let y1 = ((v1.y + 1.0) * (height as f32) / 2.0) as i32;
            
            //println!("x0 {}, y0 {}, x1 {}, y1 {}.({},{})", x0, y0, x1, y1, i, j);
            //println!("v0: {:?}, v1: {:?}", v0, v1);
            line(x0,y0,x1,y1,&mut image, white);
        }
    }
    
    image.flip_vertically().unwrap();
    image.write_tga_file("output.tga", tga_image::WRITE_RLE_FILE).unwrap();
    println!("Finished");
}
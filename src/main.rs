#[macro_use]
mod gl;
pub use gl::*;
// use std::io::prelude::*;
#[cfg(test)]
mod tests;

#[allow(non_snake_case)]
fn main() {
    let width: i32= 800;
    let height: i32= 800;
    let args: Vec<String> = std::env::args().collect();
    let eye = Vec3f::new(1,1,3);
    let center = Vec3f::new(0,0,0);
    let up = Vec3f::new(0,1,0);
    
    let mut model = if args.len() == 1 { model::Model::open("obj/african_head.obj") }
                   else if args.len() == 2 { 
                       if args[1].find(".obj") != None { model::Model::open(&args[1]) } 
                       else { panic!("Error: Parameter: {} is not an obj file.", args[1]); }
                   }
                   else { panic!("Too many parameters input."); };
    
    let mut image = gl::TGAImage::with_info(width as isize, height as isize, tga_image::RGB);
    let mut zbuffer = gl::TGAImage::with_info(width as isize, height as isize, tga_image::GRAYSCALE);
    
    let mut CameraOne = Camera::new(255.0);
    CameraOne.set_light_dir(Vec3f::new(1,1,1));
    CameraOne.lookat(eye, center, up);
    CameraOne.viewport(width / 8, height / 8, width * 3 / 4, height * 3 / 4);
    CameraOne.projection( -1.0 / (eye - center).norm() as f32);
    CameraOne.light_dir = CameraOne.light_dir.normalize();
    
    let mut shader = gl::GourauShader::new();
    
    for i in 0..model.nfaces() {
        let mut screen_coords = [Vec4f::zero();3];
        for j in 0..3 {
            screen_coords[j] = shader.vertex(&CameraOne, &mut model, i as i32, j as i32); 
        }
        gl::triangle(&mut screen_coords, &mut shader, &mut image, &mut zbuffer);
    }
    
    image.flip_vertically().unwrap();
    image.write_tga_file("output.tga", tga_image::WRITE_RLE_FILE).unwrap();
    zbuffer.flip_vertically().unwrap();
    zbuffer.write_tga_file("zbuffer.tga", tga_image::WRITE_RLE_FILE).unwrap();
    
    println!("Finished");
}




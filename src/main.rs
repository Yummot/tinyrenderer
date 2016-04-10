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
    let depth = 2000.0;
    let args: Vec<String> = std::env::args().collect();
    let eye = Vec3f::new(1,1,4);
    let center = Vec3f::new(0,0,0);
    let up = Vec3f::new(0,1,0);
    let light_dir = Vec3f::new(1,1,0).normalize();
    
    let mut model = if args.len() == 1 { model::Model::open_with_texture("obj/african_head.obj") }
                   else if args.len() == 2 { 
                       if args[1].find(".obj") != None { model::Model::open_with_texture(&args[1]) } 
                       else { panic!("Error: Parameter: {} is not an obj file.", args[1]); }
                   }
                   else { panic!("Too many parameters input."); };
    
    let mut shadowbuffer = vec![std::f32::MIN; (width * height) as usize];
    let mut zbuffer = vec![std::f32::MIN; (width * height) as usize];
    let mut CameraOne = Camera::new(depth);
    CameraOne.set_light_dir(light_dir);
    
    {
        let mut depth_image = gl::TGAImage::with_info(width as isize, height as isize, tga_image::RGB);
        CameraOne.lookat(light_dir, center, up);
        CameraOne.viewport(width / 8, height / 8, width * 3 / 4, height * 3 / 4);
        CameraOne.projection(0.0);
        
        
        let mut shader = gl::shader::DepthShader::new(depth);
        // let mut shader = gl::shader::GourauShader::new();
        for i in 0..model.nfaces() {
            let mut screen_coords = [Vec4f::zero();3];
            for j in 0..3 {
                screen_coords[j] = shader.vertex(&CameraOne, &mut model, i as i32, j as i32); 
            }
            gl::triangle(&mut screen_coords, &mut shader, &mut depth_image, &mut shadowbuffer);
        }
        
        depth_image.flip_vertically().unwrap();
        depth_image.write_tga_file("depth.tga", gl::WRITE_RLE_FILE).unwrap();
    }
    
    let m = CameraOne.viewport * CameraOne.projection * CameraOne.modelview;
    {
        let mut image = gl::TGAImage::with_info(width as isize, height as isize, tga_image::RGB);   
        CameraOne.lookat(eye, center, up);
        CameraOne.viewport(width / 8, height / 8, width * 3 / 4, height * 3 / 4);
        CameraOne.projection(-1.0 / (eye - center).norm() as f32);
        
        let mut shader = gl::shader::IShaderBuilder::new(CameraOne.modelview, (CameraOne.projection * CameraOne.modelview).inverse().transpose(), m * (CameraOne.viewport * CameraOne.projection * CameraOne.modelview).inverse())
            .light_dir(light_dir).size(width as usize, height as usize).model(model.clone()).shadowbuffer(&shadowbuffer).build();

        let mut screen_coords = [Vec4f::zero();3];
        for i in 0..model.nfaces() {
            for j in 0..3 {
                screen_coords[j] = shader.vertex(&CameraOne, &mut model, i as i32, j as i32);
            }
            gl::triangle(&mut screen_coords, &shader, &mut image, &mut zbuffer);
        }
        
        image.flip_vertically().unwrap();
        image.write_tga_file("output.tga", gl::WRITE_RLE_FILE).unwrap();
    }
    
    println!("Finished");
}




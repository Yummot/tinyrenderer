#[macro_use]
mod gl;
pub use gl::*;
#[cfg(test)]
mod tests;

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
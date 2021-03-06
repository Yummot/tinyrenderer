// use super::std;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use gl::geometry::*;
use gl::tga_image::*;
use gl::color::*;

#[derive(Debug,Clone)]
pub struct Model {
    verts_: Vec<Vec3f>,
    faces_: Vec<Vec<Vec3i>>, // this Vec3i means vertex/uv/normal
    norms_: Vec<Vec3f>,
    uv_: Vec<Vec2f>,
    diffusemap_: TGAImage,
    normalmap_: TGAImage,
    specularmap_: TGAImage,
}

fn solver(x: &&str, faces: &mut Vec<Vec<Vec3i>>, verts: &mut Vec<Vec3f>, norms: &mut Vec<Vec3f>, uv: &mut Vec<Vec2f>) {
    if x.find("v ") != None {
        let vert: Vec<&str> = x.split_whitespace().collect();

        let x = vert[1].trim().parse::<f32>().unwrap();
        let y = vert[2].trim().parse::<f32>().unwrap();
        let z = vert[3].trim().parse::<f32>().unwrap();

        verts.push(Vec3f::new(x, y, z));
    } else if x.find("f ") != None {
        let face_info: Vec<&str> = x.split_whitespace().collect();
        let mut face_vec = vec![];
        
        for x in face_info {
            if x.find("f") != None { continue }
            let single: Vec<i32> = x.split("/").map(|x| x.trim().parse::<i32>().unwrap() - 1).collect();
            let tmp = Vec3i::from_vec(&single[0..]); //TODO: need to refator
            face_vec.push(tmp);
        }
        faces.push(face_vec);
    } else if x.find("vn ") != None {
        let vert: Vec<&str> = x.split_whitespace().collect();

        let x = vert[1].trim().parse::<f32>().unwrap();
        let y = vert[2].trim().parse::<f32>().unwrap();
        let z = vert[3].trim().parse::<f32>().unwrap();
        
        norms.push(Vec3f::new(x,y,z));
    } else if x.find("vt") != None {
        let vert: Vec<&str> = x.split_whitespace().collect();

        let x = vert[1].trim().parse::<f32>().unwrap();
        let y = vert[2].trim().parse::<f32>().unwrap();
        
        uv.push(Vec2f::new(x,y));
    }
}

impl Model {
    #[allow(dead_code)]
    pub fn open(filename: &str) -> Model {
        let path = Path::new(filename);
        let mut file = File::open(&path).unwrap();

        let mut data = String::new();
        let _len = file.read_to_string(&mut data).unwrap();

        let vec_data: Vec<&str> = data.split('\n').collect();
        let data_without_comments: Vec<&str>  = vec_data.into_iter().filter(|&x| {
            match x.find("#") {
                None => return true,
                _ => return false,
            }
        }).collect();

        let mut faces_vec: Vec<Vec<Vec3i>> = vec![];
        let mut verts_vec: Vec<Vec3f> = vec![];
        let mut norm_vec: Vec<Vec3f> = vec![];
        let mut uv_vec: Vec<Vec2f> = vec![];

        let _data: Vec<()> = data_without_comments.iter()
            .map(|x| solver(x, &mut faces_vec, &mut verts_vec, &mut norm_vec, &mut uv_vec))
            .collect();

        Model {
            verts_: verts_vec,
            faces_: faces_vec,
            norms_: norm_vec,
            uv_: uv_vec,
            diffusemap_: TGAImage::new(),
            normalmap_: TGAImage::new(),
            specularmap_: TGAImage::new(),
        }
    }
    #[allow(dead_code)]
    pub fn open_with_texture(filename: &str) -> Model {
        let mut ret = Model::open(filename);
        ret.load_texture(filename, "_diffuse.tga");
        ret.load_texture(filename, "_nm.tga");
        ret.load_texture(filename, "_spec.tga");
        ret
    }
    #[allow(dead_code)]
    fn load_texture(&mut self, filename: &str, suffix: &str) {
        let prefix = filename.split('.').next().unwrap();
        let texname = prefix.to_string() + suffix;
        println!("{}", texname);
        let mut tmp = match suffix {
            "_diffuse.tga" => &mut self.diffusemap_,
            "_nm.tga" => &mut self.normalmap_,
            "_spec.tga" => &mut self.specularmap_,
            _ => return,   
        };
        tmp.read_tga_file(&texname);
        tmp.flip_vertically().unwrap();
    }
    #[allow(dead_code)]
    pub fn nverts(&self) -> usize {
        self.verts_.len()
    }
    #[allow(dead_code)]
    pub fn nfaces(&self) -> usize {
        self.faces_.len()
    }
    #[allow(dead_code)]
    pub fn vert(&self, idx: usize) -> Vec3f {
        self.verts_[idx]
    }
    #[allow(dead_code)]
    pub fn face_vert(&self, iface: i32, nthvert: i32) -> Vec3f {
         self.verts_[self.faces_[iface as usize][nthvert as usize][0] as usize]    
    }
    #[allow(dead_code)]
    pub fn face(&self, idx: usize) -> Vec<i32> {
        let mut ret = vec![];
        for i in 0..self.faces_[idx].len() { ret.push(self.faces_[idx][i][0]); }
        ret
    }
    #[allow(dead_code)]
    pub fn diffuse(&self, uvf: Vec2f) -> Color {
        let uv = Vec2i::new(uvf[0] * self.diffusemap_.get_width() as f32, uvf[1] * self.diffusemap_.get_height() as f32);
        self.diffusemap_.get(uv.x, uv.y)
    }
    #[allow(dead_code)]
    pub fn uv(&self, iface: usize, nthvert: usize) -> Vec2f {
        self.uv_[self.faces_[iface][nthvert][1] as usize]
    }
    #[allow(dead_code)]
    pub fn normal(&self, uvf: Vec2f) -> Vec3f {
        let uv = Vec2i::new(uvf[0] * self.normalmap_.get_width() as f32, uvf[1] * self.normalmap_.get_height() as f32);
        let color = self.normalmap_.get(uv[0], uv[1]);
        let mut res = Vec3f::zero();
        for i in 0..3 {
            res[2 - i] = color[i] as f32 / 255.0 * 2.0 - 1.0;
        }
        res
    }
    pub fn specular(&self, uvf: Vec2f) -> f32 {
        let uv = Vec2i::new(uvf[0] * self.specularmap_.get_width() as f32, uvf[1] * self.specularmap_.get_height() as f32);
        self.specularmap_.get(uv.x, uv.y)[0] as f32 / 1.0
    }

    pub fn face_normal(&mut self, iface: i32, nthvert: i32) -> Vec3f {
        let idx = self.faces_[iface as usize][nthvert as usize][2] as usize;
        self.norms_[idx] = self.norms_[idx].normalize();
        return self.norms_[idx];
    }
}

// pub fn nverts(&self) -> i32;
// pub fn nfaces(&self) -> i32;
// pub fn vert(idx: usize) -> Vec3f;
// pub fn face(idx: usize) -> Vec<i32>;
 

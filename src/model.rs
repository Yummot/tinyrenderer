// use super::std;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use geometry::*;
use tga_image::*;

#[derive(Debug,Clone)]
pub struct Model {
    verts_: Vec<Vec3f>,
    faces_: Vec<Vec<Vec3i>>,
    norms_: Vec<Vec3f>,
    uv_: Vec<Vec2f>,
    diffusemap_: TGAImage,
}

fn solver(x: &&str, faces: &mut Vec<Vec<Vec3i>>, verts: &mut Vec<Vec3f>) {
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
            let single = x.split("/").map(|x| x.trim().parse::<i32>().unwrap() - 1).collect();
            let tmp = Vec3i::from_vec(&single);
            face_vec.push(tmp);
        }
        faces.push(face_vec);
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

        let _data: Vec<()> = data_without_comments.iter().map(|x| solver(x, &mut faces_vec, &mut verts_vec)).collect();

        Model {
            verts_: verts_vec,
            faces_: faces_vec,
            norms_: vec![],
            uv_: vec![],
            diffusemap_: TGAImage::new(),
        }
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
    pub fn face(&self, idx: usize) -> Vec<Vec3i> {
        self.faces_[idx].clone()
    }
}

// pub fn nverts(&self) -> i32;
// pub fn nfaces(&self) -> i32;
// pub fn vert(idx: usize) -> Vec3f;
// pub fn face(idx: usize) -> Vec<i32>;
 

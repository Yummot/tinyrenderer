#[cfg(test)]
mod test_geometry{
    use super::super::gl::*;
    vec_create!(
            type Vec<5, f32> = Vec5f;
    );
    
    #[test]
    fn test_new() {
        let v2f = Vec2f::new(1.0,1.0);
        let v3f = Vec3f::new(1.0,1.0,1.0);
        let v5f = Vec5f::zero();
        
        assert_eq!(format!("{:?}", v2f) ,"Vec2 { x: 1, y: 1 }");
        assert_eq!(format!("{:?}", v3f) ,"Vec3 { x: 1, y: 1, z: 1 }");
        assert_eq!(format!("{:?}", v5f) ,"Vec5f { data: [0, 0, 0, 0, 0] }");
    }
    #[test]
    #[should_panic]
    fn test_out_bounds() {
        let v5f = Vec5f::zero();
        v5f[5];
    }
    #[test]
    fn test_index() {
        let v5f = Vec5f::zero();
        for i in 0..5 {
            assert_eq!(0.0, v5f[i]);
        }
    }
    #[test]
    fn test_index_mut() {
        let mut v5f = Vec5f::zero();
        for i in 0..5 {
            v5f[i] = i as f32;
            assert_eq!(i as f32, v5f[i]);
        }
    }
    #[test]
    fn test_cast() {
        let src = Vec3i::new(1,1,1);
        let cast = src.cast::<Vec3f>();
        assert_eq!("Vec3 { x: 1, y: 1, z: 1 }", format!("{:?}",cast));
    }
}

#[cfg(test)]
mod test_model {
    use super::super::gl::*;
    #[test]
    //unuse
    fn test_new(){
        let _model_face = Model::open("obj/african_head.obj");
        //println!("{:?}", model_face);
        //assert_eq!(false, true);
    }
    #[test]
    fn test_impl() {
        let model_face = Model::open("obj/african_head.obj");
        assert_eq!(model_face.nverts(), 1258);
        assert_eq!(model_face.nfaces(), 2492);
        assert_eq!(model_face.face(0), [23,24,25]);
        assert_eq!(model_face.vert(0), Vec3f::new(-0.000581696, -0.734665, -0.623267));
    }
    #[test]
    fn try_split_whitespace() {
        let patern = "f 24/1/24 25/2/25 26/3/26";
        let res: Vec<&str> = patern.split_whitespace().collect();
        assert_eq!(res,["f", "24/1/24", "25/2/25", "26/3/26"]);
    }
}

// #[cfg(test)]
// mod test_mat {
//     use super::super::gl::*;
//     #[test]
//     fn test_mul() {
//         let mut mat_1 = Mat::new(3, 4);
//         let mut mat_2 = Mat::new(4, 3);
//         mat_1[0][0] = 1.0;
//         mat_2[0][0] = 1.0;
//         let res = mat_1.mul(&mat_2);
//         assert!(res[0][0] == 1.0);
//     }
//     #[test]
//     fn test_builder() {
//         let mat = Mat::builder(&[&[1,2,3],&[4,5,6],&[7,8,9]]).unwrap();
//         assert_eq!(format!("{:?}", mat), "Mat { data: [[1, 2, 3], [4, 5, 6], [7, 8, 9]], rows: 3, cols: 3 }");
//     }
//     #[test]
//     fn test_transpose() {
//         let mut mat_1 = Mat::new(3, 4);
//         mat_1[1][0] = 1.0;
//         let res = mat_1.transpose();
//         assert!(res[0][1] == 1.0);
//     }
//     #[test]
//     fn test_display() {
//         let mat = Mat::builder(&[&[1,2,3],&[4,5,6],&[7,8,9]]).unwrap();
//         assert_eq!(format!("{}", mat), "[[1, 2, 3]\n [4, 5, 6]\n [7, 8, 9]]\n");    
//     }
//     #[test]
//     fn test_inverse() {
//         let mat = Mat::builder(&[&[1,0,1],&[0,1,0],&[1,0,0]]).unwrap();
//         let inv_mat = mat.inverse();
//         assert_eq!(format!("{}",inv_mat), "[[0, 0, 1]\n [0, 1, 0]\n [1, 0, -1]]\n");
//     }
//     #[test]
//     fn test_mat4_mut() {
//         let mat = Mat4::new([1f32,  2f32,  1f32,  0f32,
//                          3f32,  1f32,  4f32,  2f32,
//                          1f32,  2f32, -5f32,  4f32,
//                          3f32,  2f32,  4f32,  1f32]);
//         let mut v4f = Vec4f::zero();
//         v4f[0] = 1.0;
//         let res = v4f * mat;
//         let _res2 = mat * v4f;
//         assert_eq!(res, Vec4f::new([1.0,3.0,1.0,3.0]));
//     }    
// }



#[cfg(test)]
mod test_mat4 {
    use super::super::gl::*;
    #[test]
    fn test_sub() {
        let mut a = Mat4::identity();
        *a.at_mut(0, 1) = 1f32;
        let mut b = Mat4::identity();
        *b.at_mut(2, 3) = 3f32;
        let c = Mat4::new([0f32, 1f32, 0f32, 0f32,
                            0f32, 0f32, 0f32, 0f32,
                            0f32, 0f32, 0f32, -3f32,
                            0f32, 0f32, 0f32, 0f32]);
        assert!(a - b == c);
    }
    #[test]
    fn test_mul() {
        assert!(Mat4::identity() * Mat4::identity() == Mat4::identity());
        let a = Mat4::new([1f32,  2f32,  1f32,  0f32,
                            3f32,  1f32,  4f32,  2f32,
                            1f32,  2f32, -5f32,  4f32,
                            3f32,  2f32,  4f32,  1f32]);
        let b = Mat4::new([8f32,  0f32,  2f32,  3f32,
                            -2f32,  1f32,  0f32,  1f32,
                            5f32, -2f32,  3f32,  1f32,
                            0f32,  0f32,  4f32,  1f32]);
        let c = Mat4::new([9f32,   0f32,   5f32,   6f32,
                            42f32,  -7f32,  26f32,  16f32,
                            -21f32,  12f32,   3f32,   4f32,
                            40f32,  -6f32,  22f32,  16f32]);
        assert!(a * b == c);
    }  
    #[test]
    fn test_mat4_mut() {
        let mat = Mat4::new([1f32,  2f32,  1f32,  0f32,
                         3f32,  1f32,  4f32,  2f32,
                         1f32,  2f32, -5f32,  4f32,
                         3f32,  2f32,  4f32,  1f32]);
        let mut v4f = Vec4f::zero();
        v4f[0] = 1.0;
        let res = mat * v4f;
        assert_eq!(res, Vec4f::new([1.0,3.0,1.0,3.0]));
    }  
}
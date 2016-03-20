#[cfg(test)]
mod test_geometry{
    use super::super::geometry::*;
    vec_create!(
            type Vec<5, f32> = Vec5f;
    );
    
    #[test]
    fn test_new() {
        let v2f = Vec2f::new(1.0,1.0);
        let v3f = Vec3f::new(1.0,1.0,1.0);
        let v5f = Vec5f::new();
        
        assert_eq!(format!("{:?}", v2f) ,"Vec2 { x: 1, y: 1 }");
        assert_eq!(format!("{:?}", v3f) ,"Vec3 { x: 1, y: 1, z: 1 }");
        assert_eq!(format!("{:?}", v5f) ,"Vec5f { data: [0, 0, 0, 0, 0] }");
    }
    #[test]
    #[should_panic]
    fn test_out_bounds() {
        let v5f = Vec5f::new();
        v5f[5];
    }
    #[test]
    fn test_index() {
        let v5f = Vec5f::new();
        for i in 0..5 {
            assert_eq!(0.0, v5f[i]);
        }
    }
    #[test]
    fn test_index_mut() {
        let mut v5f = Vec5f::new();
        for i in 0..5 {
            v5f[i] = i as f32;
            assert_eq!(i as f32, v5f[i]);
        }
    }
}

#[cfg(test)]
mod test_model {
    use super::super::model::*;
    use super::super::geometry::*;
    #[test]
    //unuse
    fn test_new(){
        let _model_face = Model::new("assets/african_head.obj");
        //println!("{:?}", model_face);
        //assert_eq!(false, true);
    }
    #[test]
    fn test_impl() {
        let model_face = Model::new("assets/african_head.obj");
        assert_eq!(model_face.nverts(), 1258);
        assert_eq!(model_face.nfaces(), 2492);
        assert_eq!(model_face.face(0), [23,24,25]);
        assert_eq!(model_face.vert(0), Vec3f::new(-0.000581696, -0.734665, -0.623267));
    }
}
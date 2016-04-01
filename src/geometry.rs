pub use super::std;
pub use super::std::ops::*;
pub use super::std::cmp::PartialEq;
extern crate num;

pub use self::num::*;

pub trait Norm {
    type Output;
    type Normalize;
    fn norm(&self) -> Self::Output;
    fn normalize(&self) -> Self::Normalize;
}

macro_rules! sum {
    ($t: expr, $($rest: tt)*) => (
        $t + sum!($($rest)*)
    );
    ($t: expr) => ($t);
}

macro_rules! vec_eq {
    ($t: expr, $($rest: tt)*) => (
        $t && vec_eq!($($rest)*)
    );
    ($t: expr) => ($t);
}

macro_rules! vec_create {
    ($(type Vec<$n : expr , $t : ty> = $dst : ident;)*) => (
        $(
            #[derive(Debug, Copy, Clone)]
            pub struct $dst {
                data: [$t; $n],
            }
            
            impl $dst {
                #[allow(dead_code)]
                pub fn new() -> $dst {
                    $dst {
                        data: [0 as $t; $n],
                    }
                }
                // pub fn mul_num(&mut self, rhs: f64) -> $dst {
                //     for mut iter in self.data.iter_mut() {
                //         *iter = (*iter as f64 * rhs) as $t;
                //     }
                //     *self
                // }
            }
            
            impl ::std::ops::Index<usize> for $dst {
                type Output = $t;
                fn index<'a>(&'a self, index: usize) -> &'a $t {
                    &self.data[index]
                }
            } 
            
            impl ::std::ops::IndexMut<usize> for $dst {
                fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut $t {
                    &mut self.data[index]
                }
            }
            
            impl Norm for $dst {
                type Output = f64;
                type Normalize = Self;
                #[allow(dead_code)]
                fn norm(&self) -> f64 {
                    (self.data.iter().map(|x| x * x).fold(0.0, |sum, x| sum + x) as f64).sqrt()
                }
                fn normalize(&self) -> $dst {
                    let mut ret = *self;
                    ret = ret * (1.0 / self.norm());
                    ret
                }
            }
            
            impl Mul for $dst {
                type Output = f64;
                #[allow(dead_code)]
                fn mul(self, rhs: Self) -> f64 {
                    let mut ret = 0.0;
                    for i in 0..$n {
                        ret += self.data[i] * rhs.data[i];
                    }
                    
                    ret as f64
                }
            }
            
            impl Mul<f32> for $dst {
                type Output = Self;
                fn mul(self, rhs: f32) -> $dst {
                    let mut ret = $dst::new();
                    for i in 0..$n {
                        ret[i] = cast::<f32,$t>(cast::<$t,f32>(self[i]).unwrap() * rhs).unwrap();
                    }
                    ret
                }
            }
            
            impl Mul<f64> for $dst {
                type Output = Self;
                fn mul(self, rhs: f64) -> $dst {
                    let mut ret = $dst::new();
                    for i in 0..$n {
                        ret[i] = cast::<f64,$t>(cast::<$t,f64>(self[i]).unwrap() * rhs).unwrap();
                    }
                    ret
                }
            }
            
            impl Sub for $dst {
                type Output = $dst;
                #[allow(dead_code)]
                fn sub(self, rhs: Self) -> $dst {
                    let mut ret = [0 as $t; $n];
                    for i in 0..$n {
                        ret[i] = self.data[i] - rhs.data[i];
                    }
                    $dst {
                        data: ret,
                    }
                }
            }
            
            impl Add for $dst {
                type Output = $dst;
                #[allow(dead_code)]
                fn add(self, rhs: Self) -> $dst {
                    let mut ret = [0 as $t; $n];
                    for i in 0..$n {
                        ret[i] = self.data[i] + rhs.data[i];
                    }
                    $dst {
                        data: ret,
                    }
                }
            }
            
        )*
    );
}

vec_create!(
    type Vec<4, f32> = Vec4f;
);

#[derive(Debug, Copy, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> ::std::ops::Index<usize> for Vec2<T> {
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a T {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Error: Index out of bounds.")
        }
    }
} 

impl<T> ::std::ops::IndexMut<usize> for Vec2<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Error: Index out of bounds.")
        }
    }
}

impl<T> ::std::ops::Index<usize> for Vec3<T> {
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Error: Index out of bounds.")
        }
    }
} 

impl<T> ::std::ops::IndexMut<usize> for Vec3<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Error: Index out of bounds.")
        }
    }
}

macro_rules! vec_impl_helper {
    ($($dst : ident > ( $($attr_name : ident)*) ;)*) => (
        $(
            impl<T> Mul<$dst<T>> for $dst<T>
                where T: Clone + Mul<Output = T> + Add<Output = T>
            {
                type Output = T;
                #[allow(dead_code)]
                fn mul(self, rhs: Self) -> T {
                    sum!($(self.$attr_name * rhs.$attr_name),*) as T
                }
            }

            // impl<T> Mul<T> for $dst<T> 
            //    where T: NumCast + Num + Copy + Mul<Output = T> {
            //     type Output = Self;
            //     fn mul(self, rhs: T) -> $dst<T> {
            //         $dst {
            //             $(
            //                 $attr_name: self.$attr_name * rhs,
            //             )*
            //         }
            //     }
            // }
            
            impl<T> Mul<f32> for $dst<T>
               where T: NumCast + Num + Copy + Mul<Output = T> {
                type Output = $dst<T>;
                fn mul(self, rhs: f32) -> $dst<T> {
                    $dst {
                        $(
                            $attr_name: cast::<f32,T>(cast::<T,f32>(self.$attr_name).unwrap() * rhs).unwrap(),
                        )*
                    }
                }
            }
            
            impl<T> Mul<f64> for $dst<T>
               where T: NumCast + Num + Copy + Mul<Output = T> {
                type Output = $dst<T>;
                fn mul(self, rhs: f64) -> $dst<T> {
                    $dst {
                        $(
                            $attr_name: cast::<f64,T>(cast::<T,f64>(self.$attr_name).unwrap() * rhs).unwrap(),
                        )*
                    }
                }
            }
            
            impl<T> Mul<i32> for $dst<T>
               where T: NumCast + Num + Copy + Mul<Output = T> {
                type Output = $dst<T>;
                fn mul(self, rhs: i32) -> $dst<T> {
                    $dst {
                        $(
                            $attr_name: cast::<i32,T>(cast::<T,i32>(self.$attr_name).unwrap() * rhs).unwrap(),
                        )*
                    }
                }
            }
            
            impl<T> Sub for $dst<T>
                where T: Clone + Sub<Output = T>
            {
                type Output = $dst<T>;
                #[allow(dead_code)]
                fn sub(self, rhs: Self) -> $dst<T> {
                    $dst {
                        $(
                            $attr_name: self.$attr_name - rhs.$attr_name,
                        )*
                    }
                }
            }

            impl<T> Add for $dst<T>
                where T: Clone + Add<Output = T>
            {
                type Output = $dst<T>;
                #[allow(dead_code)]
                fn add(self, rhs: Self) -> $dst<T> {
                    $dst {
                        $(
                            $attr_name: self.$attr_name + rhs.$attr_name,
                        )*
                    }
                }
            }

            impl<T> PartialEq for $dst<T>
                where T: PartialEq
            {
                fn eq(&self, rhs: &Self) -> bool {
// vec_eq!(self.x == rhs.x, self.y == rhs.y)
                    vec_eq!($(self.$attr_name == rhs.$attr_name),*)
                }
                fn ne(&self, rhs: &Self) -> bool {
                    !self.eq(rhs)
                }
            }

            impl<T> $dst<T>
                where T: Mul<Output = T> + Clone + Copy + Zero + NumCast
            {
                // #[allow(dead_code)]
                // pub fn mul_num<N>(&self, rhs: N) -> $dst<T>
                //     where N: NumCast + Zero + Copy {
                //     $dst {
                //         $(
                //             $attr_name: num::cast::<f64,T>(num::cast::<T,f64>(self.$attr_name).unwrap() * num::cast::<N,f64>(rhs).unwrap()).unwrap(),
                //         )*
                //     }
                // }
                #[allow(dead_code)]
                pub fn new<N>($($attr_name : N),*) -> $dst<T>
                    where N: Num + NumCast + Copy, T: Num + NumCast + Copy {
                    $dst {
                        $($attr_name: num::cast::<N,T>($attr_name).unwrap(),)*
                    }
                }
                #[allow(dead_code)]
                fn empty() -> $dst<T>
                    where T: Num + NumCast + Copy {
                    $dst {
                        $($attr_name: num::cast::<u8,T>(0).unwrap(),)*
                    }
                }
                #[allow(dead_code)]
                pub fn from_vec<N>(v: & Vec<N>) -> $dst<T> 
                    where N: Num + NumCast + Copy, T: Num + NumCast + Copy {
                    let mut ret = $dst::empty();
                    for i in 0..v.len() {
                        ret[i] = num::cast::<N,T>(v[i]).unwrap();
                    }
                    ret
                }
                #[allow(dead_code)]
                pub fn to_other<N>(v: &$dst<T>) -> $dst<N>
                   where N: Num + NumCast + Copy, T: Num + NumCast + Copy {
                    $dst {
                        $($attr_name: num::cast::<T,N>(v.$attr_name).unwrap(),)*
                    }       
                }
                
                #[allow(dead_code)]
                pub fn check_add<N>(&self, rhs: &$dst<N>) -> Self
                   where N: Num + NumCast + Copy, T: Num + NumCast + Copy {
                    $dst {
                        $(
                            $attr_name: num::cast::<f64, T>(
                                num::cast::<T,f64>(self.$attr_name).unwrap() + num::cast::<N, f64>(rhs.$attr_name).unwrap()).unwrap(),
                        )*
                    }       
                }
            }

        )*
    );
}


macro_rules! norm_helper {
    ($($dst: ident > ( $($attr_name : ident)*);)*) => (
        $(
            impl<T> Norm for $dst<T>
                where T: Clone + Copy + Mul<Output = T> + Add<Output = T> + NumCast
            {
                type Output = f64;
                type Normalize = Self;
                #[allow(dead_code)]
                fn norm(&self) -> f64 {
                    (num::cast::<T,f64>(sum!($(self.$attr_name * self.$attr_name),*)).unwrap()).sqrt()
                }
                #[allow(dead_code)]
                fn normalize(&self) -> $dst<T> {
                    let mut ret = *self;
                    ret = ret * (1.0 / self.norm());
                    ret
                }
            }
        )*
    );
}

#[allow(dead_code)]
pub fn cross<T>(v1: Vec3<T>, v2: Vec3<T>) -> Vec3<T> 
    where T: Num + NumCast + Copy + Mul<Output=T> + Sub<Output=T> {
    return Vec3::new(v1.y*v2.z - v1.z*v2.y, v1.z*v2.x - v1.x*v2.z, v1.x*v2.y - v1.y*v2.x);
}

vec_impl_helper!(
    Vec2 > (x y);
    Vec3 > (x y z);
);

norm_helper!(
    Vec2 > (x y);
    Vec3 > (x y z);
);



pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;
pub type Vec3f = Vec3<f32>;
pub type Vec3i = Vec3<i32>;

impl Vec3i {
    // const Vec3<float> &v) : 
    // x(int(v.x+.5)), 
    // y(int(v.y+.5)), 
    // z(int(v.z+.5)
    #[allow(dead_code)]
    pub fn to_vec3f(src: &Self) -> Vec3f {
        let mut ret = Vec3f::empty();
        ret.x = src.x as f32 + 0.5;
        ret.y = src.y as f32 + 0.5;
        ret.z = src.z as f32 + 0.5;
        ret
    }
}

#[derive(Debug, Clone)]
pub struct Mat {
    data: Vec<Vec<f32>>,
    rows: u32, 
    cols: u32,
}


const DEFAULT_ALLOC: u32 = 4;

impl Mat {
    #[allow(dead_code)]
    pub fn new(r: u32, c: u32) -> Mat {
        Mat {
            data: vec![vec![0.0;c as usize];r as usize],
            rows: r,
            cols: c,
        }
    }
    #[allow(dead_code)]
    pub fn builder<T>(data: &[&[T]]) -> Result<Mat,&'static str>
       where T: Num + NumCast + Copy {
        let checker = data[0].len();
        let c = data[0].len();
        let mut ret = Mat::new(data.len() as u32, c as u32);
        for i in 0..data.len() {
            if checker != data[i].len() { return Err("Mat::builder: Wrong input data.") } 
            for j in 0..c {
                ret[i][j] = cast::<T,f32>(data[i][j]).unwrap();
            }
        }
        
        return Ok(ret)
    }
}

impl std::default::Default for Mat {
    fn default() -> Self {
        Mat {
            data: vec![vec![0.0;DEFAULT_ALLOC as usize];DEFAULT_ALLOC as usize],
            rows: DEFAULT_ALLOC,
            cols: DEFAULT_ALLOC,
        }
    }
}

impl std::ops::Index<usize> for Mat {
    type Output = Vec<f32>;
    fn index<'a>(&'a self, idx: usize) -> &'a Vec<f32> {
        &self.data[idx]
    }
}

impl std::ops::IndexMut<usize> for Mat {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Vec<f32> {
        &mut self.data[idx]
    }
}

impl<'a> std::ops::Mul<&'a Mat> for &'a Mat {
    type Output = Mat;
    fn mul(self, rhs: &'a Mat) -> Mat {
        if self.cols != rhs.rows { panic!("Mat::mul: Lhs.cols != Rhs.rows."); }
        let mut res = Mat::new(self.rows, rhs.cols);
        //TODO loop unrolling
        for i in 0..self.rows as usize {
            for j in 0..rhs.cols as  usize {
                for k in 0..self.cols as usize {
                    res[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        
        res
    }
}


// impl std::ops::Deref for Mat {
//     type Target = Mat;
//     fn deref<'a>(&'a self) -> &'a Self {
//         self
//     }
// }

impl Mat {
    #[allow(dead_code)]
    pub fn nrows(&self) -> u32 { self.rows }    
    #[allow(dead_code)]
    pub fn ncols(&self) -> u32 { self.cols }
    #[allow(dead_code)]
    pub fn identity(dimesions: u32) -> Mat {
        let mut ret = Mat::new(dimesions, dimesions);
        for i in 0..dimesions as usize {
            ret[i][i] = 1.0;
        }
        ret
    }
    #[allow(dead_code)]
    pub fn transpose(&self) -> Mat {
        let mut ret = Mat::new(self.cols, self.rows);
        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                ret[j][i] = self[i][j];
            }
        }
        ret 
    }
    #[allow(dead_code)]
    pub fn inverse(&self) -> Mat {
        if self.rows != self.cols { panic!("Mat::inverse not a square Matrix"); }
        // augmenting the square matrix with the identity matrix of the same dimensions A => [AI]
        let mut res = Mat::new(self.rows, self.cols * 2);
        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                res[i][j] = self[i][j];
            }
        }
        for i in 0..self.rows as usize {
            res[i][i + self.cols as usize] = 1.0;
        }
        // first pass
        for i in 0..(self.rows - 1) as usize {
            // normalize the first row
            for j in (0..res.cols as usize).rev() {
                res[i][j] /= res[i][i];
            }
            for k in i+1..self.rows as usize {
                let coeff = res[k][i];
                for j in 0..res.cols as usize {
                    res[k][j] -= res[i][j] * coeff;
                }
            }
        }
        // normalize the last row
        for j in (0..res.cols as usize).rev() {
            res[self.rows as usize - 1][j] /= res[self.rows as usize - 1][self.rows as usize - 1];
        }
        // second pass
        for i in (0..self.rows as usize).rev() {
            for k in (0..i).rev() {
                let coeff = res[k][i];
                for j in 0..res.cols as usize {
                    res[k][j] -= res[i][j] * coeff;    
                }
            }
        }
        // cut the identity matrix back
        let mut truncate = Mat::new(self.rows, self.cols);
        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                truncate[i][j] = res[i][j + self.cols as usize];
            }
        }

        return truncate;
    }
}


impl std::fmt::Display for Mat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        try!(write!(f, "["));
        for i in 0..self.rows as usize {
            if i == 0 { try!(write!(f,"[")); }
            else { try!(write!(f,"\n [")); }
            for j in 0..self.cols as usize - 1 {
                try!(write!(f,"{}, ",self[i][j]));
            }
            try!(write!(f,"{}",self[i][self.cols as usize - 1]));
            try!(write!(f,"]"));            
        }
        write!(f, "]\n")
    }
}


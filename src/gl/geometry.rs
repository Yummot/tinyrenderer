use std;
use std::iter::{FromIterator, IntoIterator};
use std::slice::Iter;
pub use gl::num::*;
use gl::num;

pub trait Vector: std::ops::Index<usize> 
                    + std::ops::Add 
                    + std::ops::Mul 
                    + std::ops::Sub
                    + Copy + Clone {
    type Vector;
    fn zero() -> Self::Vector;
}

pub trait Norm {
    type Output;
    type Normalize;
    fn norm(&self) -> Self::Output;
    fn normalize(&self) -> Self::Normalize;
}

pub trait Cast {
    type Output;
    fn cast<T>(&self) -> Self::Output;
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
            use std::ops::*;
            use std::cmp::PartialEq;
            
            #[derive(Debug, Copy, Clone)]
            pub struct $dst {
                data: [$t; $n],
            }
            
            impl $dst {
                #[allow(dead_code)]
                pub fn new(src: [$t;$n]) -> $dst {
                    $dst {
                        data: src,
                    }
                }
                #[allow(dead_code)]
                pub fn len(&self) -> usize { self.data.len() }
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
            
            impl Mul<$dst> for $dst {
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
                    let mut ret = $dst::zero();
                    for i in 0..$n {
                        ret[i] = cast::<f32,$t>(cast::<$t,f32>(self[i]).unwrap() * rhs).unwrap();
                    }
                    ret
                }
            }
            
            impl Mul<f64> for $dst {
                type Output = Self;
                fn mul(self, rhs: f64) -> $dst {
                    let mut ret = $dst::zero();
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
            
            impl PartialEq for $dst {
                #[allow(dead_code)]
                fn eq(&self, rhs: &Self) -> bool {
                    for i in 0..$n {
                        if self[i] != rhs[i] { return false; }
                    }
                    return true;
                }
            }
            impl Vector for $dst {
                type Vector = $dst;
                #[allow(dead_code)]
                fn zero() -> $dst {
                    $dst {
                        data: [0 as $t; $n],
                    }
                }
            }
        )*
    );
}

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
                where T: Clone + Mul<Output = T> + Add<Output = T> + Num
            {
                type Output = T;
                #[allow(dead_code)]
                fn mul(self, rhs: Self) -> T {
                    sum!($(self.$attr_name * rhs.$attr_name),*) 
                }
            }
            
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
                where T: Num + Clone + Sub<Output = T>
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
                where T: Num + Clone + Add<Output = T>
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
               where T: Num + NumCast + Copy {
                #[allow(dead_code)]
                pub fn new<N>($($attr_name : N),*) -> $dst<T>
                    where N: Num + NumCast + Copy {
                    $dst {
                        $($attr_name: num::cast::<N,T>($attr_name).unwrap(),)*
                    }
                }
                // #[allow(dead_code)]
                // pub fn zero() -> $dst<T> {
                //     $dst {
                //         $($attr_name: num::cast::<u8,T>(0).unwrap(),)*
                //     }
                // }
                #[allow(dead_code)]
                pub fn from_vec<N>(v: &[N]) -> $dst<T> 
                    where N: Num + NumCast + Copy {
                    let mut ret = $dst::zero();
                    for i in 0..v.len() {
                        ret[i] = num::cast::<N,T>(v[i]).unwrap();
                    }
                    ret
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
            
            impl<T> Vector for $dst<T> where T: Copy + Clone + Num + NumCast {
                type Vector = $dst<T>;
                #[allow(dead_code)]
                fn zero() -> Self::Vector {
                    $dst {
                        $($attr_name: num::cast::<u8,T>(0).unwrap(),)*
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
                where T: Clone + Copy + Mul<Output = T> + Add<Output = T> + NumCast + Num
            {
                type Output = f64;
                type Normalize = Self;
                #[allow(dead_code)]
                fn norm(&self) -> f64 {
                    (num::cast::<T,f64>(sum!($(self.$attr_name * self.$attr_name),*)).unwrap()).sqrt()
                }
                #[allow(dead_code)]
                fn normalize(&self) -> Self {
                    let mut ret = *self;
                    ret = ret * (1.0 / self.norm());
                    ret
                }
            }
        )*
    );
}

// pub fn embed<T>()

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

impl<T> Vec2<T> { pub fn len(&self) -> usize { 2 } }
impl<T> Vec3<T> { pub fn len(&self) -> usize { 3 } }

vec_create!(
    type Vec<4, f32> = Vec4f;
);



pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;
pub type Vec3f = Vec3<f32>;
pub type Vec3i = Vec3<i32>;

impl Vec3f {
    #[allow(dead_code)]
    pub fn embed(&self, fill: f32) -> Vec4f {
        let mut ret = Vec4f::zero();
        for i in 0..ret.len() {
            ret[i] = if i < self.len() { self[i] } else { fill };
        }     
        ret 
    }
}

impl Vec4f {
    #[allow(dead_code)]
    pub fn proj(&self) -> Vec3f {
        let mut ret = Vec3f::zero();
        for i in 0..ret.len() {
            ret[i] = self[i];
        }
        ret
    }
}

impl std::ops::Div<f32> for Vec4f {
    type Output = Vec4f;
    fn div(self, rhs: f32) -> Vec4f {
        let mut ret = Vec4f::zero();
        for i in 0..self.len() {
            ret[i] = self[i] / rhs;
        }
        ret
    }
}

impl Cast for Vec3i {
    type Output = Vec3f;
    fn cast<Vec3f>(&self) -> Self::Output {
        let mut ret = Vec3::<f32>::zero();
        ret.x = self.x as f32;
        ret.y = self.y as f32;
        ret.z = self.z as f32;
        ret
    }
}

impl Cast for Vec3f {
    type Output = Vec3i;
    fn cast<Vec3i>(&self) -> Self::Output {
        let mut ret = Vec3::<i32>::zero();
        ret.x = (self.x + 0.5) as i32;
        ret.y = (self.y + 0.5) as i32;
        ret.z = (self.z + 0.5) as i32;
        ret
    }
}

impl Cast for Vec2i {
    type Output = Vec2f;
    fn cast<Vec2f>(&self) -> Self::Output {
        let mut ret = Vec2::<f32>::zero();
        ret.x = self.x as f32;
        ret.y = self.y as f32;
        ret
    }
}

impl Cast for Vec2f {
    type Output = Vec2i;
    fn cast<Vec2i>(&self) -> Self::Output {
        let mut ret = Vec2::<i32>::zero();
        ret.x = (self.x + 0.5) as i32;
        ret.y = (self.y + 0.5) as i32;
        ret
    }
}





#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mat4 {
    pub mat: [f32; 16],
}

impl Mat4 {
    /// Return the zero matrix
    pub fn zero() -> Mat4 {
        Mat4 { mat: [0f32; 16] }
    }
    /// Return the identity matrix
    pub fn identity() -> Mat4 {
        Mat4 { mat:
            [1f32, 0f32, 0f32, 0f32,
             0f32, 1f32, 0f32, 0f32,
             0f32, 0f32, 1f32, 0f32,
             0f32, 0f32, 0f32, 1f32]
        }
    }
    /// Create the matrix using the values passed
    pub fn new(mat: [f32; 16]) -> Mat4 {
        Mat4 { mat: mat }
    }
    /// Access the element at row `i` column `j`
    pub fn at(&self, i: usize, j: usize) -> &f32 {
        &self.mat[4 * i + j]
    }
    /// Mutably access the element at row `i` column `j`
    pub fn at_mut(&mut self, i: usize, j: usize) -> &mut f32 {
        &mut self.mat[4 * i + j]
    }
    /// Compute and return the transpose of this matrix
    pub fn transpose(&self) -> Mat4 {
        let mut res = Mat4::zero();
        for i in 0..4 {
            for j in 0..4 {
                *res.at_mut(i, j) = *self.at(j, i);
            }
        }
        res
    }
    /// Compute and return the inverse of this matrix
    pub fn inverse(&self) -> Mat4 {
        //MESA's matrix inverse, tweaked for row-major matrices
        let mut inv = Mat4::zero();
        inv.mat[0] = self.mat[5] * self.mat[10] * self.mat[15]
            - self.mat[5]  * self.mat[11] * self.mat[14]
            - self.mat[9]  * self.mat[6]  * self.mat[15]
            + self.mat[9]  * self.mat[7]  * self.mat[14]
            + self.mat[13] * self.mat[6]  * self.mat[11]
            - self.mat[13] * self.mat[7]  * self.mat[10];

        inv.mat[4] = -self.mat[4]  * self.mat[10] * self.mat[15]
            + self.mat[4]  * self.mat[11] * self.mat[14]
            + self.mat[8]  * self.mat[6]  * self.mat[15]
            - self.mat[8]  * self.mat[7]  * self.mat[14]
            - self.mat[12] * self.mat[6]  * self.mat[11]
            + self.mat[12] * self.mat[7]  * self.mat[10];

        inv.mat[8] = self.mat[4]  * self.mat[9] * self.mat[15]
            - self.mat[4]  * self.mat[11] * self.mat[13]
            - self.mat[8]  * self.mat[5] * self.mat[15]
            + self.mat[8]  * self.mat[7] * self.mat[13]
            + self.mat[12] * self.mat[5] * self.mat[11]
            - self.mat[12] * self.mat[7] * self.mat[9];

        inv.mat[12] = -self.mat[4]  * self.mat[9] * self.mat[14]
            + self.mat[4]  * self.mat[10] * self.mat[13]
            + self.mat[8]  * self.mat[5] * self.mat[14]
            - self.mat[8]  * self.mat[6] * self.mat[13]
            - self.mat[12] * self.mat[5] * self.mat[10]
            + self.mat[12] * self.mat[6] * self.mat[9];

        inv.mat[1] = -self.mat[1]  * self.mat[10] * self.mat[15]
            + self.mat[1]  * self.mat[11] * self.mat[14]
            + self.mat[9]  * self.mat[2] * self.mat[15]
            - self.mat[9]  * self.mat[3] * self.mat[14]
            - self.mat[13] * self.mat[2] * self.mat[11]
            + self.mat[13] * self.mat[3] * self.mat[10];

        inv.mat[5] = self.mat[0]  * self.mat[10] * self.mat[15]
            - self.mat[0]  * self.mat[11] * self.mat[14]
            - self.mat[8]  * self.mat[2] * self.mat[15]
            + self.mat[8]  * self.mat[3] * self.mat[14]
            + self.mat[12] * self.mat[2] * self.mat[11]
            - self.mat[12] * self.mat[3] * self.mat[10];

        inv.mat[9] = -self.mat[0]  * self.mat[9] * self.mat[15]
            + self.mat[0]  * self.mat[11] * self.mat[13]
            + self.mat[8]  * self.mat[1] * self.mat[15]
            - self.mat[8]  * self.mat[3] * self.mat[13]
            - self.mat[12] * self.mat[1] * self.mat[11]
            + self.mat[12] * self.mat[3] * self.mat[9];

        inv.mat[13] = self.mat[0]  * self.mat[9] * self.mat[14]
            - self.mat[0]  * self.mat[10] * self.mat[13]
            - self.mat[8]  * self.mat[1] * self.mat[14]
            + self.mat[8]  * self.mat[2] * self.mat[13]
            + self.mat[12] * self.mat[1] * self.mat[10]
            - self.mat[12] * self.mat[2] * self.mat[9];

        inv.mat[2] = self.mat[1]  * self.mat[6] * self.mat[15]
            - self.mat[1]  * self.mat[7] * self.mat[14]
            - self.mat[5]  * self.mat[2] * self.mat[15]
            + self.mat[5]  * self.mat[3] * self.mat[14]
            + self.mat[13] * self.mat[2] * self.mat[7]
            - self.mat[13] * self.mat[3] * self.mat[6];

        inv.mat[6] = -self.mat[0]  * self.mat[6] * self.mat[15]
            + self.mat[0]  * self.mat[7] * self.mat[14]
            + self.mat[4]  * self.mat[2] * self.mat[15]
            - self.mat[4]  * self.mat[3] * self.mat[14]
            - self.mat[12] * self.mat[2] * self.mat[7]
            + self.mat[12] * self.mat[3] * self.mat[6];

        inv.mat[10] = self.mat[0]  * self.mat[5] * self.mat[15]
            - self.mat[0]  * self.mat[7] * self.mat[13]
            - self.mat[4]  * self.mat[1] * self.mat[15]
            + self.mat[4]  * self.mat[3] * self.mat[13]
            + self.mat[12] * self.mat[1] * self.mat[7]
            - self.mat[12] * self.mat[3] * self.mat[5];

        inv.mat[14] = -self.mat[0]  * self.mat[5] * self.mat[14]
            + self.mat[0]  * self.mat[6] * self.mat[13]
            + self.mat[4]  * self.mat[1] * self.mat[14]
            - self.mat[4]  * self.mat[2] * self.mat[13]
            - self.mat[12] * self.mat[1] * self.mat[6]
            + self.mat[12] * self.mat[2] * self.mat[5];

        inv.mat[3] = -self.mat[1] * self.mat[6] * self.mat[11]
            + self.mat[1] * self.mat[7] * self.mat[10]
            + self.mat[5] * self.mat[2] * self.mat[11]
            - self.mat[5] * self.mat[3] * self.mat[10]
            - self.mat[9] * self.mat[2] * self.mat[7]
            + self.mat[9] * self.mat[3] * self.mat[6];

        inv.mat[7] = self.mat[0] * self.mat[6] * self.mat[11]
            - self.mat[0] * self.mat[7] * self.mat[10]
            - self.mat[4] * self.mat[2] * self.mat[11]
            + self.mat[4] * self.mat[3] * self.mat[10]
            + self.mat[8] * self.mat[2] * self.mat[7]
            - self.mat[8] * self.mat[3] * self.mat[6];

        inv.mat[11] = -self.mat[0] * self.mat[5] * self.mat[11]
            + self.mat[0] * self.mat[7] * self.mat[9]
            + self.mat[4] * self.mat[1] * self.mat[11]
            - self.mat[4] * self.mat[3] * self.mat[9]
            - self.mat[8] * self.mat[1] * self.mat[7]
            + self.mat[8] * self.mat[3] * self.mat[5];

        inv.mat[15] = self.mat[0] * self.mat[5] * self.mat[10]
            - self.mat[0] * self.mat[6] * self.mat[9]
            - self.mat[4] * self.mat[1] * self.mat[10]
            + self.mat[4] * self.mat[2] * self.mat[9]
            + self.mat[8] * self.mat[1] * self.mat[6]
            - self.mat[8] * self.mat[2] * self.mat[5];

        let mut det = self.mat[0] * inv.mat[0] + self.mat[1] * inv.mat[4]
            + self.mat[2] * inv.mat[8] + self.mat[3] * inv.mat[12];
        assert!(det != 0f32);
        det = 1f32 / det;

        for x in &mut inv.mat {
            *x *= det;
        }
        inv
    }
    /// Return an iterator over the matrix's elements. The iterator goes
    /// row by row through the matrix.
    pub fn iter(&self) -> Iter<f32> {
        self.mat.iter()
    }
    pub fn has_nans(&self) -> bool {
        for x in &self.mat {
            if x.is_nan() {
                return true;
            }
        }
        false
    }
}

impl FromIterator<f32> for Mat4 {
    /// Create the matrix using the values from the iterator. The iterator should return
    /// the rows of the matrix one after another. The first 16 values returned will
    /// be used to set the matrix elements. If fewer than 16 values are returned the
    /// remaining entries will be 0
    fn from_iter<T: IntoIterator<Item = f32>>(it: T) -> Mat4 {
        let mut m = Mat4::zero();
        for (r, x) in m.mat.iter_mut().zip(it.into_iter()) {
            *r = x;
        }
        m
    }
}

impl<'a> FromIterator<&'a f32> for Mat4 {
    /// Create the matrix using the values from the iterator. The iterator should return
    /// the rows of the matrix one after another. The first 16 values returned will
    /// be used to set the matrix elements. If fewer than 16 values are returned the
    /// remaining entries will be 0
    fn from_iter<T: IntoIterator<Item = &'a f32>>(it: T) -> Mat4 {
        let mut m = Mat4::zero();
        for (r, x) in m.mat.iter_mut().zip(it.into_iter()) {
            *r = *x;
        }
        m
    }
}

impl Add for Mat4 {
    type Output = Mat4;
    /// Add two matrices together
    fn add(self, rhs: Mat4) -> Mat4 {
        self.mat.iter().zip(rhs.mat.iter()).map(|(&x, &y)| x + y).collect()
    }
}

impl Sub for Mat4 {
    type Output = Mat4;
    /// Subtract two matrices
    fn sub(self, rhs: Mat4) -> Mat4 {
        self.mat.iter().zip(rhs.mat.iter()).map(|(&x, &y)| x - y).collect()
    }
}

impl Mul for Mat4 {
    type Output = Mat4;
    /// Multiply two matrices
    fn mul(self, rhs: Mat4) -> Mat4 {
        let mut res = Mat4::zero();
        for i in 0..4 {
            for j in 0..4 {
                *res.at_mut(i, j) = *self.at(i, 0) * *rhs.at(0, j)
                    + *self.at(i, 1) * *rhs.at(1, j)
                    + *self.at(i, 2) * *rhs.at(2, j)
                    + *self.at(i, 3) * *rhs.at(3, j);
            }
        }
        res
    }
}

impl Mul<f32> for Mat4 {
    type Output = Mat4;
    /// Multiply the matrix by a scalar
    fn mul(self, rhs: f32) -> Mat4 {
        self.mat.iter().map(|&x| x * rhs).collect()
    }
}

impl Mul<Mat4> for f32 {
    type Output = Mat4;
    /// Multiply the matrix by a scalar
    fn mul(self, rhs: Mat4) -> Mat4 {
        rhs.mat.iter().map(|&x| x * self).collect()
    }
}


impl std::ops::Mul<Vec4f>  for Mat4 {
    type Output = Vec4f;
    fn mul(self, rhs: Vec4f) -> Vec4f {
        let mut ret = Vec4f::zero();
        for i in 0..4 {
            for j in 0..4 {
                ret[i] += self.at(i, j) * rhs[j];
            }
        }
        ret    
    } 
}



impl std::fmt::Display for Mat4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        try!(write!(f, "["));
        for i in 0..4 {
            if i == 0 { try!(write!(f,"[")); }
            else { try!(write!(f,"\n [")); }
            for j in 0..3 {
                try!(write!(f,"{}, ",self.at(i,j)));
            }
            try!(write!(f,"{}",self.at(i,3)));
            try!(write!(f,"]"));            
        }
        write!(f, "]\n")
    }
}

impl std::ops::Index<(usize,usize)> for Mat4 {
    type Output = f32;
    fn index<'a>(&'a self, idx: (usize, usize)) -> &'a f32 {
        if idx.0 > 3 || idx.1 > 3 { panic!("Index {:?} out of bounds.", idx) } 
        let (r, c) = idx;
        self.at(r,c)    
    }
}

impl std::ops::IndexMut<(usize,usize)> for Mat4 {
    fn index_mut<'a>(&'a mut self, idx: (usize, usize)) -> &'a mut f32 {
        if idx.0 > 3 || idx.1 > 3 { panic!("Index {:?} out of bounds.", idx) } 
        let (r, c) = idx;
        self.at_mut(r,c)    
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    data: [[f32;3];3],
}

impl Mat3 {
    pub fn zero() -> Mat3 { Mat3 { data: [[0.0;3];3] } }
    pub fn ncols(&self) -> usize { 3 }
    pub fn nrows(&self) -> usize { 3 }
    pub fn new(data:[f32;9]) -> Mat3 { 
        let mut ret = Self::zero();
        for i in 0..3 {
            for j in 0..3 {
                ret.data[i][j] = data[i * 3 + j];
            }
        } 
        ret 
    }
    pub fn indentity() -> Mat3 {
        Mat3 {
            data: [[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]]
        }
    }
    pub fn transpose(&self) -> Mat3 {
        let mut res = Mat3::zero();
        for i in 0..3 {
            for j in 0..3 {
                res[i][j] = self[j][i];
            }
        }
        res
    }
}

impl std::ops::Index<usize> for Mat3 {
    type Output = [f32;3];
    fn index<'a>(&'a self, idx: usize) -> &'a Self::Output {
        &self.data[idx]
    }
}

impl std::ops::IndexMut<usize> for Mat3 {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Self::Output {
        &mut self.data[idx]
    }
}

impl std::ops::Add<Mat3> for Mat3 {
    type Output = Mat3;
    fn add(self, rhs: Self) -> Mat3 {
        let mut ret = Self::zero();
        for i in 0..3 {
            for j in 0..3 {
                ret[i][j] = self[i][j] + rhs[i][j];    
            }
        }
        ret
    }
}

impl std::ops::Sub<Mat3> for Mat3 {
    type Output = Mat3;
    fn sub(self, rhs: Self) -> Mat3 {
        let mut ret = Self::zero();
        for i in 0..3 {
            for j in 0..3 {
                ret[i][j] = self[i][j] - rhs[i][j];    
            }
        }
        ret
    }
}

impl std::ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;
    fn mul(self, rhs: Self) -> Mat3 {
        let mut ret = Self::zero();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    ret[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        
        ret
    }
}

impl std::ops::Mul<Vec3f> for Mat3 {
    type Output = Vec3f;
    fn mul(self, rhs: Vec3f) -> Vec3f {
        let mut ret = Vec3f::zero();
        for i in 0..3 {
            for j in 0..3 {
                ret[i] += self[i][j] * rhs[j];
            }
        }
        ret    
    } 
}

// #[derive(Debug, Clone)]
// pub struct Mat {
//     data: Vec<Vec<f32>>,
//     rows: u32, 
//     cols: u32,
// }


// const DEFAULT_ALLOC: u32 = 4;

// impl Mat {
//     #[allow(dead_code)]
//     pub fn new(r: u32, c: u32) -> Mat {
//         Mat {
//             data: vec![vec![0.0;c as usize];r as usize],
//             rows: r,
//             cols: c,
//         }
//     }
//     #[allow(dead_code)]
//     pub fn builder<T>(data: &[&[T]]) -> Result<Mat,&'static str>
//        where T: Num + NumCast + Copy {
//         let checker = data[0].len();
//         let c = data[0].len();
//         let mut ret = Mat::new(data.len() as u32, c as u32);
//         for i in 0..data.len() {
//             if checker != data[i].len() { return Err("Mat::builder: Wrong input data.") } 
//             for j in 0..c {
//                 ret[i][j] = cast::<T,f32>(data[i][j]).unwrap();
//             }
//         }
        
//         return Ok(ret)
//     }
// }

// impl std::default::Default for Mat {
//     fn default() -> Self {
//         Mat {
//             data: vec![vec![0.0;DEFAULT_ALLOC as usize];DEFAULT_ALLOC as usize],
//             rows: DEFAULT_ALLOC,
//             cols: DEFAULT_ALLOC,
//         }
//     }
// }

// impl std::ops::Index<usize> for Mat {
//     type Output = Vec<f32>;
//     fn index<'a>(&'a self, idx: usize) -> &'a Vec<f32> {
//         &self.data[idx]
//     }
// }

// impl std::ops::IndexMut<usize> for Mat {
//     fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Vec<f32> {
//         &mut self.data[idx]
//     }
// }

// impl<'a> std::ops::Mul<&'a Mat> for &'a Mat {
//     type Output = Mat;
//     fn mul(self, rhs: &'a Mat) -> Mat {
//         if self.cols != rhs.rows { panic!("Mat::mul: Lhs.cols != Rhs.rows."); }
//         let mut res = Mat::new(self.rows, rhs.cols);
//         //TODO loop unrolling
//         for i in 0..self.rows as usize {
//             for j in 0..rhs.cols as  usize {
//                 for k in 0..self.cols as usize {
//                     res[i][j] += self[i][k] * rhs[k][j];
//                 }
//             }
//         }
        
//         res
//     }
// }


// impl Mat {
//     #[allow(dead_code)]
//     pub fn nrows(&self) -> u32 { self.rows }    
//     #[allow(dead_code)]
//     pub fn ncols(&self) -> u32 { self.cols }
//     #[allow(dead_code)]
//     pub fn identity(dimesions: u32) -> Mat {
//         let mut ret = Mat::new(dimesions, dimesions);
//         for i in 0..dimesions as usize {
//             ret[i][i] = 1.0;
//         }
//         ret
//     }
//     #[allow(dead_code)]
//     pub fn transpose(&self) -> Mat {
//         let mut ret = Mat::new(self.cols, self.rows);
//         for i in 0..self.rows as usize {
//             for j in 0..self.cols as usize {
//                 ret[j][i] = self[i][j];
//             }
//         }
//         ret 
//     }
//     #[allow(dead_code)]
//     pub fn mul(&self, rhs: &Mat) -> Mat {
//         if self.cols != rhs.rows { panic!("Mat::mul: Lhs.cols != Rhs.rows."); }
//         let mut res = Mat::new(self.rows, rhs.cols);
//         //TODO loop unrolling
//         for i in 0..self.rows as usize {
//             for j in 0..rhs.cols as  usize {
//                 for k in 0..self.cols as usize {
//                     res[i][j] += self[i][k] * rhs[k][j];
//                 }
//             }
//         }
        
//         res
//     }
//     #[allow(dead_code)]
//     pub fn inverse(&self) -> Mat {
//         if self.rows != self.cols { panic!("Mat::inverse not a square Matrix"); }
//         // augmenting the square matrix with the identity matrix of the same dimensions A => [AI]
//         let mut res = Mat::new(self.rows, self.cols * 2);
//         for i in 0..self.rows as usize {
//             for j in 0..self.cols as usize {
//                 res[i][j] = self[i][j];
//             }
//         }
//         for i in 0..self.rows as usize {
//             res[i][i + self.cols as usize] = 1.0;
//         }
//         // first pass
//         for i in 0..(self.rows - 1) as usize {
//             // normalize the first row
//             for j in (0..res.cols as usize).rev() {
//                 res[i][j] /= res[i][i];
//             }
//             for k in i+1..self.rows as usize {
//                 let coeff = res[k][i];
//                 for j in 0..res.cols as usize {
//                     res[k][j] -= res[i][j] * coeff;
//                 }
//             }
//         }
//         // normalize the last row
//         for j in (0..res.cols as usize).rev() {
//             res[self.rows as usize - 1][j] /= res[self.rows as usize - 1][self.rows as usize - 1];
//         }
//         // second pass
//         for i in (0..self.rows as usize).rev() {
//             for k in (0..i).rev() {
//                 let coeff = res[k][i];
//                 for j in 0..res.cols as usize {
//                     res[k][j] -= res[i][j] * coeff;    
//                 }
//             }
//         }
//         // cut the identity matrix back
//         let mut truncate = Mat::new(self.rows, self.cols);
//         for i in 0..self.rows as usize {
//             for j in 0..self.cols as usize {
//                 truncate[i][j] = res[i][j + self.cols as usize];
//             }
//         }

//         return truncate;
//     }
// }


// impl std::fmt::Display for Mat {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         try!(write!(f, "["));
//         for i in 0..self.rows as usize {
//             if i == 0 { try!(write!(f,"[")); }
//             else { try!(write!(f,"\n [")); }
//             for j in 0..self.cols as usize - 1 {
//                 try!(write!(f,"{}, ",self[i][j]));
//             }
//             try!(write!(f,"{}",self[i][self.cols as usize - 1]));
//             try!(write!(f,"]"));            
//         }
//         write!(f, "]\n")
//     }
// }
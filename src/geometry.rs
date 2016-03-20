pub use super::std::ops::*;
pub use super::std::cmp::PartialEq;
extern crate num;

use self::num::{Zero, NumCast};

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
                pub fn mul_num(&mut self, rhs: f64) -> $dst {
                    for mut iter in self.data.iter_mut() {
                        *iter = (*iter as f64 * rhs) as $t;
                    }
                    *self
                }
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
                    ret = ret.mul_num(1.0 / self.norm());
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
    x: T,
    y: T,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

macro_rules! vec_impl_helper {
    ($($dst : ident > ( $($attr_name : ident)*) ;)*) => (
        $(
            impl<T> Mul for $dst<T> 
                where T: Clone + Mul<Output = T> + Add<Output = T>
            {
                type Output = T;
                #[allow(dead_code)]
                fn mul(self, rhs: Self) -> T {
                    sum!($(self.$attr_name * rhs.$attr_name),*) as T
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
                    //vec_eq!(self.x == rhs.x, self.y == rhs.y)
                    vec_eq!($(self.$attr_name == rhs.$attr_name),*)
                }       
                fn ne(&self, rhs: &Self) -> bool {
                    !self.eq(rhs)
                }
            }
            
            impl<T> $dst<T> 
                where T: Mul<Output = T> + Clone + Copy + Zero + NumCast
            {
                #[allow(dead_code)]
                pub fn mul_num(&self, rhs: f64) -> $dst<T> {
                    $dst {
                        $(
                            $attr_name: num::cast::<f64,T>(num::cast::<T,f64>(self.$attr_name).unwrap() * rhs).unwrap(),
                        )*
                    }
                }
                #[allow(dead_code)]
                pub fn new($($attr_name : T),*) -> $dst<T> {
                    $dst {
                        $($attr_name: $attr_name,)*
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
                where T: Clone + Copy + Mul<Output = T> + Add<Output = T> + Zero + NumCast
            {
                type Output = f64;
                type Normalize = Self;
                #[allow(dead_code)]
                fn norm(&self) -> f64 {
                    (num::cast::<T,f64>(sum!($(self.$attr_name * self.$attr_name),*)).unwrap()).sqrt()
                }
                fn normalize(&self) -> $dst<T> {
                    let mut ret = *self;
                    ret = ret.mul_num(1.0 / self.norm());
                    ret
                }
            }
        )*
    );
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

/*
#[derive(Debug, Copy, Clone)]
pub struct Vec2f {
    x: f32,
    y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2i {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3f {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3i {
    x: i32,
    y: i32,
    z: i32,
}
*/



/*
macro_rules! norm_helper {
    ($($dst: ident ( $($attr_name : ident)*);)*) => (
        $(
            impl Norm for $dst {
                type Output = f64;
                type Normalize = Self;
                #[allow(dead_code)]
                fn norm(&self) -> f64 {
                    (sum!($(self.$attr_name * self.$attr_name),*) as f64).sqrt()
                }
                fn normalize(&self) -> $dst {
                    let mut ret = *self;
                    ret = ret.mul_num(1.0 / self.norm());
                    ret
                }
            }
        )*
    );
}

norm_helper!(
    Vec2f (x y);
    Vec2i (x y);
    Vec3f (x y z);
    Vec3i (x y z); 
);
*/
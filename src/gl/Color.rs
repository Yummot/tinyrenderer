use std::ops::{IndexMut, Index, Mul};
use std::mem::transmute;
#[derive(Debug, Clone, Copy)]
pub enum ColorType {
    GRAY(u8),
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
    VALUE(u32),
    None,
}
pub use self::ColorType::GRAY;
pub use self::ColorType::RGB as RGBColor;
pub use self::ColorType::RGBA as RGBAColor; 

impl ColorType {
    pub fn get_bgra_value(&self) -> u32 {
        use color::ColorType::*;
        match self {
            &GRAY(gray) => gray as u32,
            &RGB(r, g, b) => (b as u32) << 24 | (g as u32) << 16 | (r as u32) << 8,  
            &RGBA(r, g, b, a) => (b as u32) << 24 | (g as u32) << 16 | (r as u32) << 8 | (a as u32),
            &None => 0,
            &VALUE(v) => v,  
        }    
    }
    pub fn nbytes(&self) -> usize {
        match *self {
            ColorType::None => 0,
            ColorType::GRAY(_) => 1, 
            ColorType::RGB(..) => 3,
            ColorType::RGBA(..) => 4,
            _ => 0,
        } 
    }
}

impl Index<usize> for ColorType {
    type Output = u8;
    fn index<'a>(&'a self, idx: usize) -> &'a u8 {
        match self {
            &ColorType::GRAY(ref gray) => {
                if idx == 0 { gray } else { panic!("Error: ColorType::GRAY index {} is out of bounds.", idx) }    
            },
            &ColorType::RGB(ref r, ref g, ref b) => {
                match idx {
                    0 => b,
                    1 => g,
                    2 => r,
                    _ => panic!("Error: ColorType::RGB index {} is out of bounds.", idx),     
                }
            },
            &ColorType::RGBA(ref r, ref g, ref b, ref a) => {
                match idx {
                    0 => b,
                    1 => g,
                    2 => r,
                    3 => a,
                    _ => panic!("Error: ColorType::RGBA index {} is out of bounds.", idx),    
                }
            },
            _ => panic!("Error: Out of bounds while indexing ColorType::None."),
        }
    }
}

impl IndexMut<usize> for ColorType {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut u8 {
        match self {
            &mut ColorType::GRAY(ref mut gray) => {
                if idx == 0 { gray } else { panic!("Error: ColorType::GRAY index {} is out of bounds.", idx) }    
            },
            &mut ColorType::RGB(ref mut r, ref mut g, ref mut b) => {
                match idx {
                    0 => b,
                    1 => g,
                    2 => r,
                    _ => panic!("Error: ColorType::RGB index {} is out of bounds.", idx),     
                }
            },
            &mut ColorType::RGBA(ref mut r, ref mut g, ref mut b, ref mut a) => {
                match idx {
                    0 => b,
                    1 => g,
                    2 => r,
                    3 => a,
                    _ => panic!("Error: ColorType::RGBA index {} is out of bounds.", idx),    
                }
            },
            _ => panic!("Error: Out of bounds while indexing ColorType::None."),
        }
    }    
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Color {
    color: ColorType,
}
impl Color {
    #[allow(dead_code)] pub fn new() -> Color { Color { color: ColorType::None } }
    #[allow(dead_code)] pub fn with_color(color: ColorType) -> Color { Color { color: color } }
    #[allow(dead_code)] pub fn grayscale(gray: u8) -> Color { Color { color: ColorType::GRAY(gray) } }
    #[allow(dead_code)] pub fn val(&self) -> u32 { self.color.get_bgra_value() }
    #[allow(dead_code)] pub fn raw(&self) -> [u8;4] { unsafe { transmute::<u32,[u8;4]>(self.color.get_bgra_value()) } } 
    #[allow(dead_code)] pub fn nbytes(&self) -> usize { self.color.nbytes() }
    #[allow(dead_code)]
    pub fn set_val(&mut self, val: u32, bytespp: i32) {
        use color::ColorType::*;
        let tmp = unsafe { transmute::<u32,[u8;4]>(val) };
        match bytespp {
            1 => self.color = GRAY(tmp[0]),
            3 => self.color = RGB(tmp[2], tmp[1], tmp[0]),
            4 => self.color = RGBA(tmp[2], tmp[1], tmp[0], tmp[3]),  
            _ => {},   
        };
    }
}

impl Index<usize> for Color {
    type Output = u8;
    fn index<'a>(&'a self, idx: usize) -> &'a u8 { &self.color[idx] }
}
impl IndexMut<usize> for Color {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut u8 { &mut self.color[idx] }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        let mut ret = self;
        let rhs = if rhs > 1.0 { 1.0 } else if rhs < 0.0 { 0.0 } else { rhs };
        for i in 0..self.nbytes() {
            ret[i] = (ret[i] as f32 * rhs) as u8; 
        }
        ret
    }    
}


// #[allow(dead_code)] pub fn red(&self) -> u8 { self.color[2] }
// #[allow(dead_code)] pub fn green(&self) -> u8 { self.color[1] }
// #[allow(dead_code)] pub fn blue(&self) -> u8 { self.color[0] }
// #[allow(dead_code)] pub fn alpha(&self) -> u8 { self.color[3] }
// #[allow(dead_code)] pub fn set_red(&mut self, r: u8) { self.color[2] = r; }
// #[allow(dead_code)] pub fn set_green(&mut self, g: u8) { self.color[1] = g; }
// #[allow(dead_code)] pub fn set_blue(&mut self, b: u8) { self.color[0] = b; }
// #[allow(dead_code)] pub fn set_alpha(&mut self,a: u8) { self.color[3] = a; }
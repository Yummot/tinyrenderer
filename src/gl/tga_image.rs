use std;
use std::io::prelude::*;
use std::fs::File;
use std::ptr::copy as memmove;
use std::ptr::copy_nonoverlapping as memcpy;
use gl::num::{Num, NumCast, cast};

// pub trait Image {}
// impl Image for TGAImage{}

#[allow(dead_code)]
#[repr(packed)] //like c/c++ #program(push,1) /* */ #program(pop), remove padding in struct
pub struct TGAHeader {
    pub idlength: u8,
    pub colormaptype: u8,
    pub datatypecode: u8,
    pub colormaporigin: u16,
    pub colormaplength: u16,
    pub colormapdepth: u8,
    pub x_origin: u16,
    pub y_origin: u16,
    pub width: u16,
    pub height: u16,
    pub bitsperpixel: u8,
    pub imagedescriptor: u8,
}

impl TGAHeader {
    pub fn new() -> TGAHeader {
        TGAHeader {
            idlength: 0,
            colormaptype: 0,
            datatypecode: 0,
            colormaporigin: 0,
            colormaplength: 0,
            colormapdepth: 0,
            x_origin: 0,
            y_origin: 0,
            width: 0,
            height: 0,
            bitsperpixel: 0,
            imagedescriptor: 0,
        }
    }
}

#[allow(dead_code)]
pub const GRAYSCALE: isize = 1;
#[allow(dead_code)]
pub const RGB: isize = 3;
#[allow(dead_code)]
pub const RGBA: isize = 4;
#[allow(dead_code)]
pub const UNSET: isize = 0;

#[derive(Clone, Debug, Copy)]
pub struct TGAColor {
    pub bytespp: u32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}


pub struct RGBAColor(pub u8, pub u8, pub u8, pub u8);


#[allow(dead_code)]
pub fn u32_from_be(buf: &[u8]) -> u32 {
    if buf.len() == 4 {
        (buf[0] as u32) << 24 | (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | buf[3] as u32
    } else if buf.len() == 3 {
        (buf[0] as u32) << 16 | (buf[1] as u32) << 8 | buf[2] as u32
    } else if buf.len() == 2 {
        (buf[1] as u32) << 8 | buf[1] as u32
    } else if buf.len() == 1 {
        buf[0] as u32
    } else {
        panic!("Error: tga_image::base::u32_from_be bad buf parameter.")
    }
}

#[allow(dead_code)]
pub fn u32_from_le(buf: &[u8]) -> u32 {
    if buf.len() == 4 {
        (buf[3] as u32) << 24 | (buf[2] as u32) << 16 | (buf[1] as u32) << 8 | buf[0] as u32
    } else if buf.len() == 3 {
        (buf[2] as u32) << 16 | (buf[1] as u32) << 8 | buf[0] as u32
    } else if buf.len() == 2 {
        (buf[1] as u32) << 8 | buf[0] as u32
    } else if buf.len() == 1 {
        buf[0] as u32
    } else {
        panic!("Error: tga_image::base::u32_from_le bad buf parameter.")
    }
}

impl TGAColor {
    #[allow(dead_code)]
    pub fn new() -> TGAColor {
        TGAColor {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
            bytespp: 0,
        }
    }
    #[allow(dead_code)]
    pub fn with_color(rgba: RGBAColor) -> TGAColor {
        TGAColor {
            bytespp: RGBA as u32,
            b: rgba.2,
            g: rgba.1,
            r: rgba.0,
            a: rgba.3,
        }
    }


    #[inline]
    #[allow(dead_code)]
    pub fn raw(&self) -> [u8; 4] {
        [self.b, self.g, self.r, self.a]
    }
    #[inline]
    #[allow(dead_code)]
    pub fn set(&mut self, bgra: &[u8], bytespp: usize) -> Option<()> {
        if bytespp > 4 {
            None
        } else {
            for i in 0..bytespp {
                self[i] = bgra[i];
            }
            self.bytespp = bytespp as u32;
            Some(())
        }
    }
    #[inline]
    #[allow(dead_code)]
    pub fn red(&self) -> u8 { self.r }
    #[inline]
    #[allow(dead_code)]
    pub fn green(&self) -> u8 { self.g }
    #[inline]
    #[allow(dead_code)]
    pub fn blue(&self) -> u8 { self.b }
    #[inline]
    #[allow(dead_code)]
    pub fn alpha(&self) -> u8 { self.a }
    #[inline]
    #[allow(dead_code)]
    pub fn set_red(&mut self, r: u8) { self.r = r; }
    #[inline]
    #[allow(dead_code)]
    pub fn set_green(&mut self, g: u8) { self.g = g; }
    #[inline]
    #[allow(dead_code)]
    pub fn set_blue(&mut self, b: u8) { self.b = b; }
    #[inline]
    #[allow(dead_code)]
    pub fn set_alpha(&mut self, alpha: u8) { self.a = alpha; }
    #[inline]
    #[allow(dead_code)]
    pub fn val(&self) -> u32 {
        unsafe { std::mem::transmute::<[u8; 4], u32>(self.raw()) }
    }
    #[inline]
    #[allow(dead_code)]
    pub fn from_val(val: u32) -> TGAColor {
        let mut ret = TGAColor::new();
        ret.set_val(val, 4);
        ret
    }
    #[inline]
    #[allow(dead_code)]
    pub fn grayscale(val: u8) -> TGAColor {
        TGAColor {
            r: 0,
            g: 0,
            b: val,
            a: 0,
            bytespp: 1,
        }
    }
    #[inline]
    #[allow(dead_code)]
    pub fn set_val(&mut self, val: u32, bytespp: usize) {
        let raw = unsafe { std::mem::transmute::<u32, [u8; 4]>(val) };
        
        for i in 0..bytespp {
            self[i] = raw[i];
        }

        self.bytespp = match bytespp {
            1 => 1,
            3 => 3,
            4 => 4,
            _ => panic!("ERROR: TGAColor::set_val: Bad bytespp value."),
        };
    }
}

impl ::std::ops::IndexMut<usize> for TGAColor {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut u8 {
        match index {
            0 => &mut self.b,
            1 => &mut self.g,
            2 => &mut self.r,
            3 => &mut self.a,
            _ => panic!("Error: Out of bounds while indexing RGBAColor."),
        }
    }
}

// impl ::std::ops::IndexMut<char> for TGAColor {
//     fn index_mut<'a>(&'a mut self, index: char) -> &'a mut u8 {
//         match index {
//             'b' => &mut self.b,
//             'g' => &mut self.g,
//             'r' => &mut self.r,
//             'a' => &mut self.a,
//             _ => panic!("Error: Out of bounds while indexing RGBAColor.")
//         }
//     }
// }

impl ::std::ops::Index<usize> for TGAColor {
    type Output = u8;
    fn index<'a>(&'a self, index: usize) -> &'a u8 {
        match index {
            0 => &self.b,
            1 => &self.g,
            2 => &self.r,
            3 => &self.a,
            _ => panic!("Error: Out of bounds while indexing RGBAColor."),
        }
    }
}

impl std::ops::Mul<f32> for TGAColor {
    type Output = TGAColor;
    fn mul(self, rhs: f32) -> TGAColor {
        let mut ret = self;
        let rhs = if rhs > 1.0 { 1.0 } 
                  else if rhs < 0.0 { 0.0 }
                  else { rhs };
        for i in 0..4 {
            ret[i] = (ret[i] as f32 * rhs) as u8; 
        }
        ret
    }
}

#[derive(Debug, Clone)]
pub struct TGAImage {
    data: Vec<u8>,
    width: i32,
    height: i32,
    bytespp: i32,
}

#[allow(dead_code)]
pub const WRITE_RLE_FILE: bool = true;

#[allow(dead_code)]
fn read_header<R: Read>(reader: &mut R) -> Result<TGAHeader, &str> {
    let mut buf = [0u8; 18];

    reader.read_exact(&mut buf).unwrap();

    // let hdr = TGAHeader {
    //     idlength: buf[0],
    //     colormaptype: buf[1],
    //     datatypecode: buf[2],
    //     colormaporigin: u16_from_le(&buf[3..5]),
    //     colormaplength: u16_from_le(&buf[5..7]),
    //     colormapdepth: buf[7],
    //     x_origin: u16_from_le(&buf[8..10]),
    //     y_origin: u16_from_le(&buf[10..12]),
    //     width: u16_from_le(&buf[12..14]),
    //     height: u16_from_le(&buf[14..16]),
    //     bitsperpixel: buf[16],
    //     imagedescriptor: buf[17],
    // };

    let hdr = unsafe { std::mem::transmute::<[u8; 18], TGAHeader>(buf) };

    if hdr.width < 1 && hdr.height < 1 && hdr.colormaptype > 1 ||
       (hdr.colormaptype == 0 &&
        (hdr.colormaporigin > 0 || hdr.colormaplength > 0 || hdr.colormapdepth > 0)) {
        Err("corrupt TGA header")
    } else {
        Ok(hdr)
    }
}

// #[allow(dead_code)]
// fn write_header<W: Write>(ec: &mut W) -> Result<(),&str> {

// }

impl TGAImage {
    #[allow(dead_code)]
    pub fn new() -> TGAImage {
        TGAImage {
            data: vec![],
            width: 0,
            height: 0,
            bytespp: 0,
        }
    }
    #[allow(dead_code)]
    pub fn with_info(w: isize, h: isize, bpp: isize) -> TGAImage {
        use std::i32::MAX;
        if w <= MAX as isize && h <= MAX as isize && bpp <= MAX as isize {
            TGAImage {
                data: vec![0;(w * h * bpp) as usize],
                width: w as i32,
                height: h as i32,
                bytespp: bpp as i32,
            }
        } else {
            panic!("ERROR: Integer Overflow In TGAImage.(parameter wrong in w, h, or bbp.)");
        }
    }

    #[allow(dead_code)]
    pub fn read_tga_file(&mut self, filename: &str) {
        if !self.data.is_empty() {
            self.data = vec![]
        }
        let path = std::path::Path::new(filename);
        let mut file = std::fs::File::open(&path).unwrap();

        let tga_header = read_header(&mut file).unwrap();

        self.width = tga_header.width as i32;
        self.height = tga_header.height as i32;
        self.bytespp = (tga_header.bitsperpixel >> 3) as i32;

        if self.bytespp != GRAYSCALE as i32 && self.bytespp != RGB as i32 &&
           self.bytespp != RGBA as i32 {
            panic!("ERROR: bad bpp value.");
        }

        let nbytes = self.bytespp as usize * self.width as usize * self.height as usize;
        self.data = vec![0;nbytes];

        if tga_header.datatypecode == 2 || tga_header.datatypecode == 3 {
            if nbytes != file.read(&mut self.data).unwrap() {
                println!("Warning: Maybe get wrong data while file reading.");
            };
        } else if 10 == tga_header.datatypecode || 11 == tga_header.datatypecode {
            self.load_rle_data(&mut file);
        } else { panic!("Unkown file format {}", tga_header.datatypecode); }
        if (tga_header.imagedescriptor & 0x20) == 0 {
            // println!("fv");
            self.flip_vertically().unwrap();
        }
        if (tga_header.imagedescriptor & 0x10) != 0 {
            println!("fh");
            self.flip_horizontally().unwrap();
        }
    }

    #[allow(dead_code)]
    fn load_rle_data(&mut self, file: &mut File) -> Option<usize> {
        let pixelcount = self.width as usize * self.height as usize;
        let mut currentpixel = 0;
        let mut currentbyte = 0;
        // let mut colorbuffer = TGAColor::new();
        let mut chunkheader = [0u8];

        loop {
            file.read(&mut chunkheader).unwrap();
            if chunkheader[0] < 128 {
                chunkheader[0] += 1;
                for _ in 0..chunkheader[0] {
                    let mut cache = vec![0u8;self.bytespp as usize];
                    file.read(&mut cache).unwrap(); // TODO remove unwrap and refactor

                    // colorbuffer.set(&cache[0..], self.bytespp as usize);
                    // let raw = colorbuffer.raw();
                    for t in 0..self.bytespp as usize {
                        self.data[currentbyte] = cache[t];
                        currentbyte += 1;
                    }
                    currentpixel += 1;
                    if currentpixel > pixelcount {
                        return None;
                    }
                }
            } else {
                chunkheader[0] -= 127;
                let mut cache = vec![0u8;self.bytespp as usize];
                file.read(&mut cache).unwrap(); // TODO remove unwrap and refactor

                // colorbuffer.set(&cache[0..], self.bytespp as usize);
                // let raw = colorbuffer.raw();
                for _ in 0..chunkheader[0] {
                    for t in 0..self.bytespp as usize {
                        self.data[currentbyte] = cache[t];
                        currentbyte += 1;
                    }
                    currentpixel += 1;
                    if currentpixel > pixelcount {
                        return None;
                    }
                }
            }
            if currentpixel >= pixelcount {
                break;
            }
        }
        // println!("{}, {}", currentpixel, pixelcount);
        Some(pixelcount)
    }

    fn unload_rle_data<W: Write>(&self, dst: &mut W) -> Result<(), &str> {
        let max_chunk_length = 128;
        let npixels = self.width as usize * self.height as usize;
        let mut curpixel = 0;

        while curpixel < npixels {
            let chunkstart = curpixel * self.bytespp as usize;
            let mut curbyte = curpixel * self.bytespp as usize;
            let mut run_length = 1u8;
            let mut raw = true;
            while (curpixel + run_length as usize) < npixels && run_length < max_chunk_length {
                let mut succ_eq = true;
                for i in 0..self.bytespp as usize {
                    if self.data[curbyte + i] == self.data[curbyte + i + self.bytespp as usize] {
                        succ_eq = true;
                    } else {
                        succ_eq = false;
                        break;
                    }
                }
                curbyte += self.bytespp as usize;
                if run_length == 1 {
                    raw = !succ_eq;
                }
                if raw && succ_eq {
                    run_length -= 1;
                    break;
                }
                if !raw && !succ_eq {
                    break;
                }
                run_length += 1;
            }
            curpixel += run_length as usize;
            if raw {
                dst.write(&[run_length - 1]).unwrap();
            } else {
                dst.write(&[run_length + 127]).unwrap();
            }

            if raw {
                dst.write(&self.data[chunkstart..(chunkstart +
                                                  run_length as usize * self.bytespp as usize)])
                   .unwrap();
            } else {
                dst.write(&self.data[chunkstart..(chunkstart + self.bytespp as usize)]).unwrap();
            }
        }

        Ok(())
    }

    fn swap_line(&mut self, dst_idx: usize, src_idx: usize, bytes_per_line: usize) {
        let mut tmp_line = vec![0u8;bytes_per_line];

        unsafe {
            memmove(self.data[src_idx..].as_ptr(),
                    tmp_line.as_mut_ptr(),
                    bytes_per_line);
            memmove(self.data[dst_idx..].as_ptr(),
                    self.data[src_idx..].as_mut_ptr(),
                    bytes_per_line);
            memmove(tmp_line.as_ptr(),
                    self.data[dst_idx..].as_mut_ptr(),
                    bytes_per_line);
        }
    }

    #[allow(dead_code)]
    pub fn flip_vertically(&mut self) -> Result<(), &str> {
        if self.data.is_empty() {
            return Err("Error: data buffer len is 0.");
        }

        let bytes_per_line = self.width as usize * self.bytespp as usize;
        let half = self.height / 2;

        for i in 0..half as usize {
            let l1 = i * bytes_per_line;
            let l2 = (self.height as usize - 1 - i) * bytes_per_line;
            self.swap_line(l1, l2, bytes_per_line);
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn flip_horizontally(&mut self) -> Result<(), &str> {
        if self.data.is_empty() {
            return Err("Error: data buffer len is 0.");
        }

        // let half = self.width >> 1;
        let half = self.width / 2;
        let width = self.width as usize;

        for i in 0..half as usize {
            for j in 0..self.height as usize {
                let color_1 = self.get(i, j);
                let color_2 = self.get(width - 1 - i, j);
                self.set(i, j, color_2);
                self.set(width - 1 - i, j, color_1);
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn scale(&mut self, w: usize, h: usize) -> Result<(), &str> {
        if self.data.is_empty() {
            return Err("Error: data buffer len is 0.");
        }
        let mut tdata = vec![0;w * h * self.bytespp as usize];
        let nlinebytes = w * self.bytespp as usize;
        let olinebytes = self.width as usize * self.bytespp as usize;
        let mut nscanline = 0;
        let mut oscanline = 0;
        let mut erry = 0isize;

        for _ in 0..self.height as usize {
            let mut errx = (self.width as usize - w) as isize;
            let mut nx = -self.bytespp;
            let mut ox = -self.bytespp;
            for _ in 0..self.height as usize {
                ox += self.bytespp;
                errx += w as isize;
                while errx > self.width as isize {
                    errx -= self.width as isize;
                    nx += w as i32;
                    unsafe {
                        memcpy(self.data[(oscanline + ox as usize)..].as_ptr(),
                               tdata[(nscanline + nx as usize)..].as_mut_ptr(),
                               self.bytespp as usize);
                    }
                }
            }
            erry += h as isize;
            oscanline += olinebytes;
            while erry > self.height as isize {
                if erry >= (self.height << 2) as isize {
                    unsafe {
                        memcpy(tdata[nscanline..].as_ptr(),
                               tdata[(nscanline + nlinebytes)..].as_mut_ptr(),
                               nlinebytes);
                    }
                }
                erry -= self.height as isize;
                nscanline += nlinebytes;
            }
        }

        self.width = w as i32;
        self.height = h as i32;
        self.data = tdata;

        Ok(())

    }

    #[allow(dead_code)]
    pub fn get<X, Y>(&self, x: X, y: Y) -> TGAColor
        where X: Num + Copy + NumCast, Y: Num + Copy + NumCast
    {
        let x = cast::<X, usize>(x).unwrap();
        let y = cast::<Y, usize>(y).unwrap();
        if self.data.is_empty() || x >= self.width as usize || y >= self.height as usize {
            // return Err("Warning: x (y, x > width, y > height, or no data to get TGAColor.");
            return TGAColor::new();
        }
        let mut ret = TGAColor::new();
        let raw_val = u32_from_le(
            &self.data[((x + y * self.width as usize) * self.bytespp as usize)..((x + y * self.width as usize) * self.bytespp as usize + self.bytespp as usize)]
            );
        ret.set_val(raw_val, self.bytespp as usize);
        return ret;
    }

    #[allow(dead_code)]
    pub fn set<X, Y>(&mut self, x: X, y: Y, color: TGAColor) -> bool
        where X: Num + Copy + NumCast, Y: Num + Copy + NumCast
    {
        let x = cast::<X, usize>(x).unwrap();
        let y = cast::<Y, usize>(y).unwrap();
        if self.data.is_empty() || x >= self.width as usize || y >= self.height as usize {
            return false;
        }
        unsafe {
            memcpy(color.raw().as_ptr(),
                   self.data[((x + y * self.width as usize) * self.bytespp as usize)..]
                       .as_mut_ptr(),
                   self.bytespp as usize);
        }

        true
    }

    #[allow(dead_code)]
    pub fn get_width(&self) -> i32 {
        self.width
    }

    #[allow(dead_code)]
    pub fn get_height(&self) -> i32 {
        self.height
    }

    #[allow(dead_code)]
    pub fn get_bytespp(&self) -> i32 {
        self.bytespp
    }

    #[allow(dead_code)]
    pub fn buffer(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) -> Option<usize> {
        let ret = self.data.len();
        ::std::mem::replace(&mut self.data, vec![]);

        Some(ret)
    }

    #[allow(dead_code)]
    pub fn write_tga_file(&self, filename: &str, rle: bool) -> Result<(), &str> {
        let developer_area_ref = [0u8; 4];
        let extension_area_ref = [0u8; 4];

        let footer = [b'T', b'R', b'U', b'E', b'V', b'I', b'S', b'I', b'O', b'N', b'-', b'X',
                      b'F', b'I', b'L', b'E', b'.', b'\0'];

        let path = std::path::Path::new(filename);
        let mut file = match std::fs::File::create(&path) {
            Err(_) => std::fs::File::create(&path).unwrap(),
            Ok(file) => file,
        };

        let mut header = TGAHeader::new();
        header.bitsperpixel = (self.bytespp << 3) as u8;
        header.width = self.width as u16;
        header.height = self.height as u16;
        header.datatypecode = if self.bytespp == GRAYSCALE as i32 {
            if rle {
                11
            } else {
                3
            }
        } else {
            if rle {
                10
            } else {
                2
            }
        };
        header.imagedescriptor = 0x20;

        let header_bytes_buf = unsafe { std::mem::transmute::<TGAHeader, [u8; 18]>(header) };
        match file.write(&header_bytes_buf) {
            Err(_) => {
                return Err("Error: TGAImage::write_tga_file can't dump the tga \
                            file.(header_bytes_buf)")
            }
            _ => {}
        }

        if !rle {
            match file.write(&self.data[0..(self.width * self.height * self.bytespp) as usize]) {
                Err(_) => {
                    return Err("Error: TGAImage::write_tga_file can't unload raw data.(self.data)")
                }
                _ => {}
            };
        } else {
            match self.unload_rle_data(&mut file) {
                Err(_) => return Err("{}\nError: TGAImage::write_tga_file can't unload rle data."),
                _ => {}
            }
        };

        match file.write(&developer_area_ref) {
            Err(_) => {
                return Err("Error: TGAImage::write_tga_file can't dump the tga \
                            file.(developer_area_ref)")
            }
            _ => {}
        }

        match file.write(&extension_area_ref) {
            Err(_) => {
                return Err("Error: TGAImage::write_tga_file can't dump the tga \
                            file.(extension_area_ref)")
            }
            _ => {}
        }

        match file.write(&footer) {
            Err(_) => {
                return Err("Error: TGAImage::write_tga_file can't dump the tga file.(footer)")
            }
            _ => {}
        }

        Ok(())
    }
}

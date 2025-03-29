// This file will convert image files (.png .jpg ...) to eternal image format (.eimg) that stores the images as a list of pixels (and a header for dimensions)
// Suported file types : png, jpg, tiff, gif, webp, ico, bmp
extern crate image;
use std::{fs, io::Read};

#[derive(Clone, Debug)]
pub struct Image {
    pub p: Vec<(u8, u8, u8, u8)>, // rgba
    pub w: u32,
    pub h: u32,
}
impl Image {
    pub fn to_file(&self, path: &str) {
        let mut bytes:Vec<u8> = Vec::with_capacity(8 + (self.h * self.w) as usize);
        // Add the header width then height both transformed into 4 u8
        bytes.push(((self.w >> 24) & 0xFF) as u8);
        bytes.push(((self.w >> 16) & 0xFF) as u8);
        bytes.push(((self.w >> 8) & 0xFF) as u8);
        bytes.push((self.w & 0xFF) as u8);
        bytes.push(((self.h >> 24) & 0xFF) as u8);
        bytes.push(((self.h >> 16) & 0xFF) as u8);
        bytes.push(((self.h >> 8) & 0xFF) as u8);
        bytes.push((self.h & 0xFF) as u8);
        // Unpack the rgba tuples and add it to the array
        for (red,green,blue,alpha) in &self.p {
            bytes.push(*red);
            bytes.push(*green);
            bytes.push(*blue);
            bytes.push(*alpha);
        }
        std::fs::write(path, bytes).unwrap();
    }
    pub fn from_file(path: &str) -> Image {
        let mut file = fs::File::open(path).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        // Read the header
        let w = ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | (bytes[3] as u32);
        let h = ((bytes[4] as u32) << 24) | ((bytes[5] as u32) << 16) | ((bytes[6] as u32) << 8) | (bytes[7] as u32);
        // Read the pixels
        let mut pixels: Vec<(u8,u8,u8,u8)> = Vec::with_capacity((w * h) as usize);
        for i in 0..(w * h) {
            let red = bytes[(i * 4 + 8) as usize];
            let green = bytes[(i * 4 + 9) as usize];
            let blue = bytes[(i * 4 + 10) as usize];
            let alpha = bytes[(i * 4 + 11) as usize];
            pixels.push((red, green, blue, alpha));
        }
        // Return the image
        Image { p: pixels, w: w, h: h }
    }


    pub fn from(path: &str) -> Image {
        let img = image::open(path).unwrap().to_rgba8();

        let bytes: Vec<u8> = img.bytes().filter_map(Result::ok).collect();
        let mut pixels: Vec<(u8,u8,u8,u8)> = Vec::with_capacity(bytes.len() / 4);
        for i in 0..(bytes.len()/4) {
            pixels.push((bytes[i*4], bytes[i*4+1], bytes[i*4+2], bytes[i*4+3]));
        }

        return Image { p: pixels, w: img.width(), h: img.height() }
    }

    pub fn to_rgba(&self, path: &str) {
        let mut img = image::ImageBuffer::new(self.w, self.h);
        for (i, pixel) in self.p.iter().enumerate() {
            img.put_pixel((i % self.w as usize) as u32, (i / self.w as usize) as u32, image::Rgba([pixel.0, pixel.1, pixel.2, pixel.3]));
        }
        img.save(path).unwrap();
    }

    pub fn to_rgb(&self, path: &str) {
        let mut img = image::ImageBuffer::new(self.w, self.h);
        for (i, pixel) in self.p.iter().enumerate() {
            img.put_pixel((i % self.w as usize) as u32, (i / self.w as usize) as u32, image::Rgb([pixel.0, pixel.1, pixel.2]));
        }
        img.save(path).unwrap();
    }
}
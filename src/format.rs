use crate::bmp::{Image, Pixel};
use crate::vec::Color;
use std::ffi::OsStr;
use std::fs;
use std::io::{Result, Write};
use std::iter::Rev;
use std::ops::Range;
use std::path::Path;

pub trait Format {
    type IntoIter: IntoIterator<Item = u32>;

    fn new(width: u32, height: u32) -> Self;

    fn get_width(&self) -> u32;

    fn get_height(&self) -> u32;

    fn iter_rows(&self) -> Self::IntoIter;

    fn set_pixel(&mut self, x: u32, y: u32, color: Color);

    fn get_pixel(&self, x: u32, y: u32) -> Color;

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>;

    fn to_writer<W: Write>(&self, destination: &mut W) -> Result<()>;
}

pub struct Ppm {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl Ppm {
    fn get_pixel(&self, x: u32, y: u32) -> usize {
        ((self.height - y - 1) * self.width + x) as usize
    }
}

impl Format for Ppm {
    type IntoIter = Rev<Range<u32>>;

    fn new(width: u32, height: u32) -> Self {
        Ppm {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); (width * height) as usize],
        }
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn iter_rows(&self) -> Self::IntoIter {
        (0..self.get_height()).rev()
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let i = self.get_pixel(x, self.get_height() - y - 1);
        self.pixels[i] = color;
    }

    fn get_pixel(&self, x: u32, y: u32) -> Color {
        let i = self.get_pixel(x, y);
        self.pixels[i]
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut bmp_file = if path.as_ref().extension() != Some(OsStr::new("ppm")) {
            fs::File::create(path.as_ref().with_extension("ppm"))?
        } else {
            fs::File::create(path)?
        };
        self.to_writer(&mut bmp_file)
    }

    fn to_writer<W: Write>(&self, destination: &mut W) -> Result<()> {
        destination.write_all("P3\n".as_bytes())?;
        destination.write_all(format!("{} {}\n", self.width, self.height).as_bytes())?;
        destination.write_all("255\n".as_bytes())?;
        for pixel in self.pixels.iter().rev() {
            destination
                .write_all(format!("{} {} {}\n", pixel.r(), pixel.g(), pixel.b()).as_bytes())?;
        }
        Ok(())
    }
}

pub struct Bmp {
    image: Image,
}

impl Format for Bmp {
    type IntoIter = Range<u32>;

    fn new(width: u32, height: u32) -> Self {
        Bmp {
            image: Image::new(width, height),
        }
    }

    fn get_width(&self) -> u32 {
        self.image.get_width()
    }

    fn get_height(&self) -> u32 {
        self.image.get_height()
    }

    fn iter_rows(&self) -> Self::IntoIter {
        0..self.get_height()
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.image.set_pixel(
            x,
            self.get_height() - y - 1,
            Pixel::new(color.r(), color.g(), color.b()),
        )
    }

    fn get_pixel(&self, x: u32, y: u32) -> Color {
        let pixel = self.image.get_pixel(x, y);
        Color::new(
            pixel.r as f32 / 255.0,
            pixel.g as f32 / 255.0,
            pixel.b as f32 / 255.0,
        )
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if path.as_ref().extension() != Some(OsStr::new("bmp")) {
            self.image.save(path.as_ref().with_extension("bmp"))
        } else {
            self.image.save(path)
        }
    }

    fn to_writer<W: Write>(&self, destination: &mut W) -> Result<()> {
        self.image.to_writer(destination)
    }
}

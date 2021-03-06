// use crate::{colors::color, tuples::Tuple};
// use super::colors::Color;
use std::io::{Result, Write};

use crate::tuples::{color, Tuple};

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Tuple>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut c = Canvas {
            width,
            height,
            pixels: Vec::with_capacity(width * height),
        };
        for _ in 0..(width * height) {
            let black = color(0.0, 0.0, 0.0);
            c.pixels.push(black);
        }
        c
    }

    pub fn at(&self, w: usize, h: usize) -> &Tuple {
        let i = self.index(w, h);
        &self.pixels[i]
    }

    fn index(&self, w: usize, h: usize) -> usize {
        w + h * self.width
    }

    pub fn set(&mut self, w: usize, h: usize, c: Tuple) {
        let i = self.index(w, h);
        self.pixels[i] = c;
    }

    pub fn fill(&mut self, c: Tuple) {
        for n in 0..(self.width * self.height) {
            self.pixels[n] = c.clone();
        }
    }

    pub fn ppm(&self, writer: &mut dyn Write) -> Result<()> {
        writer.write_all(b"P3\n")?;
        writer.write_all(format!("{} {}\n", self.width, self.height).as_bytes())?;
        writer.write_all(b"255\n")?;

        let mut length = 0;

        for h in 0..self.height {
            for w in 0..self.width {
                let i = self.index(w, h);
                let c = &self.pixels[i];

                length = add_color(writer, length, c.x)?;
                length = add_color(writer, length, c.y)?;
                length = add_color(writer, length, c.z)?;
            }
            writer.write_all(b"\n")?;
            length = 0;
        }

        Ok(())
    }
}

fn add_color(w: &mut dyn Write, mut length: u8, c: f64) -> Result<u8> {
    let r = clamp(c * 255.0, 0, 255);
    let original_length = length;

    if original_length > 0 {
        length += 1;
    }

    length += 1;
    if r > 9 {
        length += 1;
    }
    if r > 99 {
        length += 1;
    }

    if length > 70 {
        w.write_all(b"\n")?;
        length = 0;
        length += 1;
        if r > 9 {
            length += 1;
        }
        if r > 99 {
            length += 1;
        }
    } else if original_length > 0 {
        w.write_all(b" ")?;
    }

    w.write_all(format!("{}", r).as_bytes())?;

    Ok(length)
}

fn clamp(v: f64, min: u8, max: u8) -> u8 {
    let mut r = v.round() as u8;
    if r < min {
        r = min;
    }
    if r > max {
        r = max;
    }
    r
}

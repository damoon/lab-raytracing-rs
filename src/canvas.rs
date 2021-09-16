use super::colors::Color;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut c = Canvas {
            width,
            height,
            pixels: Vec::with_capacity(width * height),
        };
        let color = Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
        for _ in 0..(width * height) {
            c.pixels.push(color.clone());
        }
        c
    }

    pub fn at(&self, w: usize, h: usize) -> Color {
        let i = self.index(w, h);
        self.pixels[i]
    }

    fn index(&self, w: usize, h: usize) -> usize {
        w + h * self.width
    }

    pub fn set(&mut self, w: usize, h: usize, c: Color) {
        let i = self.index(w, h);
        self.pixels[i] = c;
    }

    pub fn fill(&mut self, c: Color) {
        for n in 0..(self.width * self.height) {
            self.pixels[n] = c.clone();
        }
    }

    pub fn ppm(&self) -> String {
        let mut s = String::new();
        s.push_str("P3\n");
        s.push_str(format!("{} {}\n", self.width, self.height).as_str());
        s.push_str("255\n");

        let mut length = 0;

        for h in 0..self.height {
            for w in 0..self.width {
                let i = self.index(w, h);
                let c = self.pixels[i];

                let a = add_color(s, length, c.r);
                s = a.0;
                length = a.1;
                let a = add_color(s, length, c.g);
                s = a.0;
                length = a.1;
                let a = add_color(s, length, c.b);
                s = a.0;
                length = a.1;
            }
            s.push_str("\n");
            length = 0;
        }

        s
    }
}

fn add_color(mut s: String, mut length: u8, c: f32) -> (String, u8) {
    let r = clamp(c * 255.0, 0, 255);
    let original_length = length;

    if original_length > 0 {
        length = length + 1;
    }

    length = length + 1;
    if r > 9 {
        length = length + 1;
    }
    if r > 99 {
        length = length + 1;
    }

    if length > 70 {
        s.push_str("\n");
        length = 0;
        length = length + 1;
        if r > 9 {
            length = length + 1;
        }
        if r > 99 {
            length = length + 1;
        }
    } else if original_length > 0 {
        s.push_str(" ");
    }

    s.push_str(format!("{}", r).as_str());

    (s, length)
}

fn clamp(v: f32, min: u8, max: u8) -> u8 {
    let mut r = v.round() as u8;
    if r < min {
        r = min;
    }
    if r > max {
        r = max;
    }
    r
}

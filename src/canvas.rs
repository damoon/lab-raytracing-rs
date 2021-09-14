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
        let color = Color{r:0.0, g:0.0, b:0.0};
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
}

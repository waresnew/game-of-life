use crate::point::ScreenPoint;

pub struct ImageBitmap {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}
const RGBA_LEN: usize = 4;
const DEAD_COLOUR: Rgb = Rgb::new(255, 255, 255);
const ALIVE_COLOUR: Rgb = Rgb::new(0, 0, 0);
#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
impl ImageBitmap {
    pub fn new(canvas_dims: ScreenPoint) -> Self {
        let width = usize::try_from(canvas_dims.x).unwrap();
        let height = usize::try_from(canvas_dims.y).unwrap();
        Self {
            pixels: vec![DEAD_COLOUR; width * height]
                .iter()
                .flat_map(|x| [x.r, x.g, x.b, 255])
                .collect(),
            width,
            height,
        }
    }
    pub fn fill_cell(&mut self, tl: ScreenPoint, size_exp: u32) {
        let size = 1 << size_exp;
        for dx in 0..size {
            for dy in 0..size {
                let x = tl.x + dx;
                let y = tl.y + dy;
                if y >= self.height as i64 || x >= self.width as i64 || y < 0 || x < 0 {
                    continue;
                }
                let pos = usize::try_from(y).unwrap() * self.width + usize::try_from(x).unwrap();
                self.pixels[RGBA_LEN * pos] = ALIVE_COLOUR.r;
                self.pixels[RGBA_LEN * pos + 1] = ALIVE_COLOUR.g;
                self.pixels[RGBA_LEN * pos + 2] = ALIVE_COLOUR.b;
                self.pixels[RGBA_LEN * pos + 3] = 255;
            }
        }
    }
    pub fn fill_pixel(&mut self, p: ScreenPoint, colour: Rgb) {
        if p.x < 0 || p.y < 0 || p.x >= self.width as i64 || p.y >= self.height as i64 {
            return;
        }
        let pos = usize::try_from(p.y).unwrap() * self.width + usize::try_from(p.x).unwrap();
        self.pixels[RGBA_LEN * pos] = colour.r;
        self.pixels[RGBA_LEN * pos + 1] = colour.g;
        self.pixels[RGBA_LEN * pos + 2] = colour.b;
        self.pixels[RGBA_LEN * pos + 3] = 255;
    }
    pub fn into_pixels(self) -> Vec<u8> {
        self.pixels
    }
}

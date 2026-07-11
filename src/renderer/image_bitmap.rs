use crate::renderer::{CellPoint, ScreenPoint};

pub struct ImageBitmap {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}
impl ImageBitmap {
    pub fn new(canvas_dims: CellPoint) -> Self {
        let width = usize::try_from(canvas_dims.x).unwrap();
        let height = usize::try_from(canvas_dims.y).unwrap();
        Self {
            pixels: vec![0; width * height],
            width,
            height,
        }
    }
    pub fn fill(&mut self, bl: CellPoint, size_exp: u32) {
        let size = 1 << size_exp;
        let tl = CellPoint::new(bl.x, bl.y - size);
        for dx in 0..size {
            for dy in 0..size {
                let x = tl.x + dx;
                let y = tl.y + dy;
                if y >= self.height as i64 || x >= self.width as i64 || y < 0 || x < 0 {
                    continue;
                }
                assert_eq!(self.pixels[y as usize * self.width + x as usize], 0);
                self.pixels[y as usize * self.width + x as usize] = 1;
            }
        }
    }
    pub fn into_pixels(self) -> Vec<u8> {
        self.pixels
    }
}

use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;

pub trait Line {
    fn line(&mut self, start: Vertex, end: Vertex);
}

impl Line for Framebuffer {
    fn line(&mut self, start: Vertex, end: Vertex) {
        let mut x0 = start.x.round();
        let mut y0 = start.y.round();
        let x1 = end.x.round();
        let y1 = end.y.round();

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1.0 } else { -1.0 };
        let sy = if y0 < y1 { 1.0 } else { -1.0 };
        let mut err = dx + dy;

        loop {
            self.point(x0 as isize, y0 as isize);

            if x0 == x1 && y0 == y1 { break; }

            let e2 = 2.0 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

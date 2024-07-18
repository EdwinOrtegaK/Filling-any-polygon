use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;
use crate::line_impl::Line;

pub trait Polygon {
    fn draw_polygon(&mut self, points: &[Vertex]);
}

impl Polygon for Framebuffer {
    fn draw_polygon(&mut self, points: &[Vertex]) {
        if points.len() < 2 {
            return;
        }

        for i in 0..points.len() - 1 {
            self.line(points[i], points[i + 1]);
        }

        // Dibujar la última línea de regreso al primer punto
        self.line(points[points.len() - 1], points[0]);
    }
}

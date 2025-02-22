mod color;
mod framebuffer;
mod line_impl;
mod bmp;
mod vertex;
mod polygon_impl;

use framebuffer::Framebuffer;
use line_impl::Line;
use vertex::{new_vertex, Vertex};
use polygon_impl::Polygon;

fn draw_polygon(framebuffer: &mut Framebuffer, vertices: &[Vertex]) {
    if vertices.len() < 3 {
        eprintln!("Error: Se requieren al menos 3 vértices para formar un polígono.");
        return;
    }

    for i in 0..vertices.len() {
        let start = vertices[i];
        let end = if i == vertices.len() - 1 {
            vertices[0]
        } else {
            vertices[i + 1]
        };
        framebuffer.line(start, end);
    }

    framebuffer.line(vertices[vertices.len() - 1], vertices[0]);
}

fn fill_polygon(framebuffer: &mut Framebuffer, vertices: &[Vertex]) {
    let min_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, |a, b| a.min(b)).ceil() as isize;
    let max_y = vertices.iter().map(|v| v.y).fold(f32::NEG_INFINITY, |a, b| a.max(b)).floor() as isize;

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        for i in 0..vertices.len() {
            let start = vertices[i];
            let end = if i == vertices.len() - 1 { vertices[0] } else { vertices[i + 1] };

            if (start.y <= y as f32 && end.y > y as f32) || (end.y <= y as f32 && start.y > y as f32) {
                let t = (y as f32 - start.y) / (end.y - start.y);
                let x = start.x + t * (end.x - start.x);
                intersections.push(x);
            }
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i].ceil() as isize;
                let x_end = intersections[i + 1].floor() as isize;

                for x in x_start..=x_end {
                    framebuffer.point(x, y);
                }
            }
        }
    }
}

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(0x000000); // Negro
    framebuffer.clear();

    // Polígono 1
    let points = vec![
        new_vertex(165.0, 380.0, 0.0),
        new_vertex(185.0, 360.0, 0.0),
        new_vertex(180.0, 330.0, 0.0),
        new_vertex(207.0, 345.0, 0.0),
        new_vertex(233.0, 330.0, 0.0),
        new_vertex(230.0, 360.0, 0.0),
        new_vertex(250.0, 380.0, 0.0),
        new_vertex(220.0, 385.0, 0.0),
        new_vertex(205.0, 410.0, 0.0),
        new_vertex(193.0, 383.0, 0.0),
    ];

    framebuffer.set_current_color(0xFFFF00); // Amarillo
    fill_polygon(&mut framebuffer, &points);

    framebuffer.set_current_color(0xFFFFFF); // Blanco
    draw_polygon(&mut framebuffer, &points);

    // Polígono 2
    let points2 = vec![
    new_vertex(321.0, 335.0, 0.0),
    new_vertex(288.0, 286.0, 0.0),
    new_vertex(339.0, 251.0, 0.0),
    new_vertex(374.0, 302.0, 0.0),
    ];

    framebuffer.set_current_color(0x0000FF); // Azul
    fill_polygon(&mut framebuffer, &points2);

    framebuffer.set_current_color(0xFFFFFF); // Blanco
    draw_polygon(&mut framebuffer, &points2);

    // Polígono 3
    let points3 = vec![
        new_vertex(377.0, 249.0, 0.0),
        new_vertex(411.0, 197.0, 0.0),
        new_vertex(436.0, 249.0, 0.0),
    ];

    framebuffer.set_current_color(0xFF0000); // Rojo
    fill_polygon(&mut framebuffer, &points3);

    framebuffer.set_current_color(0xFFFFFF); // Blanco
    draw_polygon(&mut framebuffer, &points3);

    // Polígono 4
    let points4 = vec![
        new_vertex(413.0, 177.0, 0.0),
        new_vertex(448.0, 159.0, 0.0),
        new_vertex(502.0, 88.0, 0.0),
        new_vertex(553.0, 53.0, 0.0),
        new_vertex(535.0, 36.0, 0.0),
        new_vertex(676.0, 37.0, 0.0),
        new_vertex(660.0, 52.0, 0.0),
        new_vertex(750.0, 145.0, 0.0),
        new_vertex(761.0, 179.0, 0.0),
        new_vertex(672.0, 192.0, 0.0),
        new_vertex(659.0, 214.0, 0.0),
        new_vertex(615.0, 214.0, 0.0),
        new_vertex(632.0, 230.0, 0.0),
        new_vertex(580.0, 230.0, 0.0),
        new_vertex(597.0, 215.0, 0.0),
        new_vertex(552.0, 214.0, 0.0),
        new_vertex(517.0, 144.0, 0.0),
        new_vertex(466.0, 180.0, 0.0),
    ];

    framebuffer.set_current_color(0x00FF00); // Verde
    fill_polygon(&mut framebuffer, &points4);

    framebuffer.set_current_color(0xFFFFFF); // Blanco
    draw_polygon(&mut framebuffer, &points4);

    // Polígono 5 (agujero)
    let points5 = vec![
        new_vertex(682.0, 175.0, 0.0),
        new_vertex(708.0, 120.0, 0.0),
        new_vertex(735.0, 148.0, 0.0),
        new_vertex(739.0, 170.0, 0.0),
    ];

    framebuffer.set_current_color(0x000000); // Negro para agujero
    fill_polygon(&mut framebuffer, &points5);

    framebuffer.set_current_color(0xFFFFFF); // Blanco
    draw_polygon(&mut framebuffer, &points5);

    framebuffer.render_buffer("out.bmp").unwrap();

    println!("Framebuffer rendered to out.bmp");
}

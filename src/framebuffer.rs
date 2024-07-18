use crate::bmp;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    // Constructor para inicializar el framebuffer
    pub fn new(width: usize, height: usize) -> Self {
        let background_color = 0x000000; // Negro por defecto
        let current_color = 0xFFFFFF; // Blanco por defecto
        let buffer = vec![background_color; width * height];
        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            current_color,
        }
    }

    // Método para limpiar el framebuffer con el color de fondo
    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    // Método para dibujar un punto en (x, y) usando el color actual
    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = self.current_color;
        }
    }

    // Método para establecer el color de fondo
    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
        self.clear();
    }

    // Método para establecer el color actual
    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    // Método para obtener el valor en una posición (x, y)
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            Some(self.buffer[y * self.width + x])
        } else {
            None
        }
    }

    // Método para mostrar el contenido del framebuffer (para depuración)
    pub fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.buffer[y * self.width + x];
                print!("{:#X} ", color);
            }
            println!();
        }
    }

    // Método para escribir el framebuffer a un archivo BMP
    pub fn write_to_file(&self, file_path: &str) -> std::io::Result<()> {
        bmp::write_bmp_file(file_path, &self.buffer, self.width, self.height)
    }

    // Método para renderizar el framebuffer a un archivo BMP
    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        self.write_to_file(file_path)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    // Constructor para inicializar con valores RGB
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    // Constructor para inicializar con un valor hexadecimal
    pub fn from_hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color { r, g, b }
    }

    // Método para convertir a hexadecimal
    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    // Método para mezclar colores
    pub fn mix(&self, other: &Color) -> Color {
        Color {
            r: (self.r as u16 + other.r as u16 / 2) as u8,
            g: (self.g as u16 + other.g as u16 / 2) as u8,
            b: (self.b as u16 + other.b as u16 / 2) as u8,
        }
    }
}

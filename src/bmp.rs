use std::fs::File;
use std::io::{Write, BufWriter, Result};

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 24;

pub fn write_bmp_file(
    file_path: &str,      // Path to the output BMP file
    buffer: &[u32],       // Framebuffer pixel data
    width: usize,         // Width of the image
    height: usize         // Height of the image
) -> Result<()> {
    // Create a buffered writer for the file
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    // Write the BMP header
    write_bmp_header(&mut writer, width, height)?;

    // Write the pixel data from the framebuffer
    write_pixel_data(&mut writer, buffer, width, height)?;

    Ok(())
}

fn write_bmp_header(
    file: &mut BufWriter<File>,  // Buffered writer for the file
    width: usize,                // Width of the image
    height: usize                // Height of the image
) -> Result<()> {
    let file_size = BMP_HEADER_SIZE + (width * height * 3);
    let reserved: u32 = 0;
    let offset = BMP_PIXEL_OFFSET as u32;

    let header = [
        // BMP signature
        b'B', b'M', 

        // File size
        (file_size & 0xFF) as u8, 
        ((file_size >> 8) & 0xFF) as u8, 
        ((file_size >> 16) & 0xFF) as u8, 
        ((file_size >> 24) & 0xFF) as u8, 

        // Reserved
        (reserved & 0xFF) as u8, 
        ((reserved >> 8) & 0xFF) as u8, 
        (reserved & 0xFF) as u8, 
        ((reserved >> 8) & 0xFF) as u8, 

        // Pixel data offset
        (offset & 0xFF) as u8, 
        ((offset >> 8) & 0xFF) as u8, 
        ((offset >> 16) & 0xFF) as u8, 
        ((offset >> 24) & 0xFF) as u8, 

        // DIB header size
        40, 0, 0, 0, 

        // Width
        (width & 0xFF) as u8, 
        ((width >> 8) & 0xFF) as u8, 
        ((width >> 16) & 0xFF) as u8, 
        ((width >> 24) & 0xFF) as u8, 

        // Height
        (height & 0xFF) as u8, 
        ((height >> 8) & 0xFF) as u8, 
        ((height >> 16) & 0xFF) as u8, 
        ((height >> 24) & 0xFF) as u8, 

        // Color planes
        1, 0, 

        // Bits per pixel
        BMP_BITS_PER_PIXEL as u8, 0, 

        // Compression
        0, 0, 0, 0, 

        // Image size (can be 0 for no compression)
        0, 0, 0, 0, 

        // X pixels per meter (can be 0)
        0, 0, 0, 0, 

        // Y pixels per meter (can be 0)
        0, 0, 0, 0, 

        // Total colors (0 means default 2^n)
        0, 0, 0, 0, 

        // Important colors (0 means all)
        0, 0, 0, 0,
    ];

    file.write_all(&header)?;
    Ok(())
}

fn write_pixel_data(
    file: &mut BufWriter<File>, // Buffered writer for the file
    buffer: &[u32],             // Framebuffer pixel data
    width: usize,               // Width of the image
    height: usize               // Height of the image
) -> Result<()> {
    let padding = (4 - (width * 3) % 4) % 4;

    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = buffer[y * width + x];
            let blue = (pixel & 0xFF) as u8;
            let green = ((pixel >> 8) & 0xFF) as u8;
            let red = ((pixel >> 16) & 0xFF) as u8;
            file.write_all(&[blue, green, red])?;
        }
        for _ in 0..padding {
            file.write_all(&[0])?;
        }
    }
    Ok(())
}

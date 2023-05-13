use std::{
    cell::RefCell,
    fs::File,
    io::BufWriter,
    num::NonZeroU32,
    path::{Path, PathBuf},
    rc::Rc,
};

use richter::client::render::Extent2d;

use chrono::Utc;

const BYTES_PER_PIXEL: u32 = 4;

/// Implements the "screenshot" command.
///
/// This function returns a boxed closure which sets the `screenshot_path`
/// argument to `Some` when called.
pub fn cmd_screenshot(
    screenshot_path: Rc<RefCell<Option<PathBuf>>>,
) -> Box<dyn Fn(&[&str]) -> String> {
    Box::new(move |args| {
        let path = match args.len() {
            // TODO: make default path configurable
            0 => PathBuf::from(format!("richter-{}.png", Utc::now().format("%FT%H-%M-%S"))),
            1 => PathBuf::from(args[0]),
            _ => {
                log::error!("Usage: screenshot [PATH]");
                return "Usage: screenshot [PATH]".to_owned();
            }
        };

        screenshot_path.replace(Some(path));
        String::new()
    })
}

pub struct Capture {
    // size of the capture image
    capture_size: Extent2d,

    // width of a row in the buffer, must be a multiple of 256 for mapped reads
    row_width: u32,

    // mappable buffer
    buffer: wgpu::Buffer,
}

impl Capture {
    pub fn new(device: &wgpu::Device, capture_size: Extent2d) -> Capture {
        // bytes_per_row must be a multiple of 256
        // 4 bytes per pixel, so width must be multiple of 64
        let row_width = (capture_size.width + 63) / 64 * 64;

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("capture buffer"),
            size: (row_width * capture_size.height * BYTES_PER_PIXEL) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        Capture {
            capture_size,
            row_width,
            buffer,
        }
    }

    pub fn copy_from_texture(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        texture: wgpu::ImageCopyTexture,
    ) {
        encoder.copy_texture_to_buffer(
            texture,
            wgpu::ImageCopyBuffer {
                buffer: &self.buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.row_width * BYTES_PER_PIXEL),
                    rows_per_image: None,
                },
            },
            self.capture_size.into(),
        );
    }

    pub fn write_to_file<P>(&self, device: &wgpu::Device, path: P)
    where
        P: AsRef<Path>,
    {
        let mut data = Vec::new();
        {
            // map the buffer
            // TODO: maybe make this async so we don't force the whole program to block
            let slice = self.buffer.slice(..);
            slice.map_async(wgpu::MapMode::Read, |_| ());
            device.poll(wgpu::Maintain::Wait);

            // copy pixel data
            let mapped = slice.get_mapped_range();
            for row in mapped.chunks(self.row_width as usize * BYTES_PER_PIXEL as usize) {
                // don't copy padding
                for pixel in
                    (&row[..self.capture_size.width as usize * BYTES_PER_PIXEL as usize]).chunks(4)
                {
                    // swap BGRA->RGBA
                    data.extend_from_slice(&[pixel[2], pixel[1], pixel[0], pixel[3]]);
                }
            }
        }
        self.buffer.unmap();

        let f = File::create(path).unwrap();
        let mut png_encoder = png::Encoder::new(
            BufWriter::new(f),
            self.capture_size.width,
            self.capture_size.height,
        );
        png_encoder.set_color(png::ColorType::Rgba);
        png_encoder.set_depth(png::BitDepth::Eight);
        let mut writer = png_encoder.write_header().unwrap();
        writer.write_image_data(&data).unwrap();
    }
}

use anyhow::Result;
use gtk::cairo::{Context, Format, ImageSurface};
use image::GenericImageView;
use std::{collections::HashMap, f64};

pub struct BrushDefinition {
    name: String,
    texture: ImageSurface,
    cache: HashMap<(i32, i32), ImageSurface>,
}

impl BrushDefinition {
    pub fn from_bytes(name: &str, bytes: &[u8]) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(name, img)
    }

    fn from_image(name: &str, img: image::DynamicImage) -> Result<BrushDefinition> {
        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();
        let stride = Format::ARgb32.stride_for_width(width)?;
        let mut buff = vec![0u8; (stride as usize) * (height as usize)];
        let src = rgba.as_raw();

        for y in 0..(height as usize) {
            for x in 0..(width as usize) {
                let src_idx = (y * (width as usize) + x) * 4;
                let r = src[src_idx] as f32;
                let g = src[src_idx + 1] as f32;
                let b = src[src_idx + 2] as f32;
                let a = src[src_idx + 3] as f32;

                let af = a / 255.0;
                let r_p = (r * af).round() as u8;
                let g_p = (g * af).round() as u8;
                let b_p = (b * af).round() as u8;
                let a_u8 = a as u8;

                let row_start = y * (stride as usize);
                let dst = row_start + x * 4;

                // ordem BGRA na memória (little-endian)
                buff[dst + 0] = b_p;
                buff[dst + 1] = g_p;
                buff[dst + 2] = r_p;
                buff[dst + 3] = a_u8;
            }
        }
        let surface = ImageSurface::create_for_data(
            buff,
            Format::ARgb32,
            width as i32,
            height as i32,
            stride,
        )?;

        Ok(BrushDefinition {
            name: name.into(),
            texture: surface,
            cache: HashMap::new(),
        })
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_cached(&mut self, thickness: f64, angle_deg: i32) -> &ImageSurface {
        let key = (thickness.round() as i32, angle_deg);
        if !self.cache.contains_key(&key) {
            if let Ok(surface) = self.generate_variant(thickness, angle_deg) {
                self.cache.insert(key, surface);
            }
        }

        self.cache.get(&key).unwrap()
    }

    fn generate_variant(&self, thickness: f64, angle_deg: i32) -> Result<ImageSurface> {
        let width = self.texture.width() as f64;
        let height = self.texture.height() as f64;
        let scale = thickness / width.max(height).max(1.0);

        let scaled_width = (width * scale).ceil() as i32;
        let scaled_height = (height * scale).ceil() as i32;

        let diag = ((scaled_width * scaled_width + scaled_height * scaled_height) as f64).sqrt();
        let size = (diag.ceil() as i32).max(1) + 4;

        let surface = ImageSurface::create(Format::ARgb32, size, size)?;
        let ctx = Context::new(&surface)?;
        ctx.translate((size / 2) as f64, (size / 2) as f64);
        ctx.rotate((angle_deg as f64) * f64::consts::PI / 180.);
        ctx.scale(scale, scale);

        let src_x = -(width / 2.0);
        let src_y = -(height / 2.0);
        ctx.set_source_surface(&self.texture, src_x, src_y)?;
        ctx.paint()?;

        Ok(surface)
    }
}

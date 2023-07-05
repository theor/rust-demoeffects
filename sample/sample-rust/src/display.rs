use std::marker::PhantomData;

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, OriginDimensions, PixelColor, Point, Size, RgbColor},
    primitives::Rectangle,
    Pixel,
};

/// Output settings.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OutputSettings {
    /// Pixel scale.
    pub scale: u32,
    /// Spacing between pixels.
    pub pixel_spacing: u32,
}

impl Default for OutputSettings {
    fn default() -> Self {
        Self {
            scale: 1,
            pixel_spacing: 0,
        }
    }
}

impl OutputSettings {
    /// Calculates the size of the framebuffer required to display the scaled display.
    pub(crate) fn framebuffer_size(&self, size: Size) -> Size {
        let width = size.width;
        let height = size.height;
        let output_width = width * self.scale + width.saturating_sub(1) * self.pixel_spacing;
        let output_height = height * self.scale + height.saturating_sub(1) * self.pixel_spacing;

        Size::new(output_width, output_height)
    }
}

struct Display<C> {
    backing: Vec<u8>,
    size: Size,
    canvas_size: Size,
    output_settings: OutputSettings,
    _color_type: PhantomData<C>,
}

impl<C> Display<C>
where
    C: PixelColor + Into<Rgb888>,
{
    pub fn new(size: Size, backing: Vec<u8>) -> Self {
        Self {
            backing: backing,
            size: size,
            canvas_size: size,
            output_settings: Default::default(),
            _color_type: PhantomData::default(),
        }
    }
    pub fn flush(&mut self) {
        // let backing = self.backing.as_mut_slice();
        // let image_data =
        //     ImageData::new_with_u8_clamped_array(Clamped(backing), self.canvas_size.width)
        //         .expect("could not create ImageData");
        // self.context.put_image_data(&image_data, 0., 0.)?;
        // Ok(())
    }
}

impl<C> OriginDimensions for Display<C>
where
    C: PixelColor + Into<Rgb888>,
{
    fn size(&self) -> Size {
        self.size
    }
}

impl<C> DrawTarget for Display<C>
where
    C: PixelColor + Into<Rgb888>,
{
    type Color = C;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let canvas_width = self.canvas_size.width as usize;
        let backing = self.backing.as_mut_slice();

        let scale = self.output_settings.scale as usize;

        // source: https://github.com/embedded-graphics/simulator/blob/master/src/output_settings.rs#L39
        let pitch = scale + self.output_settings.pixel_spacing as usize;

        let bounding_box = Rectangle::new(Point::new(0, 0), self.size);
        for pixel in pixels.into_iter() {
            let point = pixel.0;
            if bounding_box.contains(point) {
                let rgb: Rgb888 = pixel.1.into();
                let rgb_slice = &[rgb.r(), rgb.g(), rgb.b(), 255];
                let py = point.y as usize;
                let px = point.x as usize;

                let x_offset = px * 4 * pitch;
                for y in 0..scale {
                    let y_offset = py * 4 * canvas_width * pitch + y * 4 * canvas_width;
                    for x in 0..scale {
                        let pixel_offset = y_offset + x_offset + x * 4;
                        backing[pixel_offset..pixel_offset + 4].copy_from_slice(rgb_slice);
                    }
                }
            }
        }

        Ok(())
    }
}

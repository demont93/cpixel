use std::iter::Sum;

use crate::{BitmapImage, Cpixel, Dimensions};
use crate::cpixel::CpixelConverter;

trait Brightness {
    fn min() -> Self;
    fn max() -> Self;
    fn average(&self, rhs: &Self) -> Self;
}

impl Brightness for u8 {
    fn min() -> Self {
        u8::MIN
    }

    fn max() -> Self {
        u8::MAX
    }

    fn average(&self, rhs: &Self) -> Self {
        (self as u16 + rhs as u16 / 2) as u8
    }
}

pub struct Converter<T> {
    converter: CpixelConverter<T>,
    cpixel_dimensions: Dimensions,
    output_constraints: Dimensions,
    input_image_dimensions: Dimensions,
    output_dimensions: Dimensions,
    maximize_contrast: bool
}

impl<PixelType> Converter<PixelType> {
    pub fn new(
        output_constraints: &Dimensions,
        input_image_dimensions: &Dimensions,
        cpixel_dimensions: &Dimensions,
        maximize_contrast: bool
    ) -> Self {
        Self {
            converter: Default::default(),
            cpixel_dimensions: *cpixel_dimensions,
            output_constraints: *output_constraints,
            input_image_dimensions: *input_image_dimensions,
            output_dimensions: Self::generate_output_dimensions(
                input_image_dimensions,
                output_constraints,
                cpixel_dimensions
            ),
            maximize_contrast
        }
    }
    pub fn maximizing_contrast_on(&self) -> bool {
        self.maximize_contrast
    }

    pub fn constraints(&self) -> &Dimensions {
        &self.output_constraints
    }

    pub fn image_settings(&self) -> &Dimensions {
        &self.input_image_dimensions
    }

    pub fn cpixel_dimensions_settings(&self) -> &Dimensions {
        &self.cpixel_dimensions
    }

    pub fn output_dimensions(&self) -> &Dimensions {
        &self.output_dimensions
    }

    pub fn with_settings(
        self,
        output_constraints: &Dimensions,
        input_image_dimensions: &Dimensions,
        cpixel_dimensions: &Dimensions,
    ) -> Self {
        Converter {
            converter: self.converter,
            output_constraints: *output_constraints,
            input_image_dimensions: *input_image_dimensions,
            cpixel_dimensions: *cpixel_dimensions,
            output_dimensions: Self::generate_output_dimensions(
                input_image_dimensions,
                output_constraints,
                cpixel_dimensions,
            ),
            maximize_contrast: self.maximize_contrast
        }
    }

    fn generate_output_dimensions(
        image_dimensions: &Dimensions,
        output_constraints: &Dimensions,
        cpixel_dimensions: &Dimensions,
    ) -> Dimensions {
        let screen_in_pixels = Dimensions {
            height: output_constraints.height * cpixel_dimensions.height,
            width: output_constraints.width * cpixel_dimensions.width,
        };
        Dimensions::fit_with_locked_ratio(image_dimensions, &screen_in_pixels)
    }
}

impl<T: Into<u8> + Default + Copy + Sum + PartialOrd + From<u8>>
Converter<T> {
    pub fn convert_one(&mut self, image: &BitmapImage<T>) -> BitmapImage<Cpixel> {
        self.converter.convert_one(
            &image.resize(&self.output_dimensions),
            &self.cpixel_dimensions,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::bitmap_image::BitmapImage;
    use crate::converter::Converter;
    use crate::cpixel::Cpixel;
    use crate::dimensions::Dimensions;

    #[test]
    fn test_can_instance_converter() {
        let input_image_dimensions = Dimensions { height: 1, width: 1 };
        let output_constraints = Dimensions { height: 1, width: 1 };
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        Converter::<u8>::new(
            &output_constraints,
            &input_image_dimensions,
            &cpixel_dimensions,
            false,
        );
    }

    #[test]
    fn test_singleton_pixel_min() {
        let input_image_dimensions = Dimensions { height: 1, width: 1 };
        let output_constraints = Dimensions { height: 1, width: 1 };
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        let mut converter = Converter::<u8>::new(
            &output_constraints,
            &input_image_dimensions,
            &cpixel_dimensions,
            false,
        );
        let image = BitmapImage::new(input_image_dimensions, vec![0_u8]);
        let cpixel_image = converter.convert_one(&image);
        assert_eq!(cpixel_image.buffer, vec![Cpixel(' ')]);
    }

    #[test]
    fn test_singleton_pixel_max() {
        let input_image_dimensions = Dimensions { height: 1, width: 1 };
        let output_constraints = Dimensions { height: 1, width: 1 };
        let cpixel_dimensions = Dimensions { height: 1, width: 1 };
        let mut converter: Converter<u8> = Converter::new(
            &output_constraints,
            &input_image_dimensions,
            &cpixel_dimensions,
            false,
        );
        let image = BitmapImage::new(input_image_dimensions, vec![255_u8]);
        let cpixel_image = converter.convert_one(&image);
        assert_eq!(cpixel_image.buffer, vec![Cpixel('N')]);
    }
}

use crate::Color32;

/// An image stored in RAM.
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ImageData {
    /// RGBA image.
    Color(ColorImage),
    /// Used for the font texture.
    Alpha(AlphaImage),
}

// ----------------------------------------------------------------------------

/// A 2D RGBA color image in RAM.
#[derive(Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ColorImage {
    /// width, height
    pub size: [usize; 2],
    /// The pixels, row by row, from top to bottom.
    pub pixels: Vec<Color32>,
}

impl ColorImage {
    pub fn new(size: [usize; 2], color: Color32) -> Self {
        Self {
            size,
            pixels: vec![color; size[0] * size[1]],
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.size[0]
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.size[1]
    }

    /// Create an `Image` from flat RGBA data.
    /// Panics unless `size[0] * size[1] * 4 == rgba.len()`.
    /// This is usually what you want to use after having loaded an image.
    pub fn from_rgba_unmultiplied(size: [usize; 2], rgba: &[u8]) -> Self {
        assert_eq!(size[0] * size[1] * 4, rgba.len());
        let pixels = rgba
            .chunks_exact(4)
            .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();
        Self { size, pixels }
    }
}

impl From<ColorImage> for ImageData {
    #[inline(always)]
    fn from(image: ColorImage) -> Self {
        Self::Color(image)
    }
}

// ----------------------------------------------------------------------------

/// An 8-bit image, representing difference levels of transparent white.
///
/// Used for the font texture
#[derive(Clone, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AlphaImage {
    /// width, height
    pub size: [usize; 2],
    /// The alpha (linear space 0-255) of something white.
    ///
    /// One byte per pixel. Often you want to use [`Self::srgba_pixels`] instead.
    pub pixels: Vec<u8>,
}

impl AlphaImage {
    pub fn new(size: [usize; 2]) -> Self {
        Self {
            size,
            pixels: vec![0; size[0] * size[1]],
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.size[0]
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.size[1]
    }

    /// Returns the textures as `sRGBA` premultiplied pixels, row by row, top to bottom.
    ///
    /// `gamma` should normally be set to 1.0.
    /// If you are having problems with text looking skinny and pixelated, try
    /// setting a lower gamma, e.g. `0.5`.
    pub fn srgba_pixels(&'_ self, gamma: f32) -> impl Iterator<Item = super::Color32> + '_ {
        let srgba_from_alpha_lut: Vec<Color32> = (0..=255)
            .map(|a| {
                let a = super::color::linear_f32_from_linear_u8(a).powf(gamma);
                super::Rgba::from_white_alpha(a).into()
            })
            .collect();

        self.pixels
            .iter()
            .map(move |&a| srgba_from_alpha_lut[a as usize])
    }
}

impl std::ops::Index<(usize, usize)> for AlphaImage {
    type Output = u8;

    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &u8 {
        let [w, h] = self.size;
        assert!(x < w && y < h);
        &self.pixels[y * w + x]
    }
}

impl std::ops::IndexMut<(usize, usize)> for AlphaImage {
    #[inline]
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut u8 {
        let [w, h] = self.size;
        assert!(x < w && y < h);
        &mut self.pixels[y * w + x]
    }
}

impl From<AlphaImage> for ImageData {
    #[inline(always)]
    fn from(image: AlphaImage) -> Self {
        Self::Alpha(image)
    }
}

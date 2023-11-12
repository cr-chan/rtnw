use stb_image::image;

pub const BYTES_PER_PIXEL: usize = 3;
static MAGENTA: [u8; BYTES_PER_PIXEL] = [255, 0, 255];

#[derive(Default, Clone)]
pub struct RtwImage {
    data: Vec<u8>,
    image_width: usize,
    image_height: usize,
    bytes_per_scanline: usize,
}

impl RtwImage {
    pub fn new(image_filename: &str) -> Self {
        let filename = image_filename;
        let imagedir = std::env::var("RTW_IMAGES").unwrap_or_else(|_| String::from("images"));

        let mut _self = Self::default();
        if let Some(result) = _self.load(&format!("{}/{}", imagedir, filename)) {
            return result;
        }

        let mut path = String::from(filename);
        for _ in 0..6 {
            if let Some(result) = _self.load(&path) {
                return result;
            }
            path = format!("../{}", path);
        }

        panic!("ERROR: Could not load image file \"{}\".", filename);
    }

    pub fn load(&mut self, filename: &str) -> Option<RtwImage> {
        let load_result = image::load_with_depth(filename, BYTES_PER_PIXEL, false);
        match load_result {
            image::LoadResult::Error(e) => {
                eprintln!("Failed to load image: {}", e);
                None
            }

            image::LoadResult::ImageF32(_) => {
                eprintln!("Unsupported image format: ImageF32");
                None
            }

            image::LoadResult::ImageU8(image) => {
                assert_eq!(image.depth, BYTES_PER_PIXEL);
                let data = image.data;
                let bytes_per_scanline = image.depth * image.width;
                Some(RtwImage {
                    data,
                    image_width: image.width,
                    image_height: image.height,
                    bytes_per_scanline,
                })
            }
        }
    }

    pub fn width(&self) -> usize {
        self.image_width
    }

    pub fn height(&self) -> usize {
        self.image_height
    }

    pub fn pixel_data(&self, x: usize, y: usize) -> &[u8] {
        if self.data.is_empty() {
            &MAGENTA
        } else {
            let x = Self::clamp(x, 0, self.image_width);
            let y = Self::clamp(y, 0, self.image_height);
            let start = (y * self.bytes_per_scanline) + (x * BYTES_PER_PIXEL);
            let end = start + BYTES_PER_PIXEL;

            &self.data[start..end]
        }
    }

    fn clamp(x: usize, low: usize, high: usize) -> usize {
        std::cmp::max(low, std::cmp::min(x, high - 1))
    }
}

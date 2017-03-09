mod ffi;

#[derive(Debug)]
pub struct Error {
    e: ffi::dc1394error_t,
    descr: String,
}

impl Error {
    pub fn new(e: ffi::dc1394error_t) -> Error {
        Error {
            e: e,
            descr: format!("{:?}",e),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.descr
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self,f)
    }
}

macro_rules! dc1394_try {
    ($x:expr) => {
        match unsafe { $x } {
            ffi::dc1394error_t::DC1394_SUCCESS => {},
            e => {return Err(Error::new(e));},
        }
    }
}

pub enum Tile {
    RGGB,
    GBRG,
    GRBG,
    BGGR,
}

impl From<Tile> for ffi::dc1394color_filter_t {
    fn from(t: Tile) -> Self {
        match t {
            Tile::RGGB => ffi::dc1394color_filter_t::DC1394_COLOR_FILTER_RGGB,
            Tile::GBRG => ffi::dc1394color_filter_t::DC1394_COLOR_FILTER_GBRG,
            Tile::GRBG => ffi::dc1394color_filter_t::DC1394_COLOR_FILTER_GRBG,
            Tile::BGGR => ffi::dc1394color_filter_t::DC1394_COLOR_FILTER_BGGR,
        }
    }
}

pub enum Method {
    Nearest,
    Simple,
    Bilinear,
    HQLinear,
    Downsample,
    EdgeSense,
    VNG,
    AHD,
}

impl From<Method> for ffi::dc1394bayer_method_t {
    fn from(m: Method) -> Self {
        match m {
            Method::Nearest => ffi::dc1394bayer_method_t::DC1394_BAYER_METHOD_NEAREST,
            Method::Simple => ffi::dc1394bayer_method_t::DC1394_BAYER_METHOD_SIMPLE,
            Method::Bilinear => ffi::dc1394bayer_method_t::DC1394_BAYER_METHOD_BILINEAR,
            Method::HQLinear => ffi::dc1394bayer_method_t::DC1394_BAYER_METHOD_HQLINEAR,
            Method::Downsample => ffi::dc1394bayer_method_t::DC1394_BAYER_METHOD_DOWNSAMPLE,
            Method::EdgeSense => ffi::dc1394bayer_method_t::DC1394_BAYER_METHOD_EDGESENSE,
            Method::VNG => ffi::dc1394bayer_method_t::DC1394_BAYER_METHOD_VNG,
            Method::AHD => ffi::dc1394bayer_method_t::DC1394_BAYER_METHOD_AHD,
        }
    }
}

pub fn decode(bayer: &[u8], width: u32, height: u32, tile: Tile, method: Method) -> Result<Vec<u8>,Error> {
    let num_bytes = width*height*3;
    let mut rgb: Vec<u8> = vec![0; num_bytes as usize];
    dc1394_try!(ffi::dc1394_bayer_decoding_8bit(
        bayer.as_ptr(),
        rgb.as_mut_ptr(),
        width,
        height,
        tile.into(),
        method.into()));
    Ok(rgb)
}

#[test]
fn test_decode_runs() {
    let w = 10;
    let h = 10;
    let mut buf = Vec::new();
    for _x in 0..w {
        for _y in 0..h {
            buf.push(0);
        }
    }
    decode(&buf, 10, 10, Tile::BGGR, Method::Bilinear).unwrap();
}

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum dc1394color_filter_t {
    DC1394_COLOR_FILTER_RGGB = 512,
    DC1394_COLOR_FILTER_GBRG = 513,
    DC1394_COLOR_FILTER_GRBG = 514,
    DC1394_COLOR_FILTER_BGGR = 515,
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum dc1394bayer_method_t {
    DC1394_BAYER_METHOD_NEAREST = 0,
    DC1394_BAYER_METHOD_SIMPLE = 1,
    DC1394_BAYER_METHOD_BILINEAR = 2,
    DC1394_BAYER_METHOD_HQLINEAR = 3,
    DC1394_BAYER_METHOD_DOWNSAMPLE = 4,
    DC1394_BAYER_METHOD_EDGESENSE = 5,
    DC1394_BAYER_METHOD_VNG = 6,
    DC1394_BAYER_METHOD_AHD = 7,
}

#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum dc1394error_t {
    DC1394_SUCCESS = 0,
    DC1394_FAILURE = -1,
    DC1394_NOT_A_CAMERA = -2,
    DC1394_FUNCTION_NOT_SUPPORTED = -3,
    DC1394_CAMERA_NOT_INITIALIZED = -4,
    DC1394_MEMORY_ALLOCATION_FAILURE = -5,
    DC1394_TAGGED_REGISTER_NOT_FOUND = -6,
    DC1394_NO_ISO_CHANNEL = -7,
    DC1394_NO_BANDWIDTH = -8,
    DC1394_IOCTL_FAILURE = -9,
    DC1394_CAPTURE_IS_NOT_SET = -10,
    DC1394_CAPTURE_IS_RUNNING = -11,
    DC1394_RAW1394_FAILURE = -12,
    DC1394_FORMAT7_ERROR_FLAG_1 = -13,
    DC1394_FORMAT7_ERROR_FLAG_2 = -14,
    DC1394_INVALID_ARGUMENT_VALUE = -15,
    DC1394_REQ_VALUE_OUTSIDE_RANGE = -16,
    DC1394_INVALID_FEATURE = -17,
    DC1394_INVALID_VIDEO_FORMAT = -18,
    DC1394_INVALID_VIDEO_MODE = -19,
    DC1394_INVALID_FRAMERATE = -20,
    DC1394_INVALID_TRIGGER_MODE = -21,
    DC1394_INVALID_TRIGGER_SOURCE = -22,
    DC1394_INVALID_ISO_SPEED = -23,
    DC1394_INVALID_IIDC_VERSION = -24,
    DC1394_INVALID_COLOR_CODING = -25,
    DC1394_INVALID_COLOR_FILTER = -26,
    DC1394_INVALID_CAPTURE_POLICY = -27,
    DC1394_INVALID_ERROR_CODE = -28,
    DC1394_INVALID_BAYER_METHOD = -29,
    DC1394_INVALID_VIDEO1394_DEVICE = -30,
    DC1394_INVALID_OPERATION_MODE = -31,
    DC1394_INVALID_TRIGGER_POLARITY = -32,
    DC1394_INVALID_FEATURE_MODE = -33,
    DC1394_INVALID_LOG_TYPE = -34,
    DC1394_INVALID_BYTE_ORDER = -35,
    DC1394_INVALID_STEREO_METHOD = -36,
    DC1394_BASLER_NO_MORE_SFF_CHUNKS = -37,
    DC1394_BASLER_CORRUPTED_SFF_CHUNK = -38,
    DC1394_BASLER_UNKNOWN_SFF_CHUNK = -39,
}

extern "C" {
    pub fn dc1394_bayer_decoding_8bit(bayer: *const uint8_t,
                                      rgb: *mut uint8_t, width: uint32_t,
                                      height: uint32_t,
                                      tile: dc1394color_filter_t,
                                      method: dc1394bayer_method_t)
     -> dc1394error_t;
}

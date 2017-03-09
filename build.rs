extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("libdc1394/dc1394/bayer.c")
        .include("libdc1394")
        .compile("libdc1394-debayer.a");
}

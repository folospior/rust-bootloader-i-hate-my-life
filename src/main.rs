#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;
use tinybmp::Bmp;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltRegion};
use uefi::proto::console::gop::{BltPixel, GraphicsOutput};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    uefi::println!("init");
    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();
    uefi::println!("gop handle");
    let mut gop = boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();
    uefi::println!("gop opened");

    draw_avatar(gop.get_mut().unwrap());
    uefi::println!("avatar drawn");
    boot::stall(10_000_000);
    Status::SUCCESS
}

fn draw_avatar(gop: &mut GraphicsOutput) {
    let bmp_data = include_bytes!("resources/avatar.bmp");
    uefi::println!("avatar bytes included");
    let bmp = Bmp::<Rgb888>::from_slice(bmp_data).unwrap();
    uefi::println!("bmp object creacted");

    uefi::println!(
        "x: {}, y: {}",
        gop.current_mode_info().resolution().0,
        gop.current_mode_info().resolution().1
    );

    let width: usize = bmp.as_raw().header().image_size.width.try_into().unwrap();
    uefi::println!("width gotten");
    let height: usize = bmp.as_raw().header().image_size.height.try_into().unwrap();
    uefi::println!("height gotten");

    let mut buf = Vec::with_capacity(width * height);
    uefi::println!("buffer created");
    let mut count = 0;
    for pixel in bmp.pixels() {
        let color = pixel.1;
        buf.push(BltPixel::new(color.r(), color.g(), color.b()));
        count += 1;
        uefi::println!("pixel {} loaded into buffer", count);
    }

    uefi::println!("buffer populated");

    loop {
        gop.blt(BltOp::BufferToVideo {
            buffer: &buf,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (width, height),
        })
        .expect("Failed to draw BMP");
    }
}

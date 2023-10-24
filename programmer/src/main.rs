extern crate image;

use std::env;
use image::{imageops::*, Pixel};
use std::{thread, time};
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let mut args = env::args();
    args.next(); // skip command name

    let img_path = match args.next() {
        None => {
            eprintln!("Error: Input file path is not specified. ");
            eprintln!("Usage: cargo run /path/to/input/image");
            return;
        },
        Some(s) => s,
    };

    // Load Image
    let img = image::open(&img_path).unwrap();

    let img = img.resize(100,  70, FilterType::Nearest);
    let mut img = img.grayscale();
    let mut img = img.as_mut_luma8().unwrap();
    // dither(&mut img, &BiLevel);

    // let _gray = img.resize_exact(128, 128, imageops::FilterType::Nearest).into_luma8();
    // img.save("resize100x70.png").unwrap();

    // let img = img.to_rgb(); // derive RGB Image

    // println!("Name: {}", img_path.rsplit("/").next().unwrap());
    // println!("Size: {}x{}", img.width(), img.height());

    let (width, height) = img.dimensions();
    println!("{width},{height}");

    // for pixel in img.pixels() {
    //     // println!("{},{:?}",i, pixel.to_luma());
    // }


    let ten_millis = time::Duration::from_millis(130);
    let ch = vec!["*","%","$","@","#","&"];
    for i in 6..height-3{
        for j in 0..width{
            let pixel = img[(j, i)];
            let value = pixel.to_luma().0[0];
            let mut rng = rand::thread_rng();
            if value < 130{
                let s:&str = ch[rng.gen_range(0..5)];
                print!("{}", &s);
            } else{
                print!(" ");
            }
            // println!("({i},{j},value={value})");
        }
        println!("");
        thread::sleep(ten_millis);
    }
}

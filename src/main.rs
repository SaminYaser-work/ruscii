use image::GenericImageView;
use std::{env, process::exit};

fn get_str_ascii(intesity: u8) -> &'static str {
    let index = intesity / 16;
    let ascii = [
        " ", ".", ",", "-", ":", ";", "+", "v", "=", "o", "x", "O", "X", "%", "@", "#",
    ];
    ascii[index as usize]
}

fn get_image(dir: &str, scale: u32) {
    let img_res = image::open(dir);

    let img = match img_res {
        Ok(img) => img,
        Err(_) => {
            println!("Error: Could not open image.");
            exit(1);
        }
    };

    // println!("{:?}", img.dimensions());
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pix = img.get_pixel(x, y);
                let mut intesity = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
                if pix[3] == 0 {
                    intesity = 0;
                }

                print!("{}", get_str_ascii(intesity));
            }
        }

        if y % (scale * 2) == 0 {
            println!("");
        }
    }
}

fn main() {
    let mut scale = 4;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args.len() > 2 {
            let scale_res = args[2].parse();

            scale = match scale_res {
                Ok(scale) => scale,
                Err(_) => {
                    println!("Provided scale value is not a number.");
                    exit(1);
                }
            };
        }

        get_image(&args[1], scale);
    } else {
        println!("Missing argument: Please provide a path to an image.");
        exit(1);
    }
}

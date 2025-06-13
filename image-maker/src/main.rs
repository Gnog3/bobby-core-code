use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
};

use image::{ImageReader, Rgb};

#[derive(Debug)]
struct Operation {
    matrix: u64,
    pos_x: u16,
    pos_y: u16,
    color: u32,
}

fn main() {
    let img = ImageReader::open("image.png").unwrap().decode().unwrap();
    let img = img.to_rgb8();
    let width = img.width();
    let height = img.height();
    let mut encoded = vec![];
    for y_start in (0..height).step_by(8) {
        for x_start in (0..width).step_by(8) {
            let mut set = HashSet::new();

            for y in y_start..y_start + 8 {
                for x in x_start..x_start + 8 {
                    let Some(&color) = img.get_pixel_checked(x, y) else {
                        continue;
                    };
                    if color.0 == [255, 255, 255] {
                        continue;
                    }
                    set.insert(color);
                }
            }

            for set_color in set {
                let mut matrix = 0u64;
                let mut i = 0;
                for y in y_start..y_start + 8 {
                    for x in x_start..x_start + 8 {
                        let Some(&color) = img.get_pixel_checked(x, y) else {
                            i += 1;
                            continue;
                        };
                        if color == set_color {
                            matrix |= 1 << i;
                        }

                        i += 1;
                    }
                }
                let color = (set_color.0[0] as u32) << 16
                    | (set_color.0[1] as u32) << 8
                    | set_color.0[2] as u32;
                encoded.push(Operation {
                    matrix,
                    pos_x: x_start as u16,
                    pos_y: y_start as u16,
                    color,
                });
            }
        }
    }
    println!("encoded len: {}", encoded.len());
    let mut vec = vec![];
    for e in encoded {
        vec.extend_from_slice(&e.matrix.to_le_bytes());
        vec.extend_from_slice(&e.pos_x.to_le_bytes());
        vec.extend_from_slice(&e.pos_y.to_le_bytes());
        vec.extend_from_slice(&e.color.to_le_bytes());
    }
    fs::write("output.bin", vec).unwrap();
}

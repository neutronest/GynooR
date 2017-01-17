extern crate image;
extern crate gynoo_r;
use image::{GenericImage, ImageBuffer, Luma, Rgba};
use std::fs::File;
use std::path::Path;
use std::vec::Vec;
use std::mem;
use std::cmp;

use gynoo_r::util;
use gynoo_r::geometry::{GVector, GSphere, GRay, GCamera};

fn main() {
    println!("hello world!");
    let w = 256;
    let h = 256;
    let max_depth = 20.0;

    let mut v = vec![0 as u8; (w*h*4)];
    let sphere = GSphere{center: GVector{x: 0.0, y: 10.0, z: -10.0},
                         radius: 10.0};
    let camera = GCamera::new(GVector{x: 0.0, y: 10.0, z: 10.0},
                              GVector{x: 0.0, y: 0.0, z: -1.0},
                              GVector{x: 0.0, y: 1.0, z: 0.0},
                              90.0);
    let mut idx = 0;
    for y in 0..h {

        let sy = 1.0 - (y as i32 as f64) / (h as f64);
        for x in 0..w {

            let sx = (x as i32 as f64) / (w as f64);
            let ray = camera.generate_ray(sx, sy);
            let res = sphere.get_intersect(ray);
            // render pixels
            if res.flag {

                //println!("{}, {}", sx, sy);
                //println!("res: {}", res.distance);

                let depth = (255 - cmp::min((res.distance / max_depth * 255.0) as i32, 255) ) as u8;
                v[idx] = depth;
                v[idx+1] = depth;
                v[idx+2] = depth;
                v[idx+3] = 255 as u8;
            } else {
                v[idx] = 0 as u8;
                v[idx+1] = 0 as u8;
                v[idx+2] = 0 as u8;
                v[idx+3] = 255 as u8;
            }
            idx += 4;
        }
    }




    /*

    let mut v = vec![0 as u8; (w*h*4)];
    let mut idx = 0;
    for i_idx in 0..w {
        for j_idx in 0..h {
            let i = i_idx.clone();
            let j = j_idx.clone();

            v[idx] = ( (i as i32 as f32) / (w as f32) * 255.0) as u8;
            idx += 1;
            v[idx] = ( (j as i32 as f32) / (h as f32) * 255.0) as u8;
            idx += 1;
            v[idx] = 255 as u8;
            idx += 1;
            v[idx] = 128 as u8;
            idx += 1;
        }
    }

    
    idx = 0;
    for i_idx in 0..w {
        for j_idx in 0..h {
            println!("{}, {}, {}, {}", v[idx],
                     v[idx+1], v[idx+2], v[idx+3]);
            idx += 4;
        }
    }
    */

    let img : ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(w as u32, h as u32,  v).unwrap();
    //let img = ImageBuffer::new(512, 512);

    /*
    let img = ImageBuffer::from_fn(10, 10, |x, y| {

        let idx = (x+y) % 4;
        println!("x, y: {}, {}",x, y);
        match idx % 4 {
            0 => image::Luma([(x as f64 / 1024.0 * 255.0) as u8]),
            1 => image::Luma([(y as f64 / 1024.0 * 255.0) as u8]),
            2 => image::Luma([0]),
            3 => image::Luma([255]),
            _ => image::Luma([(x as f64 / 1024.0 * 255.0) as u8])
        }
    });
    */
       //let ref mut fout = File::create(&Path::new("test.png")).unwrap();
    let _ = img.save(&Path::new("test.png")).unwrap();
}

use std::{env, fs::{File,read_to_string}, io::{BufWriter, Write}};
use std::vec::Vec;
use nalgebra::Vector3;

mod renderable;
mod material;
mod rays;
mod scene;
mod utils;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub scene_grammar);

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("No environment file given!");
    }

    let path = &args[1];

    let file_contents = read_to_string(path)
        .expect("Could not read file");

    let output_path = path.to_owned() + ".ppm";
    let mut file = match File::create(&output_path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    let mut writer = BufWriter::new(file);

    let sc = scene_grammar::SceneParser::new().parse(&file_contents).unwrap();
    let mut image: Vec<Vec<Vector3<f64>>> = sc.render(1000, 1000);
    writer.write(b"P6\n1000 1000\n255\n");
    image.reverse();
    for row in image {
        let mut row_vec: Vec<u8> = Vec::new();
        for col in row {
            row_vec.push((col.x * 255.0) as u8);
            row_vec.push((col.y * 255.0) as u8);
            row_vec.push((col.z * 255.0) as u8);
        }
        writer.write_all(&row_vec);
    }

    println!("Hello, world!");
}

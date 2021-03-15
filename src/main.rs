use std::{env, fs};
use std::vec::Vec;

mod renderable;
mod material;
mod rays;
mod scene;
mod utils;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub scene_grammar);


// fn parse_line(line: &str) -> renderable::Object {
//
// }
//
// fn parse_file(filepath: &str) -> Vec<renderable::Object> {
//     let file = File::open(filepath).unwrap();
//     let reader = BufReader::new(file);
//
//     let mut objects: Vec<renderable::Object> = Vec::new();
//
// }

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("No environment file given!");
    }

    let path = &args[1];

    let file_contents = fs::read_to_string(path)
        .expect("Could not read file");
    //
    // let mut objects: Vec<renderable::Object> = Vec::new();

    let sc = scene_grammar::SceneParser::new().parse(&file_contents).unwrap();
    println!("Hello, world!");
}

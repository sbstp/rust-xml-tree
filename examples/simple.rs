#![feature(old_io, old_path)]

extern crate xml_tree;

use std::old_io::{BufferedReader, File};

use xml_tree::{build, EventReader};

fn main() {
    let file = File::open(&Path::new("data/ex1.xml")).unwrap();
    let reader = BufferedReader::new(file);
    let mut parser = EventReader::new(reader);
    let res = build(&mut parser);
    match res {
        Err(ref err) => println!("{}", err),
        Ok(ref doc) => {
            println!("{}", doc.root.borrow().name);
        }
    }
}

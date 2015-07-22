extern crate xml_tree;

use std::fs::File;
use xml_tree::{build, EventReader};

fn main() {
    let file = File::open("data/ex1.xml").unwrap();
    let mut parser = EventReader::new(file);
    let res = build(&mut parser);
    match res {
        Err(ref err) => println!("{}", err),
        Ok(ref doc) => {
            println!("{}", doc);
        }
    }
}

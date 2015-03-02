extern crate xml_tree;

use std::old_io::{BufferedReader, File};

use xml_tree::{build, BuildError, Document, Element, EventReader, Node, ParserError};

fn main() {
    let file = File::open(&Path::new("data/ex1.xml")).unwrap();
    let reader = BufferedReader::new(file);
    let mut parser = EventReader::new(reader);
    let res = build(&mut parser);
    match res {
        Err(ref err) => println!("error"),
        Ok(ref doc) => {
            println!("{:?}", doc.root);
        }
    }
}

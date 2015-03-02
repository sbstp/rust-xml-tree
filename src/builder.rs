use std::old_io::Buffer;

use dom::{Document, Element, Node};
use error::BuildError;

use xml::reader::EventReader;
use xml::reader::Events;
use xml::reader::events::XmlEvent;

pub fn build<B: Buffer>(reader: &mut EventReader<B>) -> Result<Document, BuildError> {
    let mut it = reader.events();

    // pre-process the document
    loop {
        match it.next() {
            None => return Err(BuildError::BuildError),
            Some(event) => {
                match event {
                    XmlEvent::StartElement { name, attributes, namespace } => {
                        let mut parent = Element::new(name, attributes, namespace);
                        try!(build_rec(&mut it, &mut parent));
                        //println!("{:?}", Node::Element(parent));
                        return Ok(Document{root: parent});
                    }
                    _ => continue,
                }
            }
        }
    }
}

fn build_rec<B: Buffer>(it: &mut Events<B>, parent: &mut Element) -> Result<(), BuildError> {
    loop {
        match it.next() {
            None => return Err(BuildError::BuildError),
            Some(event) => {
                match event {
                    // START OF CHILD
                    XmlEvent::StartElement { name, attributes, namespace } => {
                        //println!("START {}", name.borrow().local_name);
                        let mut elem = Element::new(name, attributes, namespace);
                        try!(build_rec(it, &mut elem));
                        parent.add_child(Node::Element(elem));
                    }
                    // END OF PARENT
                    XmlEvent::EndElement { name: _ } => {
                        //println!("END {}", name.borrow().local_name);
                        return Ok(());
                    }
                    XmlEvent::Characters(text) | XmlEvent::CData(text) => {
                        //println!("TEXT {}", text);
                        parent.add_child(Node::Text(text));
                    }
                    XmlEvent::Error(err) => return Err(BuildError::ParserError(err)),
                    _ => {}
                }
            }
        }
    }
}

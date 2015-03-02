use std::old_io::Buffer;

use dom::{Document, Element, Node};
use error::BuildError;

use xml::reader::EventReader;
use xml::reader::Events;
use xml::reader::events::XmlEvent;

/// Produce a Document from an EventReader.
pub fn build<B: Buffer>(reader: &mut EventReader<B>) -> Result<Document, BuildError> {
    let mut it = reader.events();

    // pre-process the document
    loop {
        match it.next() {
            None => return Err(BuildError::BuildError),
            Some(event) => {
                match event {
                    // Found the root element.
                    XmlEvent::StartElement { name, attributes, namespace } => {
                        let mut parent = Element::new(name, attributes, namespace);
                        try!(build_rec(&mut it, &mut parent));
                        return Ok(Document{root: parent});
                    }
                    // Ignore other events.
                    _ => continue,
                }
            }
        }
    }
}

// Build the tree in a recursive manner.
fn build_rec<B: Buffer>(it: &mut Events<B>, parent: &mut Element) -> Result<(), BuildError> {
    loop {
        match it.next() {
            None => return Err(BuildError::BuildError),
            Some(event) => {
                match event {
                    // Start of new child.
                    XmlEvent::StartElement { name, attributes, namespace } => {
                        let mut elem = Element::new(name, attributes, namespace);
                        try!(build_rec(it, &mut elem));
                        parent.add_child(Node::Element(elem));
                    }
                    // End of parent.
                    XmlEvent::EndElement { name: _ } => {
                        return Ok(());
                    }
                    // Text nodes.
                    XmlEvent::Characters(text) | XmlEvent::CData(text) => {
                        parent.add_child(Node::Text(text));
                    }
                    //
                    XmlEvent::Error(err) => return Err(BuildError::ParserError(err)),
                    // Ignore other events.
                    _ => {}
                }
            }
        }
    }
}

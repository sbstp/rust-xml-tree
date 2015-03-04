use std::old_io::Buffer;

use dom::{Document, Element, Text, Node, RcNode_new, RcElement, RcElement_new};
use error::BuildError;

use xml::common::XmlVersion;
use xml::reader::EventReader;
use xml::reader::events::XmlEvent;

pub fn build<B: Buffer>(reader: &mut EventReader<B>) -> Result<Document, BuildError> {
    let mut root: Option<RcElement> = None;
    let mut curr: Option<RcElement> = None;
    let mut doc_version: Option<XmlVersion> = None;
    let mut doc_encoding: Option<String> = None;

    for event in reader.events() {
        match event {
            // StartDocument
            XmlEvent::StartDocument { version, encoding, standalone: _ } => {
                doc_version = Some(version);
                doc_encoding = Some(encoding);
            }
            // EndDocument
            XmlEvent::EndDocument => {
                // TODO
            }
            // StartElement
            XmlEvent::StartElement { name, attributes, namespace } => {
                // Check if a root exists.
                match root {
                    // If it does not, it must be created.
                    // Also sets the current element to the root.
                    None => {
                        let elem = Element::new_root(name, attributes, namespace);
                        let rcelem = RcElement_new(elem);
                        root = Some(rcelem.clone());
                        curr = Some(rcelem.clone());
                    }
                    // If it does, the new element will be appended,
                    // to the current element.
                    Some (_) => {
                        match curr {
                            // this should never happen
                            None => return Err(BuildError::BuildError),
                            Some(parent) => {
                                // create the element with the parent as a weak reference
                                let elem = Element::new(parent.clone().downgrade(), name, attributes, namespace);
                                let rcelem = RcElement_new(elem);
                                // create a node from the element
                                let node = Node::Element(rcelem.clone());
                                let rcnode = RcNode_new(node);
                                // add the element to the parent
                                parent.borrow_mut().add_child(rcnode);
                                // move into the new element
                                curr = Some(rcelem.clone());
                            }
                        }
                    }
                }
            }
            // EndElement
            XmlEvent::EndElement { name: _ } => {
                match curr {
                    // this should never happen
                    None => return Err(BuildError::BuildError),
                    Some(elem) => {
                        // move out of the element by setting the current
                        // element to the parent of the element we're
                        // exiting
                        match elem.borrow().parent {
                            // current has no parent, so it's probably root
                            None => {
                                // check if root is valid
                                match root {
                                    None => return Err(BuildError::BuildError),
                                    Some(ref tmp_root) => {
                                        curr = Some(tmp_root.clone());
                                    }
                                }
                            }
                            Some(ref parent) => {
                                // TODO unwrap
                                curr = Some(parent.clone().upgrade().unwrap());
                            }
                        }
                    }
                }
            },
            // Cdata or Characters
            XmlEvent::CData(content) | XmlEvent::Characters(content) => {
                match curr {
                    // this should never happen
                    None => return Err(BuildError::BuildError),
                    Some(ref parent) => {
                        // create the text node with the parent as a weak reference
                        let text = Text::new(parent.clone().downgrade(), content);
                        let node = Node::Text(text);
                        let rcnode = RcNode_new(node);
                        // add the text node to the parent
                        parent.borrow_mut().add_child(rcnode.clone());
                    }
                }
            },
            // Error
            XmlEvent::Error(err) => {
                return Err(BuildError::ParserError(err));
            }
            // Ignore other events.
            _ => {}
        }
    }

    Ok(Document {
        version: doc_version,
        encoding: doc_encoding,
        root: root.unwrap().clone(),
    })
}


// /// Produce a Document from an EventReader.
// pub fn build<B: Buffer>(reader: &mut EventReader<B>) -> Result<Document, BuildError> {
//     let mut it = reader.events();
//     // document metadata
//     let mut doc_version: Option<XmlVersion> = None;
//     let mut doc_encoding: Option<String> = None;
//
//     // pre-process the document
//     loop {
//         match it.next() {
//             None => return Err(BuildError::BuildError),
//             Some(event) => {
//                 match event {
//                     // Document metadata.
//                     XmlEvent::StartDocument { version, encoding, standalone: _ } => {
//                         doc_version = Some(version);
//                         doc_encoding = Some(encoding);
//                     }
//                     // Found the root element.
//                     XmlEvent::StartElement { name, attributes, namespace } => {
//                         let mut parent = Element::new(name, attributes, namespace);
//                         try!(build_rec(&mut it, &mut parent));
//                         return Ok(Document{
//                             version: doc_version,
//                             encoding: doc_encoding,
//                             root: parent,
//                         });
//                     }
//                     XmlEvent::Error(err) => return Err(BuildError::ParserError(err)),
//                     // Ignore other events.
//                     _ => continue,
//                 }
//             }
//         }
//     }
// }
//
// // Build the tree in a recursive manner.
// fn build_rec<B: Buffer>(it: &mut Events<B>, parent: &mut Element) -> Result<(), BuildError> {
//     loop {
//         match it.next() {
//             None => return Err(BuildError::BuildError),
//             Some(event) => {
//                 match event {
//                     // Start of new child.
//                     XmlEvent::StartElement { name, attributes, namespace } => {
//                         let mut elem = Element::new(name, attributes, namespace);
//                         try!(build_rec(it, &mut elem));
//                         parent.add_child(Node::Element(elem));
//                     }
//                     // End of parent.
//                     XmlEvent::EndElement { name: _ } => {
//                         return Ok(());
//                     }
//                     // Text nodes.
//                     XmlEvent::Characters(text) | XmlEvent::CData(text) => {
//                         parent.add_child(Node::Text(text));
//                     }
//                     //
//                     XmlEvent::Error(err) => return Err(BuildError::ParserError(err)),
//                     // Ignore other events.
//                     _ => {}
//                 }
//             }
//         }
//     }
// }

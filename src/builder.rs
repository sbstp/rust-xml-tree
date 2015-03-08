use std::old_io::Buffer;

use dom::{Document, Element, Text, Node, rc_node_new, RcElement, rc_element_new, rc_text_new};
use error::BuildError;

use xml::common::XmlVersion;
use xml::reader::EventReader;
use xml::reader::events::XmlEvent;

/// `build` creates a `Document` from an `EventReader`.
///
/// `build` iterates through the events yielded by the provided `EventReader`.
/// This allows you to configure the `EvenReader` as you desire. It will create
/// a reference counted DOM that you can then manipulate. The builder will return
/// a `BuildError` if the source document is invalid of if it is empty.
///
/// The builder should not panic. It should only panic in case of an impossible
/// scenario. Please report any panics.
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
            // StartElement
            XmlEvent::StartElement { name, attributes, namespace } => {
                // Check if a root exists.
                match root {
                    // If it does not, it must be created.
                    // Also sets the current element to the root.
                    None => {
                        let elem = Element::new_root(name, attributes, namespace);
                        let rcelem = rc_element_new(elem);
                        root = Some(rcelem.clone());
                        curr = Some(rcelem.clone());
                    }
                    // If it does, the new element will be appended,
                    // to the current element.
                    Some (_) => {
                        match curr {
                            // This should never happen.
                            None => panic!("Root is set but current is not."),
                            Some(parent) => {
                                // create the element with the parent as a weak reference
                                let elem = Element::new(parent.clone().downgrade(), name, attributes, namespace);
                                let rcelem = rc_element_new(elem);
                                // create a node from the element
                                let node = Node::Element(rcelem.clone());
                                let rcnode = rc_node_new(node);
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
                    // This should never happen.
                    None => panic!("End element before start element."),
                    Some(elem) => {
                        // move out of the element by setting the current
                        // element to the parent of the element we're
                        // exiting
                        match elem.borrow().parent {
                            // current has no parent, so it's probably root
                            None => {
                                // check if root is valid
                                match root {
                                    // This should never happen.
                                    None => panic!("End element without any root."),
                                    Some(ref root) => {
                                        curr = Some(root.clone());
                                    }
                                }
                            }
                            Some(ref parent) => {
                                curr = Some(parent.clone().upgrade()
                                    .expect("Unable to upgrade WeakReference, parent was dropped."));
                            }
                        }
                    }
                }
            },
            // Cdata or Characters
            XmlEvent::CData(content) | XmlEvent::Characters(content) => {
                match curr {
                    // This should never happen.
                    None => panic!("Text node before any element."),
                    Some(ref parent) => {
                        // create the text node with the parent as a weak reference
                        let text = Text::new(parent.clone().downgrade(), content);
                        let rctext = rc_text_new(text);
                        // create wrapper node
                        let node = Node::Text(rctext);
                        let rcnode = rc_node_new(node);
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

    match root {
        None => Err(BuildError::UndefinedRoot),
        Some (root) => {
            Ok(Document {
                version: doc_version,
                encoding: doc_encoding,
                root: root.clone(),
            })
        }
    }
}

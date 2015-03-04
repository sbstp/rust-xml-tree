use std::cell::RefCell;
use std::rc::{Rc, Weak};

use xml::attribute::OwnedAttribute;
use xml::common::XmlVersion;
use xml::name::OwnedName;
use xml::namespace::Namespace;

/// Describes an XML Document.
pub struct Document {
    // document version
    pub version: Option<XmlVersion>,
    // document encoding
    pub encoding: Option<String>,
    // root element
    pub root: RcElement,
}

/// Describes a node with shared ownership.
pub type RcNode = Rc<RefCell<Node>>;

/// Handy consructor for RcNodes.
#[allow(non_snake_case)]
pub fn RcNode_new(node: Node) -> RcNode {
    Rc::new(RefCell::new(node))
}

/// Describes an element with shared ownership.
pub type RcElement = Rc<RefCell<Element>>;

/// Handy constructor for RcElements.
#[allow(non_snake_case)]
pub fn RcElement_new(element: Element) -> RcElement {
    Rc::new(RefCell::new(element))
}

/// Describes a weak reference to an element.
pub type WeakElement = Weak<RefCell<Element>>;

/// Describes an element of the DOM tree.
pub struct Element {
    /// parent element
    pub parent: Option<WeakElement>,
    /// element name
    pub name: OwnedName,
    /// element attributes
    pub attributes: Vec<OwnedAttribute>,
    /// element namespace
    pub namespace: Namespace,
    /// children nodes
    children: Vec<RcNode>,
}

impl Element {

    pub fn new(parent: WeakElement, name: OwnedName, attributes: Vec<OwnedAttribute>, namespace: Namespace) -> Element {
        Element {
            parent: Some(parent),
            name: name,
            attributes: attributes,
            namespace: namespace,
            children: Vec::new(),
        }
    }

    pub fn new_root(name: OwnedName, attributes: Vec<OwnedAttribute>, namespace: Namespace) -> Element {
        Element {
            parent: None,
            name: name,
            attributes: attributes,
            namespace: namespace,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, node: RcNode) {
        self.children.push(node);
    }

}

/// Describes a text node of the DOM tree.
pub struct Text {
    /// parent element
    pub parent: WeakElement,
    /// the text node's content
    pub content: String,
}

impl Text {

    pub fn new(parent: WeakElement, content: String) -> Text {
        Text {
            parent: parent,
            content: content,
        }
    }

}

/// Describes a node of the XML tree.
/// The node can be an element or a text node.
pub enum Node {
    Element(RcElement),
    Text(Text),
}

#[cfg(test)]
mod tests {

    // use std::old_io::{Buffer, MemReader};
    //
    // use {build, Document, Element, ElementIterator};
    //
    // use xml::EventReader;
    // use xml::common::XmlVersion;
    //
    // fn xml_to_doc(text: &str) -> Document {
    //     let mut reader = EventReader::new(MemReader::new(text.as_bytes().to_vec()));
    //     let res = build(&mut reader);
    //     match res {
    //         Ok(doc) => doc,
    //         Err(err) => panic!("Error: {}", err),
    //     }
    // }
    //
    // #[test]
    // fn test_find() {
    //     let xml = "<root><item>aa</item><item>bb</item><item>cc</item><notitem></notitem></root>";
    //     let doc = xml_to_doc(xml);
    //
    //     let elems = doc.root.find("item");
    //     assert_eq!(elems.len(), 3);
    // }
    //
    // #[test]
    // fn test_text_simple() {
    //     let xml = "<root>abc</root>";
    //     let doc = xml_to_doc(xml);
    //
    //     assert_eq!(doc.root.text(), "abc");
    // }
    //
    // #[test]
    // fn test_test_complex() {
    //     let xml = "<root>abc<sep></sep>def</root>";
    //     let doc = xml_to_doc(xml);
    //
    //     assert_eq!(doc.root.text(), "abcdef");
    // }
    //
    // #[test]
    // fn test_iter_elements() {
    //     let xml = "<root>abc<sep></sep>def<oy></oy></root>";
    //     let doc = xml_to_doc(xml);
    //
    //     let it: ElementIterator = doc.root.iter_elements();
    //     let v: Vec<&Element> = it.collect();
    //
    //     assert_eq!(doc.root.len(), 4);
    //     assert_eq!(v.len(), 2);
    // }
    //
    // #[test]
    // fn test_iter_text() {
    //     let xml = "<root>abc<sep></sep>def<oy></oy></root>";
    //     let doc = xml_to_doc(xml);
    //
    //     let it = doc.root.iter_text();
    //     let v: Vec<&str> = it.collect();
    //
    //     assert_eq!(doc.root.len(), 4);
    //     assert_eq!(v.len(), 2);
    // }
    //
    // #[test]
    // fn test_version_encoding() {
    //     let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><root><item></item></root>";
    //     let doc = xml_to_doc(xml);
    //
    //     assert!(doc.version == Some(XmlVersion::Version10));
    //     assert_eq!(doc.encoding, Some("UTF-8".to_string()));
    // }

}

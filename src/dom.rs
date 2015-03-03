use std::fmt;
use std::iter::Iterator;
use std::slice::Iter;

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
    pub root: Element,
}

/// Describes an element of the tree.
pub struct Element {
    pub name: OwnedName,
    pub attributes: Vec<OwnedAttribute>,
    pub namespace: Namespace,
    children: Vec<Box<Node>>,
}

impl Element {

    /// Create a new Element from the XmlEvent::StartElement's data.
    pub fn new(name: OwnedName, attributes: Vec<OwnedAttribute>, namespace: Namespace) -> Element {
        Element {
            name: name,
            attributes: attributes,
            namespace: namespace,
            children: Vec::new(),
        }
    }

    /// Add a child to this element's children list.
    pub fn add_child(&mut self, child: Node) {
        self.children.push(Box::new(child));
    }

    /// Return the number of child nodes.
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Find children by name.
    pub fn find<'a>(&'a self, name: &str) -> Vec<&'a Element> {
        self.iter_elements().filter(|&elem| elem.name.borrow().local_name == name).collect()
    }

    /// Get the text nodes of this Element concatenated.
    pub fn text(&self) -> String {
        let mut buf = String::new();
        for text in self.iter_text() {
            buf.push_str(text);
        }
        buf
    }

    /// Create an iterator that only yields Node::Element node types.
    pub fn iter_elements(&self) -> ElementIterator {
        ElementIterator { source: Box::new(self.children.iter()) }
    }

    // Create an iterator that only yields Node::Text node types.
    pub fn iter_text(&self) -> TextIterator {
        TextIterator { source: Box::new(self.children.iter()) }
    }

    /// Print this element in a pretty way.
    fn pretty_indent(&self, f: &mut fmt::Formatter, indent: &str) -> fmt::Result {
        let name = self.name.borrow().local_name;
        let next_indent = String::from_str(indent) + "  ";

        try!(write!(f, "{}<{}>\n", indent, name));
        for child in self.children.iter() {
            try!(child.pretty_indent(f, next_indent.as_slice()));
        }
        try!(write!(f, "{}</{}>\n", indent, name));

        Ok(())
    }

}

impl fmt::Debug for Element {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_indent(f, "")
    }

}

impl fmt::Display for Element {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name.borrow().local_name;

        try!(write!(f, "<{}>", name));
        for child in self.children.iter() {
            try!(child.fmt(f));
        }
        try!(write!(f, "</{}>", name));

        Ok(())
    }

}

/// Iterator for element nodes.
pub struct ElementIterator<'a> {
    source: Box<Iter<'a, Box<Node>>>,
}

impl<'a> Iterator for ElementIterator<'a> {

    type Item = &'a Element;

    fn next(&mut self) -> Option<&'a Element> {
        loop {
            let it = self.source.next();
            match it {
                None => return None,
                Some(node) => {
                    match **node {
                        Node::Text(_) => continue,
                        Node::Element(ref elem) => return Some(elem),
                    }
                }
            }
        }
    }

}

/// Iterator for text nodes.
pub struct TextIterator<'a> {
    source: Box<Iter<'a, Box<Node>>>,
}

impl<'a> Iterator for TextIterator<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        loop {
            let it = self.source.next();
            match it {
                None => return None,
                Some(node) => {
                    match **node {
                        Node::Element(_) => continue,
                        Node::Text(ref text) => return Some(text.as_slice()),
                    }
                }
            }
        }
    }

}


/// Node of the tree.
/// A node can be a Text node or an Element node.
/// Text nodes do not have children.
pub enum Node {
    Text(String),
    Element(Element),
}

impl Node {

    fn pretty_indent(&self, f: &mut fmt::Formatter, indent: &str) -> fmt::Result {
        match *self {
            Node::Text(ref text) => write!(f, "{}{}\n", indent, text),
            Node::Element(ref elem) => elem.pretty_indent(f, indent),
        }
    }

}

impl fmt::Debug for Node {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_indent(f, "")
    }

}

impl fmt::Display for Node {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Text(ref text) => write!(f, "{}", text),
            Node::Element(ref elem) => elem.fmt(f),
        }
    }

}

#[cfg(test)]
mod tests {

    use std::old_io::{Buffer, MemReader};

    use {build, Document, Element, ElementIterator};

    use xml::EventReader;
    use xml::common::XmlVersion;

    fn xml_to_doc(text: &str) -> Document {
        let mut reader = EventReader::new(MemReader::new(text.as_bytes().to_vec()));
        let res = build(&mut reader);
        match res {
            Ok(doc) => doc,
            Err(err) => panic!("Error: {}", err),
        }
    }

    #[test]
    fn test_find() {
        let xml = "<root><item>aa</item><item>bb</item><item>cc</item><notitem></notitem></root>";
        let doc = xml_to_doc(xml);

        let elems = doc.root.find("item");
        assert_eq!(elems.len(), 3);
    }

    #[test]
    fn test_text_simple() {
        let xml = "<root>abc</root>";
        let doc = xml_to_doc(xml);

        assert_eq!(doc.root.text(), "abc");
    }

    #[test]
    fn test_test_complex() {
        let xml = "<root>abc<sep></sep>def</root>";
        let doc = xml_to_doc(xml);

        assert_eq!(doc.root.text(), "abcdef");
    }

    #[test]
    fn test_iter_elements() {
        let xml = "<root>abc<sep></sep>def<oy></oy></root>";
        let doc = xml_to_doc(xml);

        let it: ElementIterator = doc.root.iter_elements();
        let v: Vec<&Element> = it.collect();

        assert_eq!(doc.root.len(), 4);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_iter_text() {
        let xml = "<root>abc<sep></sep>def<oy></oy></root>";
        let doc = xml_to_doc(xml);

        let it = doc.root.iter_text();
        let v: Vec<&str> = it.collect();

        assert_eq!(doc.root.len(), 4);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_version_encoding() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><root><item></item></root>";
        let doc = xml_to_doc(xml);

        assert!(doc.version == Some(XmlVersion::Version10));
        assert_eq!(doc.encoding, Some("UTF-8".to_string()));
    }

}

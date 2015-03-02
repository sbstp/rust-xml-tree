use std::fmt;

use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;

/// Describes an XML Document.
pub struct Document {
    pub root: Element,
}

/// Describes an element of the tree.
pub struct Element {
    pub name: OwnedName,
    pub attributes: Vec<OwnedAttribute>,
    pub namespace: Namespace,
    pub children: Vec<Box<Node>>,
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

    /// Find children by name.
    pub fn find<'a>(&'a self, name: &str) -> Vec<&'a Element> {
        let mut matches: Vec<&'a Element> = Vec::new();
        for child in self.children.iter() {
            match **child {
                Node::Element(ref elem) => {
                    if elem.name.borrow().local_name == name {
                        matches.push(elem);
                    }
                }
                _ => continue,
            }
        }
        matches
    }

    /// Get the text nodes of this Element concatenated.
    pub fn text(&self) -> String {
        let mut buff = String::new();
        for child in self.children.iter() {
            match **child {
                Node::Text(ref text) => {
                    buff.push_str(text);
                }
                _ => continue,
            }
        }
        buff
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

    use {build, Document};

    use xml::EventReader;

    fn xml_to_doc(text: &str) -> Document {
        let mut reader = EventReader::new(MemReader::new(text.as_bytes().to_vec()));
        build(&mut reader).ok().unwrap()
    }

    #[test]
    fn test_find() {
        let xml = "<root><item>aa</item><item>bb</item><item>cc</item></root>";
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

}

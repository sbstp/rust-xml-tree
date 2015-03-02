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

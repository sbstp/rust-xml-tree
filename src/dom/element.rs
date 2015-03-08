use std::cell::RefCell;
use std::fmt;
use std::iter::Iterator;
use std::rc::{Rc, Weak};
use std::slice::Iter;

use dom::{self, Node, RcNode, TextIterator};

use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;

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

    /// Return the number of child nodes.
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Find children by name.
    /// Currently ignores namespaces.
    // TODO namespace
    pub fn find(&self, name: &str) -> Vec<RcElement> {
        self.iter_elements().filter(|elem| {
            elem.borrow().name.borrow().local_name == name
        }).collect()
    }

    /// Get the text nodes of this Element concatenated.
    pub fn text(&self) -> String {
        let mut buf = String::new();
        for text in self.iter_text() {
            buf.push_str(text.borrow().content.as_slice());
        }
        buf
    }

    /// Create an iterator that yield all children nodes.
    pub fn iter<'a>(&'a self) -> Iter<'a, RcNode> {
        self.children.iter()
    }

    /// Create an iterator that only yields Node::Element node types.
    pub fn iter_elements(&self) -> ElementIterator {
        ElementIterator { source: Box::new(self.iter()) }
    }

    /// Create an iterator that only yields Node::Text node types.
    pub fn iter_text(&self) -> TextIterator {
        TextIterator::new(Box::new(self.iter()))
    }

    /// Format the Element in a pretty way.
    pub fn format_pretty<W: fmt::Write>(&self, w: &mut W, indent: usize, inc: usize) -> fmt::Result {
        let name = self.name.borrow().local_name; // TODO namespace
        let padding = dom::util::padding(indent, inc);

        try!(write!(w, "{}<{}>\n", padding, name));
        for child in self.iter() {
            try!(child.borrow().format_pretty(w, indent + 1, inc))
        }
        try!(write!(w, "{}</{}>\n", padding, name));

        Ok(())
    }

}

impl fmt::Debug for Element {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        try!(self.format_pretty(&mut buf, 0, 2));
        f.write_str(buf.as_slice())
    }

}

impl fmt::Display for Element {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name.borrow().local_name;
        try!(write!(f, "<{}>", name));
        for child in self.iter() {
            try!(child.borrow().fmt(f));
        }
        try!(write!(f, "</{}>", name));
        Ok(())
    }

}

/// Iterator for element nodes.
pub struct ElementIterator<'a> {
    source: Box<Iter<'a, RcNode>>,
}

impl<'a> Iterator for ElementIterator<'a> {

    type Item = RcElement;

    fn next(&mut self) -> Option<RcElement> {
        loop {
            let it = self.source.next();
            match it {
                None => return None,
                Some(node) => {
                    match *node.borrow() {
                        Node::Text(_) => continue,
                        Node::Element(ref elem) => return Some(elem.clone()),
                    }
                }
            }
        }
    }

}

/// Describes an element with shared ownership.
pub type RcElement = Rc<RefCell<Element>>;

/// Handy constructor for RcElements.
pub fn rc_element_new(element: Element) -> RcElement {
    Rc::new(RefCell::new(element))
}

/// Describes a weak reference to an element.
pub type WeakElement = Weak<RefCell<Element>>;

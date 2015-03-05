use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use dom::element::RcElement;
use dom::text::Text;

/// Describes a node of the XML tree.
/// The node can be an element or a text node.
pub enum Node {
    Element(RcElement),
    Text(Text),
}

impl Node {

    /// Format the Element in a pretty way.
    pub fn format_pretty<W: fmt::Write>(&self, w: &mut W, indent: usize, inc: usize) -> fmt::Result {
        match *self {
            Node::Element(ref elem) => elem.borrow().format_pretty(w, indent, inc),
            Node::Text(ref text) => text.format_pretty(w, indent, inc),
        }
    }

}

impl fmt::Debug for Node {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Element(ref elem) => elem.borrow().fmt(f),
            Node::Text(ref elem) => elem.fmt(f),
        }
    }

}

impl fmt::Display for Node {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Element(ref elem) => elem.borrow().fmt(f),
            Node::Text(ref elem) => elem.fmt(f),
        }
    }

}

/// Describes a node with shared ownership.
pub type RcNode = Rc<RefCell<Node>>;

/// Handy consructor for RcNodes.
pub fn rc_node_new(node: Node) -> RcNode {
    Rc::new(RefCell::new(node))
}

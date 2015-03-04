use std::cell::RefCell;
use std::rc::Rc;

use dom::element::RcElement;
use dom::text::Text;

/// Describes a node of the XML tree.
/// The node can be an element or a text node.
pub enum Node {
    Element(RcElement),
    Text(Text),
}

/// Describes a node with shared ownership.
pub type RcNode = Rc<RefCell<Node>>;

/// Handy consructor for RcNodes.
pub fn rc_node_new(node: Node) -> RcNode {
    Rc::new(RefCell::new(node))
}

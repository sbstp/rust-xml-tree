use std::iter::Iterator;
use std::slice::Iter;

use dom::{Node, RcNode, WeakElement};

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

/// Iterator for text nodes.
pub struct TextIterator<'a> {
    source: Box<Iter<'a, RcNode>>,
}

impl<'a> TextIterator<'a> {

    pub fn new<'b>(source: Box<Iter<'b, RcNode>>) -> TextIterator<'b> {
        TextIterator {
            source: source,
        }
    }

}

impl<'a> Iterator for TextIterator<'a> {

    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            let it = self.source.next();
            match it {
                None => return None,
                Some(node) => {
                    match *node.borrow() {
                        Node::Element(_) => continue,
                        // TODO cloning the string is not ideal here.
                        Node::Text(ref text) => return Some(text.content.clone()),
                    }
                }
            }
        }
    }

}

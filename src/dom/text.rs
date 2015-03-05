use std::fmt;
use std::iter::Iterator;
use std::slice::Iter;

use dom::{self, Node, RcNode, WeakElement};

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

    /// Format the Element in a pretty way.
    pub fn format_pretty<W: fmt::Write>(&self, w: &mut W, indent: usize, inc: usize) -> fmt::Result {
        let padding = dom::util::padding(indent, inc);
        try!(write!(w, "{}{}\n", padding, self.content));
        Ok(())
    }

}

impl fmt::Debug for Text {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        try!(self.format_pretty(&mut buf, 0, 2));
        f.write_str(buf.as_slice())
    }

}

impl fmt::Display for Text {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
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

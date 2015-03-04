#![feature(alloc, core, old_io)]

extern crate xml;

pub use builder::build;
pub use dom::{
    Document,
    Element, ElementIterator, RcElement, WeakElement,
    Text, TextIterator,
    Node, RcNode,
};
pub use error::BuildError;

pub use xml::EventReader;
pub use xml::attribute::OwnedAttribute;
pub use xml::common::Error as ParserError;
pub use xml::common::XmlVersion;
pub use xml::name::OwnedName;
pub use xml::namespace::Namespace;
pub use xml::reader::config::ParserConfig;
pub use xml::reader::events::XmlEvent;

mod builder;
mod dom;
mod error;

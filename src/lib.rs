#![feature(collections, core, old_io)]

extern crate xml;

pub use builder::build;
pub use dom::{Document, Element, Node};
pub use error::BuildError;
pub use xml::EventReader;
pub use xml::common::Error as ParserError;

mod builder;
mod dom;
mod error;

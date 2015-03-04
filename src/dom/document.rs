use dom::element::RcElement;

use xml::common::XmlVersion;

/// Describes an XML Document.
pub struct Document {
    // document version
    pub version: Option<XmlVersion>,
    // document encoding
    pub encoding: Option<String>,
    // root element
    pub root: RcElement,
}

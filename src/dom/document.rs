use std::fmt;

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

impl Document {

    fn print_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.version.is_some() || self.encoding.is_some() {
            try!(write!(f, "<?xml "));
            if self.version.is_some() {
                try!(write!(f, "version=\"{}\"", self.version.as_ref().unwrap()));
            }if self.encoding.is_some() {
                try!(write!(f, "encoding=\"{}\"", self.encoding.as_ref().unwrap()));
            }
            try!(write!(f, "?>"));
        }
        Ok(())
    }

}

impl fmt::Debug for Document {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(self.print_header(f));
        try!(f.write_str("\n"));
        write!(f, "{:?}", *self.root.borrow())
    }

}

impl fmt::Display for Document {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(self.print_header(f));
        write!(f, "{}", *self.root.borrow())
    }

}

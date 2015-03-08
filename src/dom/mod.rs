pub use self::document::Document;
pub use self::element::{Element, ElementIterator, RcElement, WeakElement, rc_element_new};
pub use self::node::{Node, RcNode, rc_node_new};
pub use self::text::{Text, TextIterator, RcText, rc_text_new};

mod document;
mod element;
mod node;
mod text;
mod util;

#[cfg(test)]
mod tests {

    use std::old_io::{MemReader};

    use builder::build;
    use super::{Document, RcElement, RcText};

    use xml::EventReader;
    use xml::common::XmlVersion;

    fn xml_to_doc(text: &str) -> Document {
        let mut reader = EventReader::new(MemReader::new(text.as_bytes().to_vec()));
        let res = build(&mut reader);
        match res {
            Ok(doc) => doc,
            Err(err) => panic!("Error: {}", err),
        }
    }

    #[test]
    fn test_find() {
        let xml = "<root><item>aa</item><item>bb</item><item>cc</item><notitem></notitem></root>";
        let doc = xml_to_doc(xml);
        let root = doc.root.borrow();

        let elems = root.find("item");
        assert_eq!(elems.len(), 3);
    }

    #[test]
    fn test_text_simple() {
        let xml = "<root>abc</root>";
        let doc = xml_to_doc(xml);

        assert_eq!(doc.root.borrow().text(), "abc");
    }

    #[test]
    fn test_test_complex() {
        let xml = "<root>abc<sep></sep>def</root>";
        let doc = xml_to_doc(xml);

        assert_eq!(doc.root.borrow().text(), "abcdef");
    }

    #[test]
    fn test_iter_elements() {
        let xml = "<root>abc<sep></sep>def<oy></oy></root>";
        let doc = xml_to_doc(xml);
        let root = doc.root.borrow();

        let it = root.iter_elements();
        let v: Vec<RcElement> = it.collect();

        assert_eq!(root.len(), 4);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_iter_text() {
        let xml = "<root>abc<sep></sep>def<oy></oy></root>";
        let doc = xml_to_doc(xml);
        let root = doc.root.borrow();

        let it = root.iter_text();
        let v: Vec<RcText> = it.collect();

        assert_eq!(root.len(), 4);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_version_encoding() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><root><item></item></root>";
        let doc = xml_to_doc(xml);

        assert!(doc.version == Some(XmlVersion::Version10));
        assert_eq!(doc.encoding, Some("UTF-8".to_string()));
    }

    #[test]
    fn test_get_parent() {
        let xml = "<root><item></item><item></item></root>";
        let doc = xml_to_doc(xml);
        let root = doc.root.borrow();

        for child in root.find("item").iter() {
            assert!(child.borrow().get_parent().is_some());
        }
    }

}

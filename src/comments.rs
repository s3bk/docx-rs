//! Comments part
//!
//! The corresponding ZIP item is `/word/comments.xml`.

use std::borrow::Cow;
use hard_xml::{XmlRead};

use crate::document::Paragraph;

/// The root element of the comments document part.
#[derive(Debug, Default, XmlRead)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:comments")]
pub struct Comments<'a> {
    // Specifies the comments
    #[xml(child = "w:comment")]
    pub comments: Vec<Comment<'a>>,
}

#[derive(Debug, Default, XmlRead)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:comment")]
pub struct Comment<'a> {
    // Specifies the id of the comment.
    #[xml(attr = "w:id")]
    pub id: Cow<'a, str>,

    // Specifies the body of the comment.
    #[xml(child = "w:p")]
    pub content: Paragraph<'a>,
}

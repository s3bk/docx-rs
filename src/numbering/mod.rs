//! Numbering part
//!
//! The corresponding ZIP item is `/word/numbering.xml`.

mod level;

pub use self::{
    level::*,
};

use std::borrow::Cow;
use hard_xml::{XmlRead, XmlWrite};

/// The root element of the numbering document part.
#[derive(Debug, Default, XmlRead)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:numbering")]
pub struct Numberings<'a> {
    #[xml(child = "w:abstractNum")]
    pub abstract_nums: Vec<AbstractNumbering<'a>>,

    #[xml(child = "w:num")]
    pub nums: Vec<Numbering>,
}

#[derive(Debug, Default, XmlRead)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:abstractNum")]
pub struct AbstractNumbering<'a> {
    /// Specifies the id of the abstract numbering.
    #[xml(attr = "w:abstractNumId")]
    pub abstract_num_id: usize,

    #[xml(child = "w:lvl")]
    pub levels: Vec<Level<'a>>,

    #[xml(child = "w:numStyleLink")]
    pub num_style_link: Option<NumStyleLink<'a>>,
}

#[derive(Debug, Default, XmlRead)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:num")]
pub struct Numbering {
    /// Specifies the id of the numbering.
    #[xml(attr = "w:numId")]
    pub num_id: usize,

    /// Specifies the id of the abstract numbering.
    #[xml(child = "w:abstractNumId")]
    pub abstract_num_id: AbstractNumberingId,
}

#[derive(Clone, Copy, Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:abstractNumId")]
pub struct AbstractNumberingId {
    #[xml(attr = "w:val")]
    pub value: usize,
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:numStyleLink")]
pub struct NumStyleLink<'a> {
    #[xml(attr = "w:val")]
    pub val: Cow<'a, str>,
}

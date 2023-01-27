use hard_xml::{XmlRead, XmlWrite};

/// Tab
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tab")]
pub struct Tab;

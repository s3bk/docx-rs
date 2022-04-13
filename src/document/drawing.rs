use derive_more::From;
use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    document::{r#break::Break, tab::Tab, text::Text},
    formatting::CharacterProperty,
};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:drawing")]
pub struct Drawing {
    /// comment
    #[xml(default, child = "wp:anchor")]
    pub anchor: Anchor,
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:anchor")]
pub struct Anchor {
    #[xml(default, child = "a:graphic")]
    pub graphic: Graphic,

    #[xml(default, child = "wp:extent")]
    pub extend: Extend,
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:graphic")]
pub struct Graphic {
    #[xml(default, child = "a:graphicData")]
    pub data: GraphicData,
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:graphicData")]
pub struct GraphicData {
    #[xml(default, child = "pic:pic")]
    pub pic: Picture,
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "pic:pic")]
pub struct Picture {
    #[xml(default, child = "pic:blipFill")]
    pub fill: BlipFill,
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "pic:blipFill")]
pub struct BlipFill {
    #[xml(default, child = "a:blip")]
    pub blip: Blip,
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "a:blip")]
pub struct Blip {
    #[xml(default, attr = "r:embed")]
    pub embed: String,
}


#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "wp:extent")]
pub struct Extend {
    #[xml(default, attr = "cx")]
    pub cx: u64,

    #[xml(default, attr = "cy")]
    pub cy: u64,
}
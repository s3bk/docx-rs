use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:rFonts")]
pub struct Fonts<'a> {
    #[xml(attr = "w:ascii")]
    pub ascii: Option<Cow<'a, str>>,

    #[xml(attr = "w:hAnsi")]
    pub h_ansi: Option<Cow<'a, str>>,

    #[xml(attr = "w:cs")]
    pub cs: Option<Cow<'a, str>>,

    #[xml(attr = "w:eastAsia")]
    pub east_asia: Option<Cow<'a, str>>,
}
